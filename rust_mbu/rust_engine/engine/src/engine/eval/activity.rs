use sdk::{
    bitboard::{Bitboard, Direction},
    position::{Color, Piece, Position},
    square::File,
};

const MOBILITY_BONUS_BISHOP: i32 = 2;
const MOBILITY_BONUS_ROOK: i32 = 2;
const MOBILITY_BONUS_QUEEN: i32 = 2;

#[must_use]
pub fn bonus_for_mobility(position: &Position) -> i32 {
    mobility(position, Color::White) - mobility(position, Color::Black)
}

fn mobility(position: &Position, color: Color) -> i32 {
    let mut bonus = 0;
    let friendly_pieces = position.occupation(&color);

    let bishop_attacks = bishops_fill(position, color) & !friendly_pieces;
    let rook_attacks = rooks_fill(position, color) & !friendly_pieces;
    let queen_attacks = queen_fill(position, color) & !friendly_pieces;

    bonus += i32::from(bishop_attacks.count()) * MOBILITY_BONUS_BISHOP;
    bonus += i32::from(rook_attacks.count()) * MOBILITY_BONUS_ROOK;
    bonus += i32::from(queen_attacks.count()) * MOBILITY_BONUS_QUEEN;

    bonus
}

fn bishops_fill(position: &Position, color: Color) -> Bitboard {
    let bishops = position.pieces[color as usize][Piece::Bishop as usize];
    let empty = !position.occupied;
    let mut gen = Bitboard::empty();

    gen |= attack_fill(bishops, empty, Direction::NorthEast);
    gen |= attack_fill(bishops, empty, Direction::NorthWest);
    gen |= attack_fill(bishops, empty, Direction::SouthEast);
    gen |= attack_fill(bishops, empty, Direction::SouthWest);

    gen
}

#[must_use]
pub fn rooks_fill(position: &Position, color: Color) -> Bitboard {
    let rooks = position.pieces[color as usize][Piece::Rook as usize];
    let empty = !position.occupied;
    let mut gen = Bitboard::empty();

    gen |= attack_fill(rooks, empty, Direction::North);
    gen |= attack_fill(rooks, empty, Direction::South);
    gen |= attack_fill(rooks, empty, Direction::East);
    gen |= attack_fill(rooks, empty, Direction::West);

    gen
}

fn queen_fill(position: &Position, color: Color) -> Bitboard {
    let queens = position.pieces[color as usize][Piece::Queen as usize];
    let empty = !position.occupied;
    let mut gen = Bitboard::empty();

    gen |= attack_fill(queens, empty, Direction::North);
    gen |= attack_fill(queens, empty, Direction::South);
    gen |= attack_fill(queens, empty, Direction::East);
    gen |= attack_fill(queens, empty, Direction::West);
    gen |= attack_fill(queens, empty, Direction::NorthEast);
    gen |= attack_fill(queens, empty, Direction::NorthWest);
    gen |= attack_fill(queens, empty, Direction::SouthEast);
    gen |= attack_fill(queens, empty, Direction::SouthWest);

    gen
}

fn attack_fill(mut gen: Bitboard, mut empty: Bitboard, direction: Direction) -> Bitboard {
    let mut flood = gen;

    macro_rules! fill {
        ($op:tt, $shift:expr) => {
            {
                for _ in 0..5 {
                    gen = (gen $op $shift) & empty;
                    flood |= gen;
                }
                flood |= (gen $op $shift) & empty;

                flood $op $shift
            }
        };
        ($op:tt, $shift:expr, $not:expr) => {
            {
                empty &= $not;
                for _ in 0..5 {
                    gen = (gen $op $shift) & empty;
                    flood |= gen;
                }
                flood |= (gen $op $shift) & empty;

                (flood $op $shift) & $not
            }
        };
    }

    match direction {
        Direction::South => fill!(>>, 8),
        Direction::North => fill!(<<, 8),
        Direction::East => fill!(<<, 1, !File::A.bitboard()),
        Direction::NorthEast => fill!(<<, 9, !File::A.bitboard()),
        Direction::SouthEast => fill!(>>, 7, !File::A.bitboard()),
        Direction::West => fill!(>>, 1, !File::H.bitboard()),
        Direction::SouthWest => fill!(>>, 9, !File::H.bitboard()),
        Direction::NorthWest => fill!(<<, 7, !File::H.bitboard()),
    }
}

#[cfg(test)]
mod tests {
    use sdk::position::tests::*;

    use crate::engine::eval::activity::{
        MOBILITY_BONUS_BISHOP, MOBILITY_BONUS_QUEEN, MOBILITY_BONUS_ROOK,
    };

    #[test]
    fn test_activity() {
        #[rustfmt::skip]
        let board = [
            0, 0, 0, 0, 0, 0, 0, 0, 
            Q, 0, 0, 0, 0, 0, 0, 0, 
            p, p, p, 0, 0, 0, 0, 0,
            P, 0, P, 0, 0, 0, 0, 0,
            0, b, p, 0, 0, 0, 0, 0,
            P, p, p, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, R, 0, 0, 0, 0, 0, 0,
        ];

        let pos = test_board(&board);
        assert_eq!(
            super::bonus_for_mobility(&pos),
            MOBILITY_BONUS_ROOK * 9 - MOBILITY_BONUS_BISHOP * 3 + MOBILITY_BONUS_QUEEN * 11
        );
    }
}
