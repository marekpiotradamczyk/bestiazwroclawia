use itertools::Itertools;
use sdk::{
    position::{Piece, Position},
    square::Square,
};

use crate::{
    generators::movegen::MoveGen,
    r#move::{MakeMove, Move, MoveKind},
};

pub trait ChessNotation {
    fn get_from_notation(&self, pos: &Position, mv: &Move) -> (String, String);
    fn to_algebraic_notation(&self, pos: &Position, mv: &Move) -> String;
}

impl ChessNotation for MoveGen {
    fn get_from_notation(&self, pos: &Position, mv: &Move) -> (String, String) {
        let from_square = mv.from();
        let from_piece = pos.piece_at(&from_square);
        let is_pawn = matches!(from_piece.map(|p| p.0), Some(Piece::Pawn));

        let squares = self
            .attacks_to_square(pos, mv.to(), pos.turn, pos.occupied)
            .into_iter()
            .filter(|sq| sq != &from_square && pos.piece_at(sq) == from_piece)
            .collect_vec();

        let mut can_distinguish_by_file = true;
        let mut can_distinguish_by_rank = true;

        for sq in &squares {
            if sq.file() == from_square.file() {
                can_distinguish_by_file = false;
            }
            if sq.rank() == from_square.rank() {
                can_distinguish_by_rank = false;
            }
        }

        let file = from_square.file().to_string();
        let rank = from_square.rank().to_string();
        let empty = "".to_string();

        if squares.is_empty() && is_pawn {
            return (file, empty);
        } else if squares.is_empty() {
            return (empty.clone(), empty);
        }

        if can_distinguish_by_file {
            (file, empty)
        } else if can_distinguish_by_rank {
            (empty, rank)
        } else {
            (empty.clone(), empty)
        }
    }

    fn to_algebraic_notation(&self, pos: &Position, mv: &Move) -> String {
        let (piece, _) = pos.piece_at(&mv.from()).expect("No piece at from square.");

        if piece == Piece::King
            && matches!(mv.kind(), MoveKind::Castling)
            && ((mv.from() == Square::E1 && mv.to() == Square::G1)
                || (mv.from() == Square::E8 && mv.to() == Square::G8))
        {
            return "O-O".to_string();
        }
        if piece == Piece::King
            && matches!(mv.kind(), MoveKind::Castling)
            && ((mv.from() == Square::E1 && mv.to() == Square::C1)
                || (mv.from() == Square::E8 && mv.to() == Square::C8))
        {
            return "O-O-O".to_string();
        }

        let piece_char = piece.to_string().to_uppercase();

        let is_pawn = piece == Piece::Pawn;

        let (from_file, from_rank) = self.get_from_notation(pos, mv);

        let to_square = mv.to().to_string();
        let promoted_to = mv
            .promotion()
            .map(|piece| format!("={}", piece.to_string().to_uppercase()))
            .unwrap_or("".to_string());

        let capture_indicator = if mv.is_capture() { "x" } else { "" };

        let check_indicator = {
            let mut cloned_pos = pos.clone();
            cloned_pos.make_move(mv).unwrap();
            if self.is_check(&cloned_pos) {
                "+"
            } else {
                ""
            }
        };

        if is_pawn && mv.is_capture() {
            format!("{from_file}{from_rank}x{to_square}{promoted_to}{check_indicator}")
        } else if is_pawn {
            format!("{to_square}{promoted_to}{check_indicator}")
        } else {
            format!(
                "{piece_char}{from_file}{from_rank}{capture_indicator}{to_square}{check_indicator}"
            )
        }
    }
}
