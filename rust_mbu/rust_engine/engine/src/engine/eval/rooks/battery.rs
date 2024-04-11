use sdk::position::{Color, Piece, Position};

pub const BATTERY_BONUS: i32 = 15;

#[must_use]
pub fn bonus_for_rook_batteries(pos: &Position) -> i32 {
    let white_batteries = count_batteries(pos, Color::White);
    let black_batteries = count_batteries(pos, Color::Black);

    (white_batteries as i32 - black_batteries as i32) * BATTERY_BONUS
}

fn count_batteries(pos: &Position, side: Color) -> usize {
    let mut rooks = pos.pieces[side as usize][Piece::Rook as usize];

    let mut count = 0;

    while !rooks.is_empty() {
        let rook = rooks.lsb();
        let file = rook.file();

        let rooks_on_file_bb = rooks & file.bitboard();

        if rooks_on_file_bb.count() > 1 {
            count += 1;
        }

        rooks ^= rooks_on_file_bb;
    }

    count
}
