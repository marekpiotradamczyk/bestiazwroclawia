use sdk::{
    position::{Color, Piece, Position},
    square::FILE_MASKS,
};

pub const STACKED_PAWN_PENALTY: i32 = -5;

#[must_use]
pub fn stacked_pawns(pos: &Position) -> i32 {
    let white_stacked_pawns = stacked_pawns_count(pos, Color::White);
    let black_stacked_pawns = stacked_pawns_count(pos, Color::Black);

    (white_stacked_pawns as i32 - black_stacked_pawns as i32) * STACKED_PAWN_PENALTY
}

fn stacked_pawns_count(pos: &Position, color: Color) -> usize {
    let pawns_bb = pos.pieces[color as usize][Piece::Pawn as usize];

    let mut count = 0;

    (0..8).for_each(|file| {
        let file_bb = FILE_MASKS[file];

        let pawns_on_file = pawns_bb & file_bb;
        let pawns_count = pawns_on_file.count();

        if pawns_count > 1 {
            count += pawns_count as usize - 1;
        }
    });

    count
}

#[cfg(test)]
mod tests {
    use sdk::{
        fen::Fen,
        position::{Color, Position},
    };

    use crate::engine::eval::pawns::stacked::{stacked_pawns_count, STACKED_PAWN_PENALTY};
    use sdk::position::tests::*;

    #[test]
    fn test_find_doubled_pawns() {
        let pos = Position::default();
        assert_eq!(stacked_pawns_count(&pos, Color::White), 0);

        let pos = Position::from_fen(
            "rnbqkb1r/2p1p1pp/p2P4/1p1P1P2/8/8/PP1P1P1P/RNBQKBNR b KQkq - 0 7".to_string(),
        )
        .unwrap();
        assert_eq!(stacked_pawns_count(&pos, Color::White), 3);
    }

    #[test]
    fn test_stacked_pawns() {
        #[rustfmt::skip]
        let board = [
            0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, p, 0, 0, 0, 0, 
            p, p, p, p, p, 0, 0, 0,
            0, 0, 0, p, 0, 0, 0, 0,
            0, P, P, P, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, P, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let pos = test_board(&board);

        assert_eq!(super::stacked_pawns(&pos), -STACKED_PAWN_PENALTY * 2);
    }
}
