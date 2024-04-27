use sdk::{
    bitboard::{Bitboard, Direction},
    position::{Color, Piece, Position},
    square::FILE_MASKS,
};

pub const STRONG_SQUARE_BONUS: i32 = 20;
pub const STRONG_SQUARE_PIECE_BONUS: i32 = 20;

#[must_use]
pub fn bonus(pos: &Position) -> i32 {
    let white_strong_squares = i32::from(strong_squares(pos, Color::White).count());
    let black_strong_squares = i32::from(strong_squares(pos, Color::Black).count());

    (white_strong_squares - black_strong_squares) * STRONG_SQUARE_BONUS
}

#[must_use]
pub fn bonus_for_piece(pos: &Position) -> i32 {
    let white_strong_squares = strong_squares(pos, Color::White);
    let black_strong_squares = strong_squares(pos, Color::Black);

    let mut bonus = 0;

    let white_minor_pieces = pos.pieces[Color::White as usize][Piece::Knight as usize]
        | pos.pieces[Color::White as usize][Piece::Bishop as usize];

    let black_minor_pieces = pos.pieces[Color::Black as usize][Piece::Knight as usize]
        | pos.pieces[Color::Black as usize][Piece::Bishop as usize];

    for sq in white_strong_squares {
        if white_minor_pieces.has(sq) {
            bonus += STRONG_SQUARE_PIECE_BONUS;
        }
    }

    for sq in black_strong_squares {
        if black_minor_pieces.has(sq) {
            bonus -= STRONG_SQUARE_PIECE_BONUS;
        }
    }

    bonus
}

#[must_use]
pub fn strong_squares(pos: &Position, color: Color) -> Bitboard {
    let pawns = pos.pieces[color as usize][Piece::Pawn as usize];

    let (left, right) = match color {
        Color::White => (Direction::NorthWest, Direction::NorthEast),
        Color::Black => (Direction::SouthWest, Direction::SouthEast),
    };

    let maybe_strong_squares = pawns.shift(&left) | pawns.shift(&right);

    let mut strong_squares = Bitboard::empty();

    let enemy_pawns = pos.pieces[color.enemy() as usize][Piece::Pawn as usize];

    for sq in maybe_strong_squares {
        let file = sq.file() as usize;
        let rank = sq.rank() as usize;

        if (color == Color::White && rank < 4) || (color == Color::Black && rank > 3) {
            continue;
        }

        let enemy_pawns_on_left_file = if file == 0 {
            Bitboard::empty()
        } else {
            enemy_pawns & FILE_MASKS[file - 1]
        };

        let enemy_pawns_on_right_file = if file == 7 {
            Bitboard::empty()
        } else {
            enemy_pawns & FILE_MASKS[file + 1]
        };

        let mut is_strong_square = true;

        for enemy_pawn in enemy_pawns_on_left_file | enemy_pawns_on_right_file {
            let enemy_rank = enemy_pawn.rank() as usize;

            match color {
                Color::White => {
                    if enemy_rank > rank {
                        is_strong_square = false;
                        break;
                    }
                }
                Color::Black => {
                    if enemy_rank < rank {
                        is_strong_square = false;
                        break;
                    }
                }
            }
        }

        if is_strong_square {
            strong_squares |= sq.into();
        }
    }

    strong_squares
}

#[cfg(test)]
mod tests {
    use sdk::position::tests::*;

    use crate::engine::eval::pawns::strong_squares::{
        STRONG_SQUARE_BONUS, STRONG_SQUARE_PIECE_BONUS,
    };

    #[test]
    fn test_strong_squares() {
        #[rustfmt::skip]
        let board = [
            0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, p, 0, 0, 0, 0,
            0, 0, 0, B, 0, 0, 0, 0,
            0, P, P, P, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, P, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let pos = test_board(&board);
        assert_eq!(super::bonus(&pos), STRONG_SQUARE_BONUS * 3);
        assert_eq!(super::bonus_for_piece(&pos), STRONG_SQUARE_PIECE_BONUS);
    }
}
