use core::fmt;
use std::{collections::HashSet, fmt::Formatter};

use sdk::{
    fen::Fen,
    hash::ZOBRIST_KEYS,
    position::{CastlingKind, Color, Piece, Position},
    square::{File, Square},
};

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Clone, Copy, Hash, Eq)]
pub struct Move {
    pub inner: u16,
}

impl PartialEq for Move {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

pub trait MakeMove {
    fn make_move(&mut self, mv: &Move) -> Result<Option<Piece>>;
    fn make_null_move(&mut self);
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let from = self.from();
        let to = self.to();

        if let Some(promotion) = self.promotion() {
            write!(f, "from={}, to={}, promotion={}", from, to, promotion)
        } else {
            write!(f, "from={}, to={}", from, to)
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let promotion = if let Some(promotion) = self.promotion() {
            format!("{promotion}")
        } else {
            "".to_string()
        };
        write!(f, "{}{}{}", self.from(), self.to(), promotion)
    }
}

impl MakeMove for Position {
    fn make_move(&mut self, mv: &Move) -> Result<Option<Piece>> {
        let from = mv.from();
        let to = mv.to();
        let color = self.turn;

        let old_castling = self.castling.inner;

        for (rook_sq, kind) in [
            (Square::A1, CastlingKind::WhiteQueenside),
            (Square::H1, CastlingKind::WhiteKingside),
            (Square::A8, CastlingKind::BlackQueenside),
            (Square::H8, CastlingKind::BlackKingside),
        ]
        .iter()
        {
            if mv.from() == *rook_sq || mv.to() == *rook_sq {
                self.castling.remove_castling_kind(kind);
            }
        }

        let (from_piece, from_color) = self
            .remove_piece_at(&from)
            .expect("BUG: No piece at from square");

        if from_piece == Piece::King {
            self.castling.remove_color_castling(&color);
        }

        // Update moved piece hash
        self.hash ^= ZOBRIST_KEYS.pieces[from_color as usize][from_piece as usize][from as usize];
        self.hash ^= ZOBRIST_KEYS.pieces[from_color as usize][from_piece as usize][to as usize];

        if matches!(mv.kind(), MoveKind::Castling) {
            self.castling.remove_color_castling(&color);
        }

        // Update castling hash
        if self.castling.inner != old_castling {
            self.hash ^= ZOBRIST_KEYS.castling_rights[old_castling as usize];
            self.hash ^= ZOBRIST_KEYS.castling_rights[self.castling.inner as usize];
        }

        // Update en passant hash
        if let Some(en_passant) = self.en_passant {
            self.hash ^= ZOBRIST_KEYS.en_passant[en_passant as usize];
        }

        let captured = match mv.kind() {
            MoveKind::Quiet => {
                self.add_piece_at(to, from_piece, from_color)?;

                None
            }
            MoveKind::Capture => {
                let (target_piece, target_color) = self.remove_piece_at(&to).unwrap();

                self.add_piece_at(to, from_piece, from_color)?;

                // Update captured piece hash
                self.hash ^=
                    ZOBRIST_KEYS.pieces[target_color as usize][target_piece as usize][to as usize];

                Some((target_piece, target_color))
            }
            MoveKind::EnPassant => {
                let captured_sq = mv
                    .to()
                    .offset(if color == Color::White { -1 } else { 1 }, 0)
                    .expect("BUG: Invalid en passant square");

                let (target_piece, target_color) = self
                    .remove_piece_at(&captured_sq)
                    .expect("BUG: No piece at to square");

                self.add_piece_at(to, from_piece, from_color)?;

                // Update captured piece hash
                self.hash ^= ZOBRIST_KEYS.pieces[target_color as usize][target_piece as usize]
                    [captured_sq as usize];

                Some((target_piece, target_color))
            }
            MoveKind::Castling => {
                let castling = mv
                    .castling_kind(&self.turn)
                    .expect("BUG: Move does not castle.");

                let (rook_from, _) = castling.from_squares();
                let (rook_to, king_to) = castling.target_squares();

                let (rook, _) = self.remove_piece_at(&rook_from).unwrap_or_else(|| {
                    panic!(
                        "BUG: No piece at {rook_from} from square\n{self}, FEN: {}",
                        self.to_fen()
                    )
                });

                self.add_piece_at(king_to, from_piece, from_color)?;
                self.add_piece_at(rook_to, rook, from_color)?;

                // Update rook hash as king hash is already updated
                self.hash ^= ZOBRIST_KEYS.pieces[from_color as usize][Piece::Rook as usize]
                    [rook_from as usize];
                self.hash ^= ZOBRIST_KEYS.pieces[from_color as usize][Piece::Rook as usize]
                    [rook_to as usize];

                None
            }
            MoveKind::Promotion | MoveKind::PromotionCapture => {
                let promotion = mv.promotion().expect("BUG: No promotion piece");
                let captured = self.remove_piece_at(&to);

                self.add_piece_at(to, promotion, from_color)?;

                // Update pawn hash
                self.hash ^=
                    ZOBRIST_KEYS.pieces[from_color as usize][Piece::Pawn as usize][to as usize];
                // Update promotion hash
                self.hash ^=
                    ZOBRIST_KEYS.pieces[from_color as usize][promotion as usize][to as usize];

                // Update captured piece hash
                if let Some((target_piece, target_color)) = captured {
                    self.hash ^= ZOBRIST_KEYS.pieces[target_color as usize][target_piece as usize]
                        [to as usize];
                }

                captured
            }
            MoveKind::DoublePawnPush => {
                let captured = self.remove_piece_at(&to);

                self.add_piece_at(to, from_piece, from_color)?;

                let enpass_sq = mv
                    .to()
                    .offset(if color == Color::White { -1 } else { 1 }, 0)
                    .expect("BUG: Invalid en passant square");

                self.en_passant = Some(enpass_sq);

                // Update en_passant hash
                self.hash ^= ZOBRIST_KEYS.en_passant[enpass_sq as usize];

                captured
            }
        }
        .map(|(piece, _)| piece);

        self.occupied = self.occupation(&Color::White) | self.occupation(&Color::Black);
        if !matches!(mv.kind(), MoveKind::DoublePawnPush) {
            self.en_passant = None;
        }
        self.halfmove_clock = if captured.is_some() || from_piece == Piece::Pawn {
            0
        } else {
            self.halfmove_clock + 1
        };
        let color = self.swap_turn();

        // Update side to move Hash
        self.hash ^= ZOBRIST_KEYS.side_to_move;

        if color == Color::White {
            self.fullmove_number += 1;
        }

        Ok(captured)
    }

    fn make_null_move(&mut self) {
        let _ = self.swap_turn();
        self.hash ^= ZOBRIST_KEYS.side_to_move;
        if let Some(en_pass) = self.en_passant {
            self.hash ^= ZOBRIST_KEYS.en_passant[en_pass as usize];
            self.en_passant = None;
        }
    }
}

#[derive(Debug)]
pub enum MoveKind {
    Quiet,
    DoublePawnPush,
    Capture,
    EnPassant,
    Castling,
    Promotion,
    PromotionCapture,
}

impl Move {
    pub const fn null() -> Move {
        Move { inner: 0 }
    }

    pub fn new(from: Square, to: Square, promotion: Option<Piece>, kind: &MoveKind) -> Move {
        let mut inner = 0;
        inner |= from as u16;
        inner |= (to as u16) << 6;
        let mut mv = Move { inner };

        match kind {
            MoveKind::Capture => {
                mv.set_capture();
            }
            MoveKind::EnPassant => {
                mv.set_enpass_capture();
            }
            MoveKind::Castling => {
                if to.file() == File::C {
                    mv.set_queen_castle();
                } else if to.file() == File::G {
                    mv.set_king_castle();
                }
            }
            MoveKind::Promotion => {
                mv.set_promotion(promotion.expect("BUG: No promotion piece"));
            }
            MoveKind::PromotionCapture => {
                mv.set_promotion_capture(promotion.expect("BUG: No promotion piece"));
            }
            MoveKind::DoublePawnPush => {
                mv.set_double_pawn_push();
            }
            _ => {}
        }

        mv
    }

    pub fn from(&self) -> Square {
        Square::from_u8((self.inner & 0b0000000000111111) as u8)
    }

    pub fn to(&self) -> Square {
        Square::from_u8(((self.inner & 0b0000111111000000) >> 6) as u8)
    }

    pub fn kind(&self) -> MoveKind {
        if self.promotion().is_some() {
            if self.is_capture() {
                MoveKind::PromotionCapture
            } else {
                MoveKind::Promotion
            }
        } else if self.is_enpass_capture() {
            MoveKind::EnPassant
        } else if self.is_capture() {
            MoveKind::Capture
        } else if self.is_king_castle() || self.is_queen_castle() {
            MoveKind::Castling
        } else if self.is_double_pawn_push() {
            MoveKind::DoublePawnPush
        } else {
            MoveKind::Quiet
        }
    }

    pub fn promotion(&self) -> Option<Piece> {
        match self.inner & 0b1011000000000000 {
            0b1000000000000000 => Some(Piece::Knight),
            0b1001000000000000 => Some(Piece::Bishop),
            0b1010000000000000 => Some(Piece::Rook),
            0b1011000000000000 => Some(Piece::Queen),
            _ => None,
        }
    }

    pub fn is_capture(&self) -> bool {
        self.inner & 0b0100000000000000 != 0
    }

    pub fn is_enpass_capture(&self) -> bool {
        self.inner & 0b1111000000000000 == 0b0101000000000000
    }

    pub fn is_quiet(&self) -> bool {
        self.inner & 0b1111000000000000 == 0
    }

    pub fn is_double_pawn_push(&self) -> bool {
        self.inner & 0b0001000000000000 != 0
    }

    pub fn is_king_castle(&self) -> bool {
        self.inner & 0b1111000000000000 == 0b0010000000000000
    }

    pub fn is_queen_castle(&self) -> bool {
        self.inner & 0b1111000000000000 == 0b0011000000000000
    }

    pub fn castling_kind(&self, color: &Color) -> Option<CastlingKind> {
        if self.is_queen_castle() {
            return Some(match color {
                Color::White => CastlingKind::WhiteQueenside,
                Color::Black => CastlingKind::BlackQueenside,
            });
        }

        if self.is_king_castle() {
            return Some(match color {
                Color::White => CastlingKind::WhiteKingside,
                Color::Black => CastlingKind::BlackKingside,
            });
        }

        None
    }

    pub fn is_irreversible(&self, pos: &Position) -> bool {
        if matches!(self.kind(), MoveKind::Capture | MoveKind::PromotionCapture) {
            return true;
        }

        let (piece, _) = pos.piece_at(&self.from()).unwrap();

        piece == Piece::Pawn
    }

    fn set_promotion(&mut self, promotion: Piece) {
        self.inner |= match promotion {
            Piece::Knight => 0b1000000000000000,
            Piece::Bishop => 0b1001000000000000,
            Piece::Rook => 0b1010000000000000,
            Piece::Queen => 0b1011000000000000,
            _ => panic!("Invalid promotion: {promotion}"),
        }
    }

    fn set_capture(&mut self) {
        self.inner |= 0b0100000000000000;
    }

    fn set_enpass_capture(&mut self) {
        self.inner |= 0b0101000000000000;
    }

    fn set_promotion_capture(&mut self, promotion: Piece) {
        self.set_promotion(promotion);
        self.inner |= 0b0100000000000000;
    }

    fn set_king_castle(&mut self) {
        self.inner |= 0b0010000000000000;
    }

    fn set_queen_castle(&mut self) {
        self.inner |= 0b0011000000000000;
    }

    fn set_double_pawn_push(&mut self) {
        self.inner |= 0b0001000000000000;
    }
}

#[cfg(test)]
mod tests {
    use sdk::{fen::Fen, position::Position, square::Square};

    use super::{MakeMove, Move, MoveKind};

    #[test]
    fn test_make_move_hash_matches() {
        let position = Position::from_fen(
            "rnqk1bnr/ppp2ppp/4P3/8/1P5P/N2P3b/P3PPPR/R1BQKBN1 b Qkq - 0 10".to_string(),
        )
        .unwrap();

        let mv = Move::new(Square::D5, Square::E6, None, &MoveKind::EnPassant);
        let mut position2 = Position::from_fen(
            "rnqk1bnr/ppp2ppp/8/3Pp3/1P5P/N2P3b/P3PPPR/R1BQKBN1 w Qkq e6 0 10".to_string(),
        )
        .unwrap();
        position2.make_move(&mv).unwrap();

        assert_eq!(position.hash, position2.hash);
    }
}
