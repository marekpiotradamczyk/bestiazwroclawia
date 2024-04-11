use sdk::position::{Color, Piece, Position};

pub const BONUS_ROOK_OPEN_FILE: i32 = 30;
pub const BONUS_ROOK_SEMI_OPEN_FILE: i32 = 18;

#[must_use]
pub fn bonus_rook_for_open_files(position: &Position) -> i32 {
    let white_rooks = position.pieces[Color::White as usize][Piece::Rook as usize];
    let black_rooks = position.pieces[Color::Black as usize][Piece::Rook as usize];

    let open_files = position.open_files();

    let white_rooks_on_open_files = (white_rooks & open_files).count();
    let black_rooks_on_open_files = (black_rooks & open_files).count();

    (i32::from(white_rooks_on_open_files) - i32::from(black_rooks_on_open_files)) * BONUS_ROOK_OPEN_FILE
}

#[must_use]
pub fn bonus_rook_for_semi_open_files(position: &Position) -> i32 {
    let white_rooks = position.pieces[Color::White as usize][Piece::Rook as usize];
    let black_rooks = position.pieces[Color::Black as usize][Piece::Rook as usize];

    let semi_open_files_white = position.semi_open_files(&Color::White);
    let semi_open_files_black = position.semi_open_files(&Color::Black);

    let white_rooks_on_semi_open_files = (white_rooks & semi_open_files_white).count();
    let black_rooks_on_semi_open_files = (black_rooks & semi_open_files_black).count();

    (i32::from(white_rooks_on_semi_open_files) - i32::from(black_rooks_on_semi_open_files))
        * BONUS_ROOK_SEMI_OPEN_FILE
}

#[cfg(test)]
mod tests {
    use sdk::{fen::Fen, position::Position};

    use crate::engine::eval::rooks::rook_on_open_files::{
        bonus_rook_for_open_files, bonus_rook_for_semi_open_files, BONUS_ROOK_OPEN_FILE,
        BONUS_ROOK_SEMI_OPEN_FILE,
    };

    #[test]
    fn test_rooks_on_open_files() {
        let pos = Position::default();

        assert_eq!(bonus_rook_for_open_files(&pos), 0);

        let pos = Position::from_fen(
            "2bqkbnr/2pppppp/n7/1p6/8/4P3/1PPP1PPP/RNBQKBNR w KQk - 0 4".to_string(),
        )
        .unwrap();

        assert_eq!(bonus_rook_for_open_files(&pos), BONUS_ROOK_OPEN_FILE * 1);
    }

    #[test]
    fn test_rooks_on_semi_open_files() {
        let pos = Position::default();

        assert_eq!(bonus_rook_for_semi_open_files(&pos), 0);

        let pos = Position::from_fen(
            "rnbqkbnr/p1pppppp/8/1P6/8/8/1PPPPPPP/RNBQKBNR b KQkq - 0 2".to_string(),
        )
        .unwrap();

        assert_eq!(
            bonus_rook_for_semi_open_files(&pos),
            BONUS_ROOK_SEMI_OPEN_FILE * 1
        );
    }
}
