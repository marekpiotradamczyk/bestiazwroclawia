use sdk::{bitboard::Bitboard, position::Position, square::Square};

use crate::generators::{movegen::MoveGen, pieces::simple_move_generator::SimpleMoveGenerator};

pub trait XRayGenerator {
    fn xray_rook_attacks(&self, square: Square, occ: Bitboard) -> Bitboard;
    fn xray_bishop_attacks(&self, square: Square, occ: Bitboard) -> Bitboard;
}

impl XRayGenerator for MoveGen {
    fn xray_rook_attacks(&self, square: Square, occ: Bitboard) -> Bitboard {
        let attacks = self.rook_moves(square, occ);
        let blockers = occ & attacks;

        attacks ^ self.rook_moves(square, occ ^ blockers)
    }

    fn xray_bishop_attacks(&self, square: Square, occ: Bitboard) -> Bitboard {
        let attacks = self.bishop_moves(square, occ);
        let blockers = occ & attacks;

        attacks ^ self.bishop_moves(square, occ ^ blockers)
    }
}
