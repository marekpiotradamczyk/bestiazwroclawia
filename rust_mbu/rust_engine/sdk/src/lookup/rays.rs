use crate::{bitboard::{Bitboard, Direction}, square::Square};

#[must_use]
pub fn generate_rays_attacks() -> [[Bitboard; 64]; 8] {
    let mut attacks = [[Bitboard::empty(); 64]; 8];

    for direction in Direction::all() {
        let (file_offset, rank_offset) = direction.offsets();


        for sq in Square::iter() {
            let mut bb = sq.bitboard();
            let mut current_sq = sq;

            while let Some(next_sq) = current_sq.offset(rank_offset, file_offset) {
                bb |= next_sq.bitboard();
                current_sq = next_sq;
            }

            attacks[direction as usize][sq as usize] = bb;
        }
    }


    attacks
}
