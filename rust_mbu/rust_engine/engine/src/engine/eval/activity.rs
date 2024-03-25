use sdk::{
    bitboard::{Bitboard, Direction},
    position::{Color, Piece, Position},
};

const MOBILITY_BONUS_BISHOP: i32 = 2;
const MOBILITY_BONUS_ROOK: i32 = 2;
const MOBILITY_BONUS_QUEEN: i32 = 2;

pub fn bonus_for_mobility(position: &Position) -> i32 {
    mobility(position, Color::White) - mobility(position, Color::Black)
}

fn mobility(position: &Position, color: Color) -> i32 {
    let mut bonus = 0;
    let friendly_pieces = position.occupation(&color);

    let bishop_attacks = bishops_fill(position, color) & !friendly_pieces;
    let rook_attacks = rooks_fill(position, color) & !friendly_pieces;
    let queen_attacks = queen_fill(position, color) & !friendly_pieces;

    bonus += bishop_attacks.count() as i32 * MOBILITY_BONUS_BISHOP;
    bonus += rook_attacks.count() as i32 * MOBILITY_BONUS_ROOK;
    bonus += queen_attacks.count() as i32 * MOBILITY_BONUS_QUEEN;

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

fn attack_fill(gen: Bitboard, pro: Bitboard, direction: Direction) -> Bitboard {
    occluded_fill(gen, pro, direction).shift(&direction)
}

fn occluded_fill(mut gen: Bitboard, mut pro: Bitboard, direction: Direction) -> Bitboard {
    match direction {
        Direction::South => {
            gen |= pro & (gen >> 8);
            pro &= pro >> 8;
            gen |= pro & (gen >> 16);
            pro &= pro >> 16;
            gen |= pro & (gen >> 32);

            gen
        }
        Direction::North => {
            gen |= pro & (gen << 8);
            pro &= pro << 8;
            gen |= pro & (gen << 16);
            pro &= pro << 16;
            gen |= pro & (gen << 32);

            gen
        }
        Direction::East => {
            gen |= pro & (gen << 1);
            pro &= pro << 1;
            gen |= pro & (gen << 2);
            pro &= pro << 2;
            gen |= pro & (gen << 4);

            gen
        }
        Direction::West => {
            gen |= pro & (gen >> 1);
            pro &= pro >> 1;
            gen |= pro & (gen >> 2);
            pro &= pro >> 2;
            gen |= pro & (gen >> 4);

            gen
        }
        Direction::NorthEast => {
            gen |= pro & (gen << 9);
            pro &= pro << 9;
            gen |= pro & (gen << 18);
            pro &= pro << 18;
            gen |= pro & (gen << 36);

            gen
        }
        Direction::NorthWest => {
            gen |= pro & (gen << 7);
            pro &= pro << 7;
            gen |= pro & (gen << 14);
            pro &= pro << 14;
            gen |= pro & (gen << 28);

            gen
        }
        Direction::SouthEast => {
            gen |= pro & (gen >> 9);
            pro &= pro >> 9;
            gen |= pro & (gen >> 18);
            pro &= pro >> 18;
            gen |= pro & (gen >> 36);

            gen
        }
        Direction::SouthWest => {
            gen |= pro & (gen >> 7);
            pro &= pro >> 7;
            gen |= pro & (gen >> 14);
            pro &= pro >> 14;
            gen |= pro & (gen >> 28);

            gen
        }
    }
}
