use std::sync::Arc;

use move_gen::generators::movegen::MoveGen;
use sdk::position::{Color, Piece, Position};

pub const KING_SAFETY_TABLE: [i32; 100] = [
    0, 0, 0, 1, 1, 2, 3, 4, 5, 6, 8, 10, 13, 16, 20, 25, 30, 36, 42, 48, 55, 62, 70, 80, 90, 100,
    110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290,
    300, 310, 320, 330, 340, 350, 360, 370, 380, 390, 400, 410, 420, 430, 440, 450, 460, 470, 480,
    490, 500, 510, 520, 530, 540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
];

pub const PIECE_ATTACK_UNITS: [i32; 6] = [0, 2, 2, 3, 5, 0];

pub fn calc_king_safety(position: &Position, move_gen: Arc<MoveGen>) -> i32 {
    let white_units = calc_king_safety_units(position, move_gen.clone(), Color::White);
    let black_units = calc_king_safety_units(position, move_gen.clone(), Color::Black);

    -(KING_SAFETY_TABLE[white_units as usize] - KING_SAFETY_TABLE[black_units as usize])
}

pub fn calc_king_safety_units(position: &Position, move_gen: Arc<MoveGen>, color: Color) -> i32 {
    let king_sq = position.pieces[color as usize][Piece::King as usize].lsb();

    let near_king = move_gen.lookups.squares_near_king[color as usize][king_sq as usize];

    let mut bonus: i32 = 0;

    for sq in near_king {
        for piece_sq in move_gen.attacks_to_square(position, sq, color.enemy(), position.occupied) {
            let (piece, _) = position.piece_at(&piece_sq).unwrap();

            bonus += PIECE_ATTACK_UNITS[piece as usize];
        }
    }

    bonus
}
