use arrayvec::ArrayVec;
use sdk::bitboard::Direction;
use sdk::lookup::sliders::Slider;
use sdk::position::Color;
use sdk::square::{Rank, Square};
use sdk::{bitboard::Bitboard, position::Piece};

use super::move_list::{CachedPosInfo, MoveList, MoveType, MOVE_GEN};
use super::{Move, MoveKind};
use crate::generators::pieces::simple_move_generator::SimpleMoveGenerator;
use crate::generators::pieces::PinnerGenerator;

impl<'a> MoveList<'a> {
    pub(crate) fn generate_knight_moves(&mut self, move_type: MoveType) {
        let CachedPosInfo {
            pinned,
            enemy_occ,
            friendly_occ,
            color,
            ..
        } = self.cached;

        let pos = self.pos;

        let bb = pos.pieces[color as usize][Piece::Knight as usize] & !pinned;

        bb.into_iter().for_each(|from_square| {
            let attacks = match move_type {
                MoveType::Capture => {
                    MOVE_GEN.knight_attacks(from_square) & !friendly_occ & enemy_occ
                }
                MoveType::NonCapture => {
                    MOVE_GEN.knight_attacks(from_square) & !friendly_occ & !enemy_occ
                }
            };

            attacks.into_iter().for_each(|target_square| {
                let kind = match move_type {
                    MoveType::Capture => MoveKind::Capture,
                    MoveType::NonCapture => MoveKind::Quiet,
                };

                let mv = Move::new(from_square, target_square, None, &kind);
                self.moves.push(mv);
            })
        })
    }
}
