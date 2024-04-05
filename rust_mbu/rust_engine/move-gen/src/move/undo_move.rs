use sdk::{
    hash::ZOBRIST_KEYS,
    position::{Color, Piece, Position},
};

use crate::r#move::MoveKind;

use super::{Move, Result};

pub trait UndoMove {
    fn undo_move(&mut self, mv: &Move) -> Result<()>;
}

impl UndoMove for Position {
    fn undo_move(&mut self, mv: &Move) -> Result<()> {
        let from = mv.from();
        let to = mv.to();
        let (to_piece, color) = self.piece_at(&to).expect("BUG: No piece at to square");
        let undo_move = self.history.pop().unwrap();

        let enemy = color.enemy();

        match mv.kind() {
            MoveKind::Castling => {
                let castling = mv
                    .castling_kind(&color)
                    .expect("BUG: Move does not castle.");

                let (rook_from, _) = castling.from_squares();
                let (rook_to, king_to) = castling.target_squares();

                let (rook, _) = self.remove_piece_at(&rook_to).unwrap();
                let king = self.remove_piece_at(&king_to).unwrap().0;

                self.add_piece_at(rook_from, rook, color)?;
                self.add_piece_at(from, king, color)?;
            }
            _ => {
                // Rollback moved piece
                self.remove_piece_at(&to)
                    .expect("BUG: No piece at to square");
                self.hash ^= ZOBRIST_KEYS.pieces[color as usize][to_piece as usize][to as usize];
                if matches!(mv.kind(), MoveKind::Promotion | MoveKind::PromotionCapture) {
                    self.add_piece_at(from, Piece::Pawn, color)?;
                } else {
                    self.add_piece_at(from, to_piece, color)?;
                }

                // Rollback captured piece if any
                if let Some(captured) = undo_move.captured {
                    let captured_sq = if matches!(mv.kind(), MoveKind::EnPassant) {
                        undo_move
                            .en_passant
                            .unwrap()
                            .offset(if color == Color::White { -1 } else { 1 }, 0)
                            .unwrap()
                    } else {
                        to
                    };

                    self.add_piece_at(captured_sq, captured, enemy)?;
                }
            }
        }

        // Update castling rights hashes
        if self.castling.inner != undo_move.castling.inner {
            self.castling = undo_move.castling.clone();
        }

        self.turn = color;

        self.halfmove_clock = undo_move.halfmove_clock;
        self.fullmove_number -= if color == Color::Black { 1 } else { 0 };
        self.hash = undo_move.hash;

        Ok(())
    }
}

mod tests {
    use sdk::{fen::Fen, position::Position, square::Square};

    use crate::r#move::{make_move::MakeMove, undo_move::UndoMove, Move, MoveKind};

    #[test]
    fn test_undo_move_quiet() {
        let mut pos = Position::default();
        let mv = Move::new(Square::E2, Square::E4, None, &MoveKind::Quiet);
        pos.make_move(&mv).unwrap();
        pos.undo_move(&mv).unwrap();

        assert_eq!(pos.to_fen(), Position::default().to_fen());
    }

    #[test]
    fn test_undo_move_capture() {
        let mut pos = Position::default();
        let mv1 = Move::new(Square::E2, Square::E4, None, &MoveKind::DoublePawnPush);
        pos.make_move(&mv1).unwrap();
        let mv2 = Move::new(Square::D7, Square::D5, None, &MoveKind::Quiet);
        pos.make_move(&mv2).unwrap();
        let mv3 = Move::new(Square::E4, Square::D5, None, &MoveKind::Capture);
        pos.make_move(&mv3).unwrap();
        pos.undo_move(&mv3).unwrap();
        pos.undo_move(&mv2).unwrap();
        pos.undo_move(&mv1).unwrap();

        assert_eq!(pos.to_fen(), Position::default().to_fen());
    }

    #[test]
    fn test_undo_move_en_passant() {
        let mut pos = Position::default();
        let mv1 = Move::new(Square::E2, Square::E4, None, &MoveKind::DoublePawnPush);
        pos.make_move(&mv1).unwrap();
        let mv2 = Move::new(Square::H7, Square::H6, None, &MoveKind::Quiet);
        pos.make_move(&mv2).unwrap();
        let mv3 = Move::new(Square::E4, Square::E5, None, &MoveKind::Quiet);
        pos.make_move(&mv3).unwrap();
        let mv4 = Move::new(Square::D7, Square::D5, None, &MoveKind::DoublePawnPush);
        pos.make_move(&mv4).unwrap();
        let mv5 = Move::new(Square::E5, Square::D6, None, &MoveKind::EnPassant);
        pos.make_move(&mv5).unwrap();
        pos.undo_move(&mv5).unwrap();
        pos.undo_move(&mv4).unwrap();
        pos.undo_move(&mv3).unwrap();
        pos.undo_move(&mv2).unwrap();
        pos.undo_move(&mv1).unwrap();

        assert_eq!(pos.to_fen(), Position::default().to_fen());
    }
}
