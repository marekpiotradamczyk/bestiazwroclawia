use sdk::position::{Color, Piece, Position};

#[must_use]
pub fn can_win(pos: &Position, side: Color) -> bool {
    let pieces = pos.pieces[side as usize];
    let pawns = pieces[Piece::Pawn as usize].count();
    let rooks = pieces[Piece::Rook as usize].count();
    let queens = pieces[Piece::Queen as usize].count();

    if pawns > 0 || rooks > 0 || queens > 0 {
        return true;
    }

    let bishops = pieces[Piece::Bishop as usize].count();
    let knights = pieces[Piece::Knight as usize].count();

    !matches!((bishops, knights), (0 | 1, 0) | (0, _))
}
