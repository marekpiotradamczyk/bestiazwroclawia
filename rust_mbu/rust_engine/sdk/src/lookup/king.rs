use crate::{
    bitboard::Bitboard,
    square::{File, Rank},
};

const FILE_A: Bitboard = File::A.bitboard();
const FILE_H: Bitboard = File::H.bitboard();
const RANK_1: Bitboard = Rank::R1.bitboard();
const RANK_8: Bitboard = Rank::R8.bitboard();

#[must_use]
pub fn gen_king_attacks() -> [Bitboard; 64] {
    let mut king_attacks = [Bitboard(0); 64];
    for sq in crate::square::Square::iter() {
        king_attacks[sq as usize] = mask_king_attacks(sq.bitboard());
    }
    king_attacks
}

#[must_use]
pub fn mask_king_attacks(king: Bitboard) -> Bitboard {
    let mut attacks = king;
    attacks |= (attacks << 1) & !FILE_A;
    attacks |= (attacks >> 1) & !FILE_H;
    attacks |= (attacks << 8) & !RANK_1;
    attacks |= (attacks >> 8) & !RANK_8;

    attacks ^ king
}
