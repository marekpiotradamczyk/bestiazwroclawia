use anyhow::anyhow;

use crate::{
    bitboard::Bitboard,
    position::{Castling, Color, Piece, Position},
    square::Square,
};

pub trait Fen {
    fn from_fen(fen: String) -> anyhow::Result<Position>;
    fn to_fen(&self) -> String;
}

impl Fen for Position {
    #[allow(clippy::too_many_lines)]
    fn from_fen(fen: String) -> anyhow::Result<Position> {
        let mut position = Position {
            pieces: [[Bitboard(0); 6]; 2],
            occupied: Bitboard(0),
            turn: Color::White,
            castling: Castling::empty(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };
        let mut fen = fen.split_whitespace();
        let ranks = fen.next().unwrap().split('/');
        let size = ranks.clone().count();
        if size != 8 {
            return Err(anyhow::anyhow!(
                "Invalid FEN: Invalid number of ranks, got {}, expected 8",
                size
            ));
        }

        for (rank, rank_str) in ranks.enumerate() {
            let mut file = 0;
            for c in rank_str.chars() {
                if let Some(digit) = c.to_digit(10) {
                    file += digit as usize;
                } else {
                    let color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

                    let piece = match c.to_ascii_lowercase() {
                        'p' => Piece::Pawn,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'r' => Piece::Rook,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => {
                            return Err(anyhow::anyhow!(
                                "Invalid FEN: Invalid piece character {}",
                                c
                            ))
                        }
                    };

                    let square: Square =
                        ((7 - u8::try_from(rank)?) * 8 + u8::try_from(file)?).try_into()?;
                    file += 1;
                    let idx = piece as usize;

                    position.pieces[color as usize][idx] |= square.bitboard();
                }
            }
        }

        position.occupied = position.occupation(&Color::White) | position.occupation(&Color::Black);

        let turn = fen.next().unwrap();
        position.turn = match turn {
            "w" => Color::White,
            "b" => Color::Black,
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid FEN: Invalid turn character {}",
                    turn
                ))
            }
        };

        let castling: Castling = fen.next().unwrap().parse()?;

        position.castling = castling;

        let en_pass = fen.next().unwrap();
        position.en_passant = if en_pass == "-" {
            None
        } else {
            let mut chars = en_pass.chars();
            let file_char = chars
                .next()
                .ok_or(anyhow!("Invalid FEN: Invalid en passant square: {en_pass}"))?;
            let rank_char = chars
                .next()
                .ok_or(anyhow!("Invalid FEN: Invalid en passant square: {en_pass}"))?;
            if (file_char as u8) < b'a' || (file_char as u8) > b'h' {
                return Err(anyhow!("Invalid FEN: Invalid en passant file: {file_char}, expected a-h. En passant square: {en_pass}"));
            }
            if (rank_char as u8) < b'1' || (rank_char as u8) > b'8' {
                return Err(anyhow!("Invalid FEN: Invalid en passant rank: {rank_char}, expected 1-8. En passant square: {en_pass}"));
            }
            let file = file_char as u8 - b'a';
            let rank = rank_char as u8 - b'1';

            (rank * 8 + file).try_into().ok()
        };

        Ok(position)
    }

    fn to_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty = 0;
        for rank in (0..8u8).rev() {
            for file in 0..8u8 {
                let square: Square = (rank * 8 + file)
                    .try_into()
                    .expect("BUG: Square out of bounds");
                let piece = self.piece_at(&square);
                if let Some((piece, color)) = piece {
                    if empty != 0 {
                        fen.push_str(&format!("{empty}"));
                        empty = 0;
                    }
                    match color {
                        Color::White => fen.push_str(&format!("{piece}").to_uppercase()),
                        Color::Black => fen.push_str(&format!("{piece}").to_lowercase()),
                    }
                } else {
                    empty += 1;
                }
            }
            if empty != 0 {
                fen.push_str(&format!("{empty}"));
                empty = 0;
            }
            if rank != 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push_str(&format!("{}", self.turn));
        fen.push(' ');
        fen.push_str(self.castling.to_string().as_str());
        fen.push(' ');
        if let Some(square) = &self.en_passant {
            fen.push_str(&square.coords_str());
        } else {
            fen.push('-');
        }
        fen.push(' ');
        fen.push_str(&format!("{}", self.halfmove_clock));
        fen.push(' ');
        fen.push_str(&format!("{}", self.fullmove_number));

        fen
    }
}

#[cfg(test)]
mod tests {

    use crate::fen::Fen;
    use crate::position::Position;

    fn test_starting_fen() {
        let starting_pos = Position::default();

        assert_eq!(
            starting_pos.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }
}
