use std::fmt::{Display, Formatter};

use crate::bitboard::Bitboard;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub const FILE_MASKS: [Bitboard; 8] = [
    Bitboard(0x0101_0101_0101_0101),
    Bitboard(0x0202_0202_0202_0202),
    Bitboard(0x0404_0404_0404_0404),
    Bitboard(0x0808_0808_0808_0808),
    Bitboard(0x1010_1010_1010_1010),
    Bitboard(0x2020_2020_2020_2020),
    Bitboard(0x4040_4040_4040_4040),
    Bitboard(0x8080_8080_8080_8080),
];

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        };

        write!(f, "{c}")
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Rank::R1 => '1',
            Rank::R2 => '2',
            Rank::R3 => '3',
            Rank::R4 => '4',
            Rank::R5 => '5',
            Rank::R6 => '6',
            Rank::R7 => '7',
            Rank::R8 => '8',
        };

        write!(f, "{c}")
    }
}

impl File {
    #[must_use]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(0x0101_0101_0101_0101 << self as u8)
    }

    #[must_use]
    pub const fn from_u8(file: u8) -> File {
        match file {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid file index"),
        }
    }
}

impl Rank {
    #[must_use]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(0xFF << (self as u8 * 8))
    }

    #[must_use]
    pub const fn from_u8(rank: u8) -> Rank {
        match rank {
            0 => Rank::R1,
            1 => Rank::R2,
            2 => Rank::R3,
            3 => Rank::R4,
            4 => Rank::R5,
            5 => Rank::R6,
            6 => Rank::R7,
            7 => Rank::R8,
            _ => panic!("Invalid rank index"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
#[repr(u8)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    #[must_use]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(1 << self as usize)
    }

    #[must_use]
    pub const fn rank(&self) -> Rank {
        let idx = *self as u8 >> 3;

        Rank::from_u8(idx)
    }

    #[must_use]
    pub const fn file(&self) -> File {
        let idx = *self as u8 & 0b00_0111;

        File::from_u8(idx)
    }

    #[allow(clippy::cast_sign_loss)]
    #[must_use]
    pub const fn offset(&self, rank_offset: i8, file_offset: i8) -> Option<Square> {
        let (file, rank): (File, Rank) = self.to_file_rank();

        let file_idx = file as i8 + file_offset;
        if file_idx > 7 || file_idx < 0 {
            return None;
        }

        let rank_idx = rank as i8 + rank_offset;
        if rank_idx > 7 || rank_idx < 0 {
            return None;
        }

        let file = File::from_u8(file_idx as u8);
        let rank = Rank::from_u8(rank_idx as u8);

        Some(Square::from_file_rank(file, rank))
    }

    #[must_use]
    pub const fn from_file_rank(file: File, rank: Rank) -> Square {
        let idx = (rank as u8 * 8) + file as u8;

        Square::from_u8(idx)
    }

    #[must_use]
    pub const fn to_file_rank(&self) -> (File, Rank) {
        let file = self.file();
        let rank = self.rank();

        (file, rank)
    }

    #[must_use]
    pub fn iter() -> SqIter {
        SqIter { idx: 0 }
    }

    #[must_use]
    pub fn coords_str(&self) -> String {
        let square = *self as u8;
        let file = (square % 8) + b'a';
        let rank = (square / 8) + b'1';
        let file_str = char::from(file);
        let rank_str = char::from(rank);
        format!("{file_str}{rank_str}")
    }

    #[must_use]
    #[inline]
    pub const fn from_u8(square: u8) -> Square {
        unsafe { std::mem::transmute(square) }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.coords_str())
    }
}

pub struct SqIter {
    idx: u8,
}

impl Iterator for SqIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 64 {
            return None;
        }

        let sq = Square::from_u8(self.idx);
        self.idx += 1;

        Some(sq)
    }
}
