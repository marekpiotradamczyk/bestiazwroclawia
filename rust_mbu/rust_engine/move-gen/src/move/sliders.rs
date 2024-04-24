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
    pub(crate) fn generate_slider_moves(&mut self, move_type: MoveType) {
        let CachedPosInfo {
            pinned,
            king_sq,
            enemy_occ,
            friendly_occ,
            blockers,
            ..
        } = self.cached;

        let pos = self.pos;

        [Slider::Bishop, Slider::Rook, Slider::Queen]
            .into_iter()
            .for_each(|slider| {
                let piece: Piece = slider.into();

                let bb = pos.pieces[pos.turn as usize][piece as usize];

                bb.into_iter().for_each(|from_square| {
                    let maybe_pinner_ray = if pinned.has(from_square) {
                        MOVE_GEN.between_pinner_inclusive(from_square, king_sq, blockers)
                    } else {
                        Bitboard::full()
                    };

                    let attacks = match move_type {
                        MoveType::Capture => {
                            MOVE_GEN.slider_moves(slider, from_square, blockers)
                                & enemy_occ
                                & maybe_pinner_ray
                        }
                        MoveType::NonCapture => {
                            MOVE_GEN.slider_moves(slider, from_square, blockers)
                                & !friendly_occ
                                & maybe_pinner_ray
                                & !enemy_occ
                        }
                    };

                    attacks.into_iter().for_each(|target_square| {
                        let captured_piece = pos.piece_at(target_square).map(|piece| piece.0);

                        let kind = if captured_piece.is_some() {
                            MoveKind::Capture
                        } else {
                            MoveKind::Quiet
                        };

                        let mv = Move::new(from_square, target_square, None, &kind);
                        self.moves.push(mv);
                    })
                })
            });
    }
}
