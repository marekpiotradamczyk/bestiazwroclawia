use move_gen::r#move::Move;
use sdk::{
    position::{Color, Piece, Position},
    square::Square,
};

pub fn see(sq: &Square, pos: &Position) -> isize {
    let mut gain = [0isize; 32];
    let may_xray = {
        let kings = pos.pieces[Color::White as usize][Piece::King as usize]
            | pos.pieces[Color::Black as usize][Piece::King as usize];
        let knights = pos.pieces[Color::White as usize][Piece::Knight as usize]
            | pos.pieces[Color::Black as usize][Piece::Knight as usize];

        pos.occupied & !(kings | knights)
    };
    let mut occ = pos.occupied;

    todo!()
}

fn get_smallest_attacker(sq: &Square, pos: &Position) -> (Square, Piece) {
    todo!()
}
