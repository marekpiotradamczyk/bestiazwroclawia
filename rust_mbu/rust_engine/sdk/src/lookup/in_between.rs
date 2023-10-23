use crate::bitboard::Bitboard;

#[must_use]
pub fn generate_in_between_squares() -> [[Bitboard; 64]; 64] {
    let mut result = [[Bitboard(0); 64]; 64];

    for sq1 in 0..64u8 {
        for sq2 in 0..64u8 {
            result[sq1 as usize][sq2 as usize] = between(sq1, sq2).into();
        }
    }

    result
}

#[allow(arithmetic_overflow)]
fn between(sq1: u8, sq2: u8) -> u64 {
    let sq1 = u64::from(sq1);
    let sq2 = u64::from(sq2);

    let m1 = u64::MAX;
    let a2a7: u64 = 0x0001_0101_0101_0100;
    let b2g7: u64 = 0x0040_2010_0804_0200;
    let h1b7: u64 = 0x0002_0408_1020_4080;

    let btwn: u64 = (m1 << sq1) ^ (m1 << sq2);

    let file: u64 = (sq2 & 7).wrapping_sub(sq1 & 7);
    let rank: u64 = ((sq2 | 7).wrapping_sub(sq1)) >> 3;

    let mut line = ((file & 7).wrapping_sub(1)) & a2a7; 
    line += 2 * (((rank & 7).wrapping_sub(1)) >> 58);
    line += (((rank.wrapping_sub(file)) & 15).wrapping_sub(1)) & b2g7;
    line += (((rank.wrapping_add(file)) & 15).wrapping_sub(1)) & h1b7;
    let mut line = Into::<u64>::into(line);
    line = line.wrapping_mul(btwn & btwn.wrapping_neg());

    line & btwn 
}
