use sdk::{
    bitboard::Bitboard,
    lookup::sliders::Slider,
    position::{Piece, Position},
};

use crate::{
    generators::movegen::MoveGen,
    r#move::{Move, MoveKind},
};

use super::simple_move_generator::SimpleMoveGenerator;

pub trait SliderMoveGenerator {
    fn generate_slider_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_>;
}

impl SliderMoveGenerator for MoveGen {
    fn generate_slider_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_> {
        let iter = [Slider::Bishop, Slider::Rook, Slider::Queen]
            .into_iter()
            .flat_map(move |slider| {
                let piece: Piece = slider.into();

                let bb = pos.pieces[pos.turn as usize][piece as usize] & !pinned_pieces;
                let blockers = friendly_occ | enemy_occ;

                bb.into_iter().flat_map(move |from_square| {
                    let attacks = self.slider_moves(slider, from_square, blockers) & !friendly_occ;

                    attacks.into_iter().map(move |target_square| {
                        let captured_piece = pos.piece_at(&target_square).map(|piece| piece.0);

                        let kind = if captured_piece.is_some() {
                            MoveKind::Capture
                        } else {
                            MoveKind::Quiet
                        };

                        Move::new(from_square, target_square, None, &kind)
                    })
                })
            });

        Box::new(iter)
    }
}
