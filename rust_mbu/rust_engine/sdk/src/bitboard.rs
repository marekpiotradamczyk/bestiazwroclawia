#![allow(clippy::cast_possible_truncation)]
use std::{
    fmt::{Debug, Display},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Mul,
        MulAssign, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

use colored::Colorize;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::square::{File, Square};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bitboard(pub u64);

pub const EMPTY: Bitboard = Bitboard(0);

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Direction {
    #[num_enum(default)]
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Bitboard {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn empty() -> Bitboard {
        Bitboard(0)
    }

    #[must_use]
    pub fn lsb(&self) -> Square {
        (self.0.trailing_zeros() as u8)
            .try_into()
            .expect("BUG: LSB couldn't be computed from bitboard.")
    }

    #[must_use]
    pub fn msb(&self) -> Square {
        (63 - self.0.leading_zeros() as u8)
            .try_into()
            .expect("BUG: MSB couldn't be computed from bitboard.")
    }

    pub fn pop_lsb(&mut self) -> Square {
        let square = self.lsb();
        self.0 &= self.0 - 1;

        square
    }

    pub fn pop_msb(&mut self) -> Square {
        let square = self.msb();
        self.0 &= self.0 - 1;

        square
    }

    #[must_use]
    pub fn has(&self, square: Square) -> bool {
        self.0 & square.bitboard().0 != 0
    }

    #[must_use]
    pub fn count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    #[must_use]
    pub fn shift(&self, direction: &Direction) -> Bitboard {
        const FILE_A: Bitboard = File::A.bitboard();
        const FILE_H: Bitboard = File::H.bitboard();

        match direction {
            Direction::North => Bitboard(self.0 << 8),
            Direction::South => Bitboard(self.0 >> 8),
            Direction::West => Bitboard((self.0 & !FILE_A.0) >> 1),
            Direction::East => Bitboard((self.0 & !FILE_H.0) << 1),
            Direction::NorthEast => Bitboard((self.0 & !FILE_H.0) << 9),
            Direction::NorthWest => Bitboard((self.0 & !FILE_A.0) << 7),
            Direction::SouthEast => Bitboard((self.0 & !FILE_H.0) >> 7),
            Direction::SouthWest => Bitboard((self.0 & !FILE_A.0) >> 9),
        }
    }

    #[must_use]
    pub fn subsets(&self) -> SubsetIterator {
        SubsetIterator {
            subset: Bitboard(0),
            set: *self,
            finished: false,
        }
    }
}

pub struct SubsetIterator {
    subset: Bitboard,
    set: Bitboard,
    finished: bool,
}

impl Iterator for SubsetIterator {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Bitboard> {
        let curr = self.subset;
        self.subset = Bitboard(self.subset.wrapping_sub(*self.set)) & self.set;

        if curr.0 == 0 && self.finished {
            None
        } else {
            if self.subset.0 == 0 {
                self.finished = true;
            }
            Some(curr)
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BoardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator {
            bitboard: self,
            index: Square::A1,
        }
    }
}

pub struct BoardIterator {
    bitboard: Bitboard,
    index: Square,
}

impl Iterator for BoardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.bitboard.0 == 0 {
            None
        } else {
            self.index = self.bitboard.pop_lsb();
            Some(self.index)
        }
    }
}

// DISPLAY

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            return writeln!(f, "Empty bitboard.");
        }
        writeln!(f)?;
        let zero = "0".green();
        let one = "1".yellow();
        for rank in (0..8).rev() {
            write!(f, "{}   ", rank + 1)?;
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.0 & (1 << square) == 0 {
                    write!(f, "{zero}  ")?;
                } else {
                    write!(f, "{one}  ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        writeln!(f, "    a  b  c  d  e  f  g  h")?;
        writeln!(f)?;
        writeln!(f, "hex: 0x{:#X}", self.0)?;
        writeln!(f, "lsb: {}", self.lsb() as usize)?;
        writeln!(f, "msb: {}", self.msb() as usize)?;
        Ok(())
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

// CONVERSIONS

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        value.bitboard()
    }
}

// OPERATORS

macro_rules! impl_math_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait<Bitboard> for Bitboard {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                Self($trait::$fn(self.0, other.0))
            }
        }

        impl $trait<Bitboard> for &Bitboard {
            type Output = Bitboard;

            fn $fn(self, other: Bitboard) -> Self::Output {
                Bitboard($trait::$fn(self.0, other.0))
            }
        }

        impl $trait<Square> for Bitboard {
            type Output = Self;

            fn $fn(self, other: Square) -> Self::Output {
                $trait::$fn(self, other.bitboard())
            }
        }

        impl $trait<u64> for Bitboard {
            type Output = Self;

            fn $fn(self, other: u64) -> Self::Output {
                Self($trait::$fn(self.0, other))
            }
        }
    )*};
}
impl_math_ops! {
    Add, add;
    Sub, sub;
    Mul, mul;
    BitAnd, bitand;
    BitOr, bitor;
    Shr, shr;
    Shl, shl;
    BitXor, bitxor;
}

macro_rules! impl_math_assign_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait for Bitboard {
            fn $fn(&mut self, other: Self) {
                $trait::$fn(&mut self.0, other.0)
            }
        }
    )*};
}
impl_math_assign_ops! {
    BitAndAssign, bitand_assign;
    BitOrAssign, bitor_assign;
    BitXorAssign, bitxor_assign;
    AddAssign, add_assign;
    SubAssign, sub_assign;
    ShlAssign, shl_assign;
    ShrAssign, shr_assign;
    MulAssign, mul_assign;
}

impl Deref for Bitboard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}
