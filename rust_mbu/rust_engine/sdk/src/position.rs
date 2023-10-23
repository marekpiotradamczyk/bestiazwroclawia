use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use derivative::Derivative;

use crate::{
    bitboard::Bitboard,
    fen::Fen,
    square::Square,
};

#[derive(Derivative)]
#[derive(Debug, Clone)]
#[derivative(Hash)]
pub struct Position {
    pub pieces: [[Bitboard; 6]; 2],
    pub occupied: Bitboard,
    pub turn: Color,
    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    pub castling: Castling,
    pub en_passant: Option<Square>,
    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    pub halfmove_clock: u16,
    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    pub fullmove_number: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Hash)]
pub struct Castling {
    inner: u8,
}

pub enum CastlingKind {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

impl FromStr for Castling {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            return Ok(Castling { inner: 0 });
        }

        let mut inner = 0u8;

        for c in s.chars() {
            match c {
                'K' => inner |= 0b1000,
                'Q' => inner |= 0b0100,
                'k' => inner |= 0b0010,
                'q' => inner |= 0b0001,
                _ => return Err(anyhow!("Invalid castling string: {}", s)),
            }
        }

        Ok(Castling { inner })
    }
}

impl From<Square> for CastlingKind {
    fn from(value: Square) -> Self {
        match value {
            Square::G1 => CastlingKind::WhiteKingside,
            Square::C1 => CastlingKind::WhiteQueenside,
            Square::G8 => CastlingKind::BlackKingside,
            Square::C8 => CastlingKind::BlackQueenside,
            _ => panic!("Invalid castling square: {value}"),
        }
    }
}

impl CastlingKind {
    /// Returns tuple of `(rook_target_square, king_target_square)`.
    #[must_use]
    pub fn target_squares(&self) -> (Square, Square) {
        match self {
            CastlingKind::WhiteKingside => (Square::E1, Square::G1),
            CastlingKind::WhiteQueenside => (Square::E1, Square::C1),
            CastlingKind::BlackKingside => (Square::E8, Square::G8),
            CastlingKind::BlackQueenside => (Square::E8, Square::C8),
        }
    }

    /// Returns tuple of `(rook_from_square, king_from_square)`.
    #[must_use]
    pub fn from_squares(&self) -> (Square, Square) {
        match self {
            CastlingKind::WhiteKingside => (Square::H1, Square::E1),
            CastlingKind::WhiteQueenside => (Square::A1, Square::E1),
            CastlingKind::BlackKingside => (Square::H8, Square::E8),
            CastlingKind::BlackQueenside => (Square::A8, Square::E8),
        }
    }
}

impl Castling {
    #[must_use]
    pub fn full() -> Castling {
        Castling { inner: 0b1111 }
    }

    #[must_use]
    pub fn empty() -> Castling {
        Castling { inner: 0 }
    }

    #[must_use]
    pub fn has_castling_kind(&self, castling_kind: &CastlingKind) -> bool {
        match castling_kind {
            CastlingKind::WhiteKingside => self.inner & 0b1000 != 0,
            CastlingKind::WhiteQueenside => self.inner & 0b0100 != 0,
            CastlingKind::BlackKingside => self.inner & 0b0010 != 0,
            CastlingKind::BlackQueenside => self.inner & 0b0001 != 0,
        }
    }

    pub fn remove_color_castling(&mut self, color: &Color) {
        match color {
            Color::White => self.inner &= 0b0011,
            Color::Black => self.inner &= 0b1100,
        }
    }

    pub fn remove_castling_kind(&mut self, castling_kind: &CastlingKind) {
        match castling_kind {
            CastlingKind::WhiteKingside => self.inner &= 0b0111,
            CastlingKind::WhiteQueenside => self.inner &= 0b1011,
            CastlingKind::BlackKingside => self.inner &= 0b1101,
            CastlingKind::BlackQueenside => self.inner &= 0b1110,
        }
    }

    pub fn add_castling_kind(&mut self, castling_kind: &CastlingKind) {
        match castling_kind {
            CastlingKind::WhiteKingside => self.inner |= 0b1000,
            CastlingKind::WhiteQueenside => self.inner |= 0b0100,
            CastlingKind::BlackKingside => self.inner |= 0b0010,
            CastlingKind::BlackQueenside => self.inner |= 0b0001,
        }
    }
}

impl Position {
    #[must_use]
    pub fn occupation(&self, color: &Color) -> Bitboard {
        self.pieces[*color as usize]
            .iter()
            .fold(Bitboard(0), |acc, x| acc | *x)
    }

    #[must_use]
    pub fn enemy(&self) -> Color {
        match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[must_use]
    pub fn swap_turn(&mut self) -> Color {
        match self.turn {
            Color::White => self.turn = Color::Black,
            Color::Black => self.turn = Color::White,
        }

        self.turn
    }

    pub fn remove_piece_at(&mut self, square: &Square) -> Option<(Piece, Color)> {
        let (piece, color) = self.piece_at(square)?;

        self.pieces[color as usize][piece as usize] ^= square.bitboard();

        Some((piece, color))
    }

    pub fn add_piece_at(
        &mut self,
        square: Square,
        piece: Piece,
        color: Color,
    ) -> Result<(), anyhow::Error> {
        if self.piece_at(&square).is_some() {
            return Err(anyhow!("Piece already at {}", square.coords_str()));
        }
        self.pieces[color as usize][piece as usize] |= Into::<Bitboard>::into(square);

        Ok(())
    }

    #[must_use]
    pub fn piece_at(&self, square: &Square) -> Option<(Piece, Color)> {
        let square = square.bitboard();
        for color in [Color::White, Color::Black] {
            for (i, piece_bb) in self.pieces[color as usize].iter().enumerate() {
                if !((piece_bb & square).is_empty()) {
                    return Some((Piece::from(i), color));
                }
            }
        }

        None
    }
}

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("Invalid piece index"),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
            .expect("Invalid starting FEN.")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "w"),
            Color::Black => write!(f, "b"),
        }
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_castling_kind(&CastlingKind::WhiteKingside) {
            write!(f, "K")?;
        }

        if self.has_castling_kind(&CastlingKind::WhiteQueenside) {
            write!(f, "Q")?;
        }

        if self.has_castling_kind(&CastlingKind::BlackKingside) {
            write!(f, "k")?;
        }

        if self.has_castling_kind(&CastlingKind::BlackQueenside) {
            write!(f, "q")?;
        }

        Ok(())
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Pawn => write!(f, "p"),
            Piece::Knight => write!(f, "n"),
            Piece::Bishop => write!(f, "b"),
            Piece::Rook => write!(f, "r"),
            Piece::Queen => write!(f, "q"),
            Piece::King => write!(f, "k"),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for rank in (0..8u8).rev() {
            for file in 0..8u8 {
                let square: Square = (rank * 8 + file)
                    .try_into()
                    .expect("BUG: Square out of bounds");
                if let Some((piece, color)) = self.piece_at(&square) {
                    write!(f, "{} ", piece.to_utf8_symbol(color))?;
                } else {
                    write!(f, "x ")?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "Turn: {}", self.turn)?;
        writeln!(f, "Castling: {}", self.castling)?;
        if let Some(en_passant) = self.en_passant {
            writeln!(f, "En passant: {en_passant}")?;
        }

        Ok(())
    }
}

impl Piece {
    #[must_use]
    pub const fn all() -> [Piece; 6] {
        [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ]
    }

    #[must_use]
    pub fn to_utf8_symbol(&self, color: Color) -> &'static str {
        match (self, color) {
            (Piece::Pawn, Color::Black) => "♙",
            (Piece::Pawn, Color::White) => "♟",
            (Piece::Knight, Color::Black) => "♘",
            (Piece::Knight, Color::White) => "♞",
            (Piece::Bishop, Color::Black) => "♗",
            (Piece::Bishop, Color::White) => "♝",
            (Piece::Rook, Color::Black) => "♖",
            (Piece::Rook, Color::White) => "♜",
            (Piece::Queen, Color::Black) => "♕",
            (Piece::Queen, Color::White) => "♛",
            (Piece::King, Color::Black) => "♔",
            (Piece::King, Color::White) => "♚",
        }
    }
}

impl From<String> for Piece {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "p" => Piece::Pawn,
            "n" => Piece::Knight,
            "b" => Piece::Bishop,
            "r" => Piece::Rook,
            "q" => Piece::Queen,
            "k" => Piece::King,
            _ => panic!("Invalid piece symbol"),
        }
    }
}

pub struct ColorIterator {
    idx: i8,
}

impl Iterator for ColorIterator {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        match self.idx - 1 {
            0 => Some(Color::White),
            1 => Some(Color::Black),
            _ => None,
        }
    }
}

impl Color {
    #[must_use]
    pub fn iter() -> ColorIterator {
        ColorIterator { idx: 0 }
    }

    #[must_use]
    pub fn enemy(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
