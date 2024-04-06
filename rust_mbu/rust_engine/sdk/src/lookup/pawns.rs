use crate::bitboard::Bitboard;
use crate::bitboard::Direction;
use crate::bitboard::EMPTY;
use crate::position::Color;
use crate::square::Rank;
use crate::square::Square;

#[must_use]
pub fn gen_single_pawn_moves() -> Vec<Vec<Bitboard>> {
    let mut pawn_moves = vec![vec![EMPTY; 64]; 2];
    for color in Color::iter() {
        let direction = match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        };

        for sq in Square::iter() {
            pawn_moves[color as usize][sq as usize] |= sq.bitboard().shift(&direction);
        }
    }
    pawn_moves
}

#[must_use]
pub fn gen_double_pawn_moves() -> Vec<Vec<Bitboard>> {
    let mut pawn_moves = vec![vec![EMPTY; 64]; 2];
    for color in Color::iter() {
        let direction = match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        };

        for sq in Square::iter() {
            let rank = sq.rank();
            if rank == Rank::R2 || rank == Rank::R7 {
                pawn_moves[color as usize][sq as usize] |=
                    sq.bitboard().shift(&direction).shift(&direction);
            }
        }
    }
    pawn_moves
}

#[must_use]
pub fn gen_pawn_attacks() -> Vec<Vec<Bitboard>> {
    let mut pawn_attacks = vec![vec![EMPTY; 64]; 2];
    for color in Color::iter() {
        let (first_dir, second_dir) = match color {
            Color::White => (Direction::NorthEast, Direction::NorthWest),
            Color::Black => (Direction::SouthEast, Direction::SouthWest),
        };

        for sq in Square::iter() {
            pawn_attacks[color as usize][sq as usize] |= sq.bitboard().shift(&first_dir);
            pawn_attacks[color as usize][sq as usize] |= sq.bitboard().shift(&second_dir);
        }
    }

    pawn_attacks
}

#[must_use]
pub fn mask_pawns_attacks(bb: Bitboard, color: &Color) -> Bitboard {
    let (first_dir, second_dir) = match color {
        Color::White => (Direction::NorthEast, Direction::NorthWest),
        Color::Black => (Direction::SouthEast, Direction::SouthWest),
    };

    let mut attacks = EMPTY;
    attacks |= bb.shift(&first_dir);
    attacks |= bb.shift(&second_dir);

    attacks
}
