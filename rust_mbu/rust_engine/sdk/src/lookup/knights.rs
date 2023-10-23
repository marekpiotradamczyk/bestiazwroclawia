use crate::{
    bitboard::Bitboard,
    square::{File, Square},
};

const FILE_A: Bitboard = File::A.bitboard();
const FILE_B: Bitboard = File::B.bitboard();
const FILE_G: Bitboard = File::G.bitboard();
const FILE_H: Bitboard = File::H.bitboard();
const FILE_AB: Bitboard = Bitboard(FILE_A.0 | FILE_B.0);
const FILE_GH: Bitboard = Bitboard(FILE_G.0 | FILE_H.0);

#[must_use]
pub fn gen_knight_attacks() -> [Bitboard; 64] {
    let mut knight_attacks = [Bitboard(0); 64];
    for sq in Square::iter() {
        knight_attacks[sq as usize] = mask_knights_attacks(sq.into());
    }
    knight_attacks
}

#[must_use]
pub fn mask_knights_attacks(bb: Bitboard) -> Bitboard {
    let mut knight_bb: Bitboard = Bitboard(0);

    knight_bb |= (bb << 17) & !FILE_A;
    knight_bb |= (bb << 15) & !FILE_H;
    knight_bb |= (bb << 10) & !FILE_AB;
    knight_bb |= (bb << 6) & !FILE_GH;
    knight_bb |= (bb >> 17) & !FILE_H;
    knight_bb |= (bb >> 15) & !FILE_A;
    knight_bb |= (bb >> 10) & !FILE_GH;
    knight_bb |= (bb >> 6) & !FILE_AB;

    knight_bb 
}
