use sdk::{
    bitboard::{Bitboard, Direction},
    position::{Color, Piece, Position},
    square::{Rank, Square},
};

use crate::{
    generators::movegen::MoveGen,
    r#move::{Move, MoveKind},
};

use super::simple_move_generator::SimpleMoveGenerator;

pub trait PawnMoveGenerator {
    fn generate_pawn_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_>;

    fn generate_pawn_attacks<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_>;
}

impl PawnMoveGenerator for MoveGen {
    fn generate_pawn_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_> {
        let color = pos.turn;
        let bb = pos.pieces[color as usize][Piece::Pawn as usize] & !pinned_pieces;
        let forward = match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        };
        let blockers = friendly_occ | enemy_occ;
        let double_push_blockers = blockers | blockers.shift(&forward);

        let iter = bb.into_iter().flat_map(move |from_square| {
            let single_moves = self.pawn_single_moves(color, from_square) & !blockers;
            let double_moves = self.pawn_double_moves(color, from_square) & !double_push_blockers;

            single_moves
                .into_iter()
                .chain(double_moves.into_iter())
                .flat_map(move |target_square| {
                    let promotion_rank = match color {
                        Color::White => Rank::R8,
                        Color::Black => Rank::R1,
                    };

                    if target_square.rank() == promotion_rank {
                        generate_promotions_vec(from_square, target_square, MoveKind::Promotion)
                    } else {
                        vec![Move::new(
                            from_square,
                            target_square,
                            None,
                            &MoveKind::Quiet,
                        )]
                    }
                    .into_iter()
                })
        });

        Box::new(iter)
    }

    fn generate_pawn_attacks<'a>(
        &'a self,
        pos: &'a Position,
        _friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_> {
        let color = pos.turn;
        let bb = pos.pieces[color as usize][Piece::Pawn as usize] & !pinned_pieces;

        let iter = bb.into_iter().flat_map(move |from_square| {
            let attacks = if let Some(en_passant) = pos.en_passant {
                self.pawn_attacks(color, from_square) & (enemy_occ | en_passant)
            } else {
                self.pawn_attacks(color, from_square) & enemy_occ
            };

            attacks.into_iter().flat_map(move |target_square| {
                let promotion_rank = match color {
                    Color::White => Rank::R8,
                    Color::Black => Rank::R1,
                };

                if let Some(en_passant) = pos.en_passant {
                    if target_square == en_passant {
                        // A hack to check if the move is legal in a very specific, rare occuring
                        // en-passant case, thus we allow ourselves to clone the position and do more
                        // calculations.
                        let mut cloned = pos.clone();
                        let captured_square = match color {
                            Color::White => en_passant.bitboard().shift(&Direction::South),
                            Color::Black => en_passant.bitboard().shift(&Direction::North),
                        };
                        cloned.occupied &= !(from_square.bitboard() | captured_square);
                        if self.is_check(&cloned) {
                            return vec![].into_iter();
                        }

                        return vec![Move::new(
                            from_square,
                            target_square,
                            None,
                            &MoveKind::EnPassant,
                        )]
                        .into_iter();
                    }
                }

                if target_square.rank() == promotion_rank {
                    generate_promotions_vec(from_square, target_square, MoveKind::PromotionCapture)
                } else {
                    let kind = MoveKind::Capture;

                    vec![Move::new(from_square, target_square, None, &kind)]
                }
                .into_iter()
            })
        });

        Box::new(iter)
    }
}

fn generate_promotions_vec(
    from_square: Square,
    target_square: Square,
    kind: MoveKind,
) -> Vec<Move> {
    vec![
        Move::new(from_square, target_square, Some(Piece::Queen), &kind),
        Move::new(from_square, target_square, Some(Piece::Rook), &kind),
        Move::new(from_square, target_square, Some(Piece::Bishop), &kind),
        Move::new(from_square, target_square, Some(Piece::Knight), &kind),
    ]
}
