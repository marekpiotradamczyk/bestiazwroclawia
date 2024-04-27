use sdk::{
    bitboard::Bitboard,
    position::{Color, Piece, Position},
    square::FILE_MASKS,
};

pub const ISOLATED_PAWN_PENALTY: i32 = -12;

#[must_use]
pub fn isolated_pawns(pos: &Position) -> i32 {
    let white_isolated_pawns = find_isolated_pawns(pos, Color::White).count();
    let black_isolated_pawns = find_isolated_pawns(pos, Color::Black).count();

    (i32::from(white_isolated_pawns) - i32::from(black_isolated_pawns)) * ISOLATED_PAWN_PENALTY
}

fn find_isolated_pawns(pos: &Position, color: Color) -> Bitboard {
    let pawns_bb = pos.pieces[color as usize][Piece::Pawn as usize];

    let mut result = Bitboard::empty();

    for sq in pawns_bb {
        let file = sq.file() as isize;

        let no_pawns_on_left_file =
            file - 1 < 0 || (pawns_bb & FILE_MASKS[file as usize - 1]).is_empty();
        let no_pawns_on_right_file =
            file + 1 >= 8 || (pawns_bb & FILE_MASKS[file as usize + 1]).is_empty();

        if no_pawns_on_left_file && no_pawns_on_right_file {
            result |= sq.bitboard();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use sdk::{
        fen::Fen,
        position::{Color, Position},
    };

    use crate::engine::eval::pawns::isolated::{find_isolated_pawns, ISOLATED_PAWN_PENALTY};
    use sdk::position::tests::*;

    #[test]
    fn test_find_isolated_pawns() {
        let pos = Position::default();
        assert!(find_isolated_pawns(&pos, Color::White).is_empty());

        let pos = Position::from_fen(
            "rnbqkbnr/pp2pPpp/1P6/3p4/8/2p5/P1PP3P/RNBQKBNR w KQkq - 0 1".to_string(),
        )
        .unwrap();
        assert_eq!(find_isolated_pawns(&pos, Color::White).count(), 2);
    }

    #[test]
    fn test_isolated_pawns() {
        #[rustfmt::skip]
        let board = [
            0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, p, 0, 0, 0, 0, 
            p, p, p, p, p, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, P, P, P, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, P, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let pos = test_board(&board);
        assert_eq!(super::isolated_pawns(&pos), ISOLATED_PAWN_PENALTY);
    }
}
