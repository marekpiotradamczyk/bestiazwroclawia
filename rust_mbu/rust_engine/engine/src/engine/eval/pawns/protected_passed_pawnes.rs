use sdk::{
    bitboard::{Bitboard, Direction},
    position::{Color, Piece, Position},
    square::FILE_MASKS,
};

const PROTECTED_PASSED_PAWNS_BONUS: i32 = 30;
const PASSED_PAWNS_BONUS: i32 = 20;

#[must_use]
pub fn passed_pawns(pos: &Position) -> i32 {
    let (white_passed, white_protected) = mask_protected_passed_pawns(pos, Color::White);
    let (black_passed, black_protected) = mask_protected_passed_pawns(pos, Color::Black);

    let passed_bonus = i32::from(white_passed - black_passed) * PASSED_PAWNS_BONUS;
    let protected_bonus =
        i32::from(white_protected - black_protected) * PROTECTED_PASSED_PAWNS_BONUS;

    passed_bonus + protected_bonus
}

#[must_use]
pub fn mask_protected_passed_pawns(pos: &Position, color: Color) -> (u8, u8) {
    let our_pawns = pos.pieces[color as usize][Piece::Pawn as usize];
    let enemy_pawns = pos.pieces[color.enemy() as usize][Piece::Pawn as usize];

    let dirs = if color == Color::White {
        [Direction::NorthEast, Direction::NorthWest]
    } else {
        [Direction::SouthEast, Direction::SouthWest]
    };

    let protected_pawns = our_pawns.shift(&dirs[0]) | our_pawns.shift(&dirs[1]);

    let mut passed_pawns = Bitboard::empty();
    for pawn in our_pawns {
        let file = pawn.file() as usize;

        let no_enemy_pawns_on_left_file =
            file == 0 || (enemy_pawns & FILE_MASKS[file - 1]).is_empty();
        let no_enemy_pawns_on_right_file =
            file == 7 || (enemy_pawns & FILE_MASKS[file + 1]).is_empty();

        if no_enemy_pawns_on_left_file && no_enemy_pawns_on_right_file {
            passed_pawns |= pawn.bitboard();
        }
    }

    let passed_pawns_not_protected = (passed_pawns & !protected_pawns).count();
    let passed_pawns_protected = (passed_pawns & protected_pawns).count();

    (passed_pawns_not_protected, passed_pawns_protected)
}
