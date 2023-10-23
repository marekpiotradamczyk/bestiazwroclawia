use sdk::{
    bitboard::Bitboard,
    position::{CastlingKind, Color, Piece, Position},
    square::Square,
};

use crate::{
    generators::movegen::MoveGen,
    r#move::{Move, MoveKind},
};

use super::simple_move_generator::SimpleMoveGenerator;

pub trait KingMoveGenerator {
    fn generate_king_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_>;

    fn generate_all_castlings<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_>;

    fn generate_castling<'a>(
        &'a self,
        pos: &'a Position,
        castling_kind: &CastlingKind,
        king_square: Square,
        occ: Bitboard,
    ) -> Option<Move>;
}

impl KingMoveGenerator for MoveGen {
    fn generate_king_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        _enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_> {
        let color = pos.turn;
        let bb = pos.pieces[color as usize][Piece::King as usize] & !pinned_pieces;

        let iter = bb.into_iter().flat_map(move |from_square| {
            let attacks = self.king_attacks(from_square) & !friendly_occ;

            attacks.into_iter().filter_map(move |target_square| {
                // Move is illegal if it leaves the king in check
                if !self
                    .attacks_to_square(pos, target_square, pos.enemy(), pos.occupied & !bb)
                    .is_empty()
                {
                    return None;
                }

                let captured_piece = pos.piece_at(&target_square).map(|piece| piece.0);
                let kind = if captured_piece.is_some() {
                    MoveKind::Capture
                } else {
                    MoveKind::Quiet
                };

                Some(Move::new(from_square, target_square, None, &kind))
            })
        });

        Box::new(iter)
    }

    fn generate_all_castlings<'a>(
        &'a self,
        pos: &'a Position,
        _friendly_occ: Bitboard,
        _enemy_occ: Bitboard,
        _pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_> {
        let king_square = pos.pieces[pos.turn as usize][Piece::King as usize].msb();

        let occ = pos.occupation(&Color::White) | pos.occupation(&Color::Black);

        let mut castling_moves = Vec::new();

        if !self.is_check(pos) {
            for castling_kind in &[
                CastlingKind::WhiteKingside,
                CastlingKind::WhiteQueenside,
                CastlingKind::BlackKingside,
                CastlingKind::BlackQueenside,
            ] {
                if let Some(mv) = self.generate_castling(pos, castling_kind, king_square, occ) {
                    castling_moves.push(mv);
                }
            }
        }

        Box::new(castling_moves.into_iter())
    }

    fn generate_castling<'a>(
        &'a self,
        pos: &'a Position,
        castling_kind: &CastlingKind,
        king_square: Square,
        occ: Bitboard,
    ) -> Option<Move> {
        let color = match castling_kind {
            CastlingKind::WhiteKingside | CastlingKind::WhiteQueenside => Color::White,
            CastlingKind::BlackKingside | CastlingKind::BlackQueenside => Color::Black,
        };

        if pos.turn != color {
            return None;
        }

        match castling_kind {
            CastlingKind::WhiteKingside | CastlingKind::WhiteQueenside => {
                if king_square != Square::E1 {
                    return None;
                }
            }
            CastlingKind::BlackKingside | CastlingKind::BlackQueenside => {
                if king_square != Square::E8 {
                    return None;
                }
            }
        };

        let between_bb = match castling_kind {
            CastlingKind::WhiteKingside => Square::F1.bitboard() | Square::G1.bitboard(),
            CastlingKind::WhiteQueenside => Square::C1.bitboard() | Square::D1.bitboard(),
            CastlingKind::BlackKingside => Square::F8.bitboard() | Square::G8.bitboard(),
            CastlingKind::BlackQueenside => Square::C8.bitboard() | Square::D8.bitboard(),
        };

        if !(between_bb & occ).is_empty() {
            return None;
        }

        let target_square: Square = match castling_kind {
            CastlingKind::WhiteKingside | CastlingKind::BlackKingside => {
                (king_square as u8 + 2).try_into().unwrap()
            }
            CastlingKind::WhiteQueenside | CastlingKind::BlackQueenside => {
                (king_square as u8 - 2).try_into().unwrap()
            }
        };

        if pos.castling.has_castling_kind(castling_kind)
            && pos.turn == color
            && (occ & between_bb).is_empty()
            && between_bb.into_iter().all(|sq| {
                self.attacks_to_square(pos, sq, pos.enemy(), pos.occupied)
                    .is_empty()
            })
        {
            return Some(Move::new(
                king_square,
                target_square,
                None,
                &MoveKind::Castling,
            ));
        }

        None
    }
}
