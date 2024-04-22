use sdk::position::{Color, Piece, Position};

use crate::engine::MOVE_GEN;

pub const KING_SAFETY_TABLE: [i32; 150] = [
    0, 0, 0, 1, 1, 2, 3, 4, 5, 6, 8, 10, 13, 16, 20, 25, 30, 36, 42, 48, 55, 62, 70, 80, 90, 100,
    110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290,
    300, 310, 320, 330, 340, 350, 360, 370, 380, 390, 400, 410, 420, 430, 440, 450, 460, 470, 480,
    490, 500, 510, 520, 530, 540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
];

pub const PIECE_ATTACK_UNITS: [i32; 6] = [0, 2, 2, 3, 5, 0];
const BONUS_FOR_UNIT: i32 = 15;
const MAX_BONUS_FOR_UNIT: i32 = 60;

pub fn calc_king_safety(position: &Position) -> i32 {
    let white_units = calc_king_safety_units(position, Color::White);
    let black_units = calc_king_safety_units(position, Color::Black);

    -(KING_SAFETY_TABLE[white_units as usize] - KING_SAFETY_TABLE[black_units as usize])
}

#[must_use]
pub fn bonus_for_pieces_close_to_king(position: &Position) -> i32 {
    let white_count = pieces_close_to_king_count(position, Color::White);
    let black_count = pieces_close_to_king_count(position, Color::Black);

    ((white_count - black_count) * BONUS_FOR_UNIT).clamp(-MAX_BONUS_FOR_UNIT, MAX_BONUS_FOR_UNIT)
}

fn pieces_close_to_king_count(position: &Position, color: Color) -> i32 {
    let king_sq = position.pieces[color as usize][Piece::King as usize].lsb();

    let near_king = MOVE_GEN.lookups.squares_near_king[color as usize][king_sq as usize];
    let friendly_pieces_count = i32::from((position.occupation(&color) & near_king).count()) - 1;
    let enemy_pieces = i32::from((position.occupation(&color.enemy()) & near_king).count());

    friendly_pieces_count - enemy_pieces
}

#[must_use]
pub fn calc_king_safety_units(position: &Position, color: Color) -> i32 {
    let king_sq = position.pieces[color as usize][Piece::King as usize].lsb();

    let near_king = MOVE_GEN.lookups.squares_near_king[color as usize][king_sq as usize];

    let mut bonus: i32 = 0;

    for sq in near_king {
        for piece_sq in MOVE_GEN.attacks_to_square(position, sq, color.enemy(), position.occupied) {
            let (piece, _) = position.piece_at(piece_sq).unwrap();

            bonus += PIECE_ATTACK_UNITS[piece as usize];
        }
    }

    bonus
}
