use sdk::{
    bitboard::Bitboard,
    position::{Color, Piece, Position},
    square::FILE_MASKS,
};

pub const ISOLATED_PAWN_PENALTY: i32 = -12;

pub fn penalty_for_isolated_pawns(pos: &Position) -> i32 {
    let white_isolated_pawns = find_isolated_pawns(pos, Color::White).count();
    let black_isolated_pawns = find_isolated_pawns(pos, Color::Black).count();

    (white_isolated_pawns as i32 - black_isolated_pawns as i32) * ISOLATED_PAWN_PENALTY
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

    use crate::engine::eval::pawns::isolated_pawns::find_isolated_pawns;

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
}
