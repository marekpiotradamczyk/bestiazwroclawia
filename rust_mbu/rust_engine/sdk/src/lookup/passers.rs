use crate::{
    bitboard::{Bitboard, Direction},
    position::Color,
    square::{Rank, Square},
};

pub fn generate_passers() -> Vec<Vec<Bitboard>> {
    let mut result = vec![vec![Bitboard::empty(); 64]; 2];
    for color in Color::iter() {
        for sq in Square::iter() {
            let mask = mask_passer_square(sq, color);

            result[color as usize][sq as usize] = mask;
        }
    }

    result
}

fn mask_passer_square(sq: Square, color: Color) -> Bitboard {
    let direction = if color == Color::White {
        Direction::North
    } else {
        Direction::South
    };

    let mut mask = sq.bitboard().shift(&direction);
    mask |= mask.shift(&Direction::East) | mask.shift(&Direction::West);
    let rank = sq.rank();
    let dest_rank = if color == Color::White {
        Rank::R8.bitboard()
    } else {
        Rank::R1.bitboard()
    };

    if rank.bitboard() == dest_rank {
        return Bitboard::empty();
    }

    while (mask & dest_rank).is_empty() {
        mask |= mask.shift(&direction);
    }

    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_passer_square_black() {
        let sq = Square::B3;
        let color = Color::Black;
        let mask = mask_passer_square(sq, color);
        assert!(!mask.has(sq));
        assert!(!mask.has(Square::A4));
        assert!(!mask.has(Square::C5));
        assert!(mask.has(Square::A1));
        assert!(mask.has(Square::B1));
        assert!(mask.has(Square::C1));
        assert!(mask.has(Square::A2));
        assert!(mask.has(Square::B2));
        assert!(mask.has(Square::C2));
    }

    #[test]
    fn test_mask_passer_square_white() {
        let sq = Square::G2;
        let color = Color::White;
        let mask = mask_passer_square(sq, color);
        assert!(!mask.has(sq));
        assert!(!mask.has(Square::F1));
        assert!(!mask.has(Square::H1));
        assert!(mask.has(Square::F8));
        assert!(mask.has(Square::G8));
        assert!(mask.has(Square::H8));
        assert!(mask.has(Square::F7));
        assert!(mask.has(Square::G7));
        assert!(mask.has(Square::H7));
    }
}
