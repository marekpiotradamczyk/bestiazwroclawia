use sdk::{
    bitboard::{Bitboard, Direction},
    position::{Color, Piece, Position},
};

use crate::engine::MOVE_GEN;

const PROTECTED_PASSED_PAWNS_BONUS: i32 = 30;
const PASSED_PAWNS_BONUS: i32 = 20;

#[must_use]
pub fn passed_pawns(pos: &Position) -> i32 {
    let (white_passed, white_protected) = mask_protected_passed_pawns(pos, Color::White);
    let (black_passed, black_protected) = mask_protected_passed_pawns(pos, Color::Black);

    let passed_bonus = (i32::from(white_passed) - i32::from(black_passed)) * PASSED_PAWNS_BONUS;
    let protected_bonus =
        (i32::from(white_protected) - i32::from(black_protected)) * PROTECTED_PASSED_PAWNS_BONUS;

    passed_bonus + protected_bonus
}

#[must_use]
pub fn mask_protected_passed_pawns(pos: &Position, color: Color) -> (u8, u8) {
    let our_pawns = pos.pieces[color as usize][Piece::Pawn as usize];
    let enemy_pawns = pos.pieces[color.enemy() as usize][Piece::Pawn as usize];

    let back_dirs = if color == Color::White {
        [Direction::SouthEast, Direction::SouthWest]
    } else {
        [Direction::NorthEast, Direction::NorthWest]
    };

    let mut passed_pawns = Bitboard::empty();
    for pawn in our_pawns {
        let front = MOVE_GEN.lookups.passers_bb[color as usize][pawn as usize];

        if (front & enemy_pawns).is_empty() {
            passed_pawns |= pawn.bitboard();
        }
    }

    let protected_pawns = our_pawns.shift(&back_dirs[0]) | our_pawns.shift(&back_dirs[1]);

    let passed_pawns_not_protected = (passed_pawns & !protected_pawns).count();
    let passed_pawns_protected = (passed_pawns & protected_pawns).count();

    (passed_pawns_not_protected, passed_pawns_protected)
}

#[cfg(test)]
mod tests {
    use sdk::position::tests::*;

    use crate::engine::eval::pawns::protected_passed_pawnes::{
        PASSED_PAWNS_BONUS, PROTECTED_PASSED_PAWNS_BONUS,
    };

    #[test]
    fn test_passed_pawns_bonus() {
        #[rustfmt::skip]
        let board = [
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, p, 0,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, p, 0, 0,
                     P, 0, 0, 0, 0, 0, P, 0,
                     0, 0, 0, 0, 0, 0, 0, 0
                    ];
        let pos = test_board(&board);
        assert_eq!(super::passed_pawns(&pos), PASSED_PAWNS_BONUS);

        #[rustfmt::skip]
        let board = [
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, p, 0,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0,
                     0, P, 0, 0, 0, p, 0, 0,
                     P, 0, 0, 0, 0, 0, P, 0,
                     0, 0, 0, 0, 0, 0, 0, 0
                    ];
        let pos = test_board(&board);
        assert_eq!(
            super::passed_pawns(&pos),
            PASSED_PAWNS_BONUS + PROTECTED_PASSED_PAWNS_BONUS
        );
    }
}
