use core::fmt;
use std::{collections::HashSet, fmt::Formatter};

use sdk::{
    bitboard::Bitboard,
    fen::Fen,
    hash::ZOBRIST_KEYS,
    position::{Castling, CastlingKind, Color, Piece, Position, UndoMove},
    square::{File, Square},
};

use super::{Move, MoveKind, Result};

pub trait MakeMove {
    fn make_move(&mut self, mv: &Move) -> Result<()>;
    fn make_null_move(&mut self);
}

impl MakeMove for Position {
    fn make_move(&mut self, mv: &Move) -> Result<()> {
        let from = mv.from();
        let to = mv.to();
        let color = self.turn;

        let mut undo_move = UndoMove {
            castling: self.castling.clone(),
            en_passant: self.en_passant,
            halfmove_clock: self.halfmove_clock,
            occupied: self.occupied,
            hash: self.hash,
            captured: None,
        };

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

        undo_move.captured = captured;
        self.history.push(undo_move);

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

        Ok(())
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

#[cfg(test)]
mod tests {
    use sdk::{fen::Fen, position::Position, square::Square};

    use crate::r#move::MoveKind;

    use super::{MakeMove, Move};

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
