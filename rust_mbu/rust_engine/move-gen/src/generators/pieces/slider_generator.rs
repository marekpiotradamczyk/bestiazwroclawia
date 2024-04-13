use sdk::{
    bitboard::Bitboard,
    lookup::sliders::Slider,
    position::{Piece, Position},
    square::Square,
};

use crate::{
    generators::movegen::MoveGen,
    r#move::{Move, MoveKind},
};

use super::{simple_move_generator::SimpleMoveGenerator, PinnerGenerator};

pub trait SliderMoveGenerator {
    fn generate_slider_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
        king_sq: Square,
    ) -> impl Iterator<Item = Move>;
}

impl SliderMoveGenerator for MoveGen {
    fn generate_slider_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
        king_sq: Square,
    ) -> impl Iterator<Item = Move> {
        [Slider::Bishop, Slider::Rook, Slider::Queen]
            .into_iter()
            .flat_map(move |slider| {
                let piece: Piece = slider.into();

                let bb = pos.pieces[pos.turn as usize][piece as usize];
                let blockers = friendly_occ | enemy_occ;

                bb.into_iter().flat_map(move |from_square| {
                    let maybe_pinner_ray = if pinned_pieces.has(from_square) {
                        self.between_pinner_inclusive(from_square, king_sq, blockers)
                    } else {
                        Bitboard::full()
                    };

                    let attacks = self.slider_moves(slider, from_square, blockers)
                        & !friendly_occ
                        & maybe_pinner_ray;

                    attacks.into_iter().map(move |target_square| {
                        let captured_piece = pos.piece_at(target_square).map(|piece| piece.0);

                        let kind = if captured_piece.is_some() {
                            MoveKind::Capture
                        } else {
                            MoveKind::Quiet
                        };

                        Move::new(from_square, target_square, None, &kind)
                    })
                })
            })
    }
}
