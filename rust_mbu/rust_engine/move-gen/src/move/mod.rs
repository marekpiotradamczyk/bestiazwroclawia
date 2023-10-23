use core::fmt;
use std::{collections::HashSet, fmt::Formatter};

use sdk::{
    position::{CastlingKind, Color, Piece, Position},
    square::{File, Square},
};

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Move {
    inner: u16,
}

pub trait MakeMove {
    fn make_move(&mut self, mv: &Move) -> Result<Option<Piece>>;
    fn undo_move(&mut self, mv: &Move, captured: Option<Piece>) -> Result<()>;
    fn validate_move(&self, mv: &Move) -> Result<()>;
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
            format!("={}", promotion)
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

        let captured = match mv.kind() {
            MoveKind::Capture | MoveKind::Quiet => {
                let captured = self.remove_piece_at(&to);

                self.add_piece_at(to, from_piece, from_color)?;

                captured
            }
            MoveKind::EnPassant => {
                let captured_sq = mv
                    .to()
                    .offset(if color == Color::White { -1 } else { 1 }, 0)
                    .expect("BUG: Invalid en passant square");

                let captured = self
                    .remove_piece_at(&captured_sq)
                    .expect("BUG: No piece at to square");

                self.add_piece_at(to, from_piece, from_color)?;

                Some(captured)
            }
            MoveKind::Castling => {
                let castling = mv
                    .castling_kind(&self.turn)
                    .expect("BUG: Move does not castle.");

                let (rook_from, _) = castling.from_squares();
                let (rook_to, king_to) = castling.target_squares();

                let (rook, _) = self
                    .remove_piece_at(&rook_from)
                    .expect("BUG: No piece at rook from square");

                self.add_piece_at(king_to, from_piece, from_color)?;
                self.add_piece_at(rook_to, rook, from_color)?;

                None
            }
            MoveKind::Promotion | MoveKind::PromotionCapture => {
                let promotion = mv.promotion().expect("BUG: No promotion piece");
                let captured = self.remove_piece_at(&to);

                self.add_piece_at(to, promotion, from_color)?;

                captured
            }
            MoveKind::DoublePawnPush => {
                let captured = self.remove_piece_at(&to);

                self.add_piece_at(to, from_piece, from_color)?;

                let enpass_sq = mv
                    .to()
                    .offset(0, if color == Color::White { -1 } else { 1 })
                    .expect("BUG: Invalid en passant square");

                self.en_passant = Some(enpass_sq);

                captured
            }
        }
        .map(|(piece, _)| piece);

        self.occupied = self.occupation(&Color::White) | self.occupation(&Color::Black);
        self.en_passant = None;
        self.halfmove_clock = if captured.is_some() || from_piece == Piece::Pawn {
            0
        } else {
            self.halfmove_clock + 1
        };
        let color = self.swap_turn();
        if color == Color::White {
            self.fullmove_number += 1;
        }

        Ok(captured)
    }

    fn undo_move(&mut self, mv: &Move, captured: Option<Piece>) -> Result<()> {
        let from = mv.from();
        let to = mv.to();

        match mv.kind() {
            MoveKind::Quiet | MoveKind::DoublePawnPush => {
                let (piece, color) = self
                    .remove_piece_at(&to)
                    .expect("BUG: No piece at to square");

                self.add_piece_at(from, piece, color)?;
            }
            MoveKind::Capture => {
                let (piece, color) = self
                    .remove_piece_at(&to)
                    .expect("BUG: No piece at to square");

                self.add_piece_at(from, piece, color)?;

                let captured_color = self.turn;
                let captured_piece = captured.expect("BUG: No captured piece in Capture move");

                self.add_piece_at(to, captured_piece, captured_color)?;
            }
            MoveKind::EnPassant => {
                let (piece, color) = self
                    .remove_piece_at(&to)
                    .expect("BUG: No piece at to square");

                self.add_piece_at(from, piece, color)?;

                let captured_color = self.turn;
                let captured_piece = captured.expect("BUG: No captured piece in EnPassant move");

                let enpass_sq = mv
                    .to()
                    .offset(
                        0,
                        if captured_color == Color::White {
                            -8
                        } else {
                            8
                        },
                    )
                    .expect("BUG: Invalid en passant square");

                self.add_piece_at(enpass_sq, captured_piece, captured_color)?;
            }
            MoveKind::Castling => {
                let castling = CastlingKind::from(to);
                let (rook_to, king_to) = castling.target_squares();
                let (rook_from, king_from) = castling.from_squares();

                let (rook, rook_color) = self
                    .remove_piece_at(&rook_to)
                    .expect("BUG: No piece at rook square");

                let (king, color) = self
                    .remove_piece_at(&king_to)
                    .expect("BUG: No piece at king square");

                self.add_piece_at(rook_from, rook, rook_color)?;
                self.add_piece_at(king_from, king, color)?;

                self.castling.add_castling_kind(&castling);
            }
            MoveKind::Promotion => {
                let (piece, color) = self
                    .remove_piece_at(&to)
                    .expect("BUG: No piece at to square");

                self.add_piece_at(from, piece, color)?;
            }
            MoveKind::PromotionCapture => {
                let (_, color) = self
                    .remove_piece_at(&to)
                    .expect("BUG: No piece at to square");

                self.add_piece_at(from, Piece::Pawn, color)?;

                let captured_color = self.turn;
                let captured_piece =
                    captured.expect("BUG: No captured piece in PromotionCapture move");

                self.add_piece_at(to, captured_piece, captured_color)?;
            }
        }

        self.turn = self.swap_turn();
        if self.turn == Color::Black {
            self.fullmove_number -= 1;
        }

        self.halfmove_clock -= 1;

        Ok(())
    }

    fn validate_move(&self, mv: &Move) -> Result<()> {
        let (_, from_color) = self
            .piece_at(&mv.from())
            .ok_or(anyhow::anyhow!("No piece at from square: {}", mv.from()))?;

        if from_color != self.turn {
            return Err(anyhow::anyhow!(
                "Cannot move piece of opposite color: {}",
                mv.from()
            ));
        }

        if let Some((_, to_color)) = self.piece_at(&mv.to()) {
            if from_color == to_color {
                return Err(anyhow::anyhow!(
                    "Cannot capture piece of same color: {}",
                    mv.to()
                ));
            }
        }

        Ok(())
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
            _ => {}
        }

        mv
    }

    pub fn from(&self) -> Square {
        Square::try_from((self.inner & 0b0000000000111111) as u8).expect("Invalid square")
    }

    pub fn to(&self) -> Square {
        Square::try_from(((self.inner & 0b0000111111000000) >> 6) as u8).expect("Invalid square")
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
        self.inner & 0b1111000000000000 != 0
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
}
