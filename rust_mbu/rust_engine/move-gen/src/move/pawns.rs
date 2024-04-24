use arrayvec::ArrayVec;
use sdk::bitboard::Direction;
use sdk::position::Color;
use sdk::square::{Rank, Square};
use sdk::{bitboard::Bitboard, position::Piece};

use super::move_list::{CachedPosInfo, MoveList, MoveType, MOVE_GEN};
use super::{Move, MoveKind};
use crate::generators::pieces::simple_move_generator::SimpleMoveGenerator;
use crate::generators::pieces::PinnerGenerator;

impl<'a> MoveList<'a> {
    pub(crate) fn generate_pawn_moves(&mut self, move_type: MoveType) {
        match move_type {
            MoveType::Capture => self.generate_pawn_captures(),
            MoveType::NonCapture => self.generate_pawn_quiet_moves(),
        }
    }

    fn generate_pawn_captures(&mut self) {
        let CachedPosInfo {
            pinned,
            king_sq,
            enemy_occ,
            friendly_occ,
            blockers: _,
            color,
        } = self.cached;
        let pos = self.pos;

        let blockers = friendly_occ | enemy_occ;
        let bb = self.pos.pieces[color as usize][Piece::Pawn as usize];

        bb.into_iter().for_each(|from_square| {
            let maybe_pinner_ray = if pinned.has(from_square) {
                MOVE_GEN.between_pinner_inclusive(from_square, king_sq, blockers)
            } else {
                Bitboard::full()
            };

            let attacks = if let Some(en_passant) = pos.en_passant {
                MOVE_GEN.pawn_attacks(color, from_square)
                    & (enemy_occ | en_passant)
                    & maybe_pinner_ray
            } else {
                MOVE_GEN.pawn_attacks(color, from_square) & enemy_occ & maybe_pinner_ray
            };

            attacks.into_iter().for_each(|target_square| {
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
                        cloned.pieces[color as usize][Piece::Pawn as usize] &=
                            !from_square.bitboard();
                        cloned.pieces[color.enemy() as usize][Piece::Pawn as usize] &=
                            !captured_square;
                        if !MOVE_GEN.is_check(&cloned) {
                            let mv =
                                Move::new(from_square, target_square, None, &MoveKind::EnPassant);

                            self.moves.push(mv);
                        }
                    }
                }

                if target_square.rank() == promotion_rank {
                    self.moves.extend(generate_promotions_vec(
                        from_square,
                        target_square,
                        MoveKind::PromotionCapture,
                    ));
                } else {
                    let mv = Move::new(from_square, target_square, None, &MoveKind::Capture);

                    self.moves.push(mv);
                }
            })
        });
    }

    fn generate_pawn_quiet_moves(&mut self) {
        let CachedPosInfo {
            pinned,
            king_sq: king_square,
            enemy_occ,
            friendly_occ,
            blockers: _,
            color,
        } = self.cached;
        let pos = self.pos;

        let bb = pos.pieces[color as usize][Piece::Pawn as usize];
        let forward = match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        };
        let blockers = friendly_occ | enemy_occ;
        let double_push_blockers = blockers | blockers.shift(&forward);

        bb.into_iter().for_each(|from_square| {
            let maybe_pinner_ray = if pinned.has(from_square) {
                MOVE_GEN.between_pinner_inclusive(from_square, king_square, blockers)
            } else {
                Bitboard::full()
            };

            let single_moves =
                MOVE_GEN.pawn_single_moves(color, from_square) & !blockers & maybe_pinner_ray;
            let double_moves = MOVE_GEN.pawn_double_moves(color, from_square)
                & !double_push_blockers
                & maybe_pinner_ray;

            single_moves
                .into_iter()
                .chain(double_moves)
                .for_each(|target_square| {
                    let promotion_rank = match color {
                        Color::White => Rank::R8,
                        Color::Black => Rank::R1,
                    };

                    if target_square.rank() == promotion_rank {
                        let promotions = generate_promotions_vec(
                            from_square,
                            target_square,
                            MoveKind::Promotion,
                        );
                        self.moves.extend(promotions);
                    } else {
                        let kind = if double_moves.has(target_square) {
                            MoveKind::DoublePawnPush
                        } else {
                            MoveKind::Quiet
                        };

                        let mv = Move::new(from_square, target_square, None, &kind);
                        self.moves.push(mv);
                    }
                })
        })
    }
}

fn generate_promotions_vec(
    from_square: Square,
    target_square: Square,
    kind: MoveKind,
) -> ArrayVec<Move, 4> {
    ArrayVec::from([
        Move::new(from_square, target_square, Some(Piece::Queen), &kind),
        Move::new(from_square, target_square, Some(Piece::Rook), &kind),
        Move::new(from_square, target_square, Some(Piece::Bishop), &kind),
        Move::new(from_square, target_square, Some(Piece::Knight), &kind),
    ])
}
