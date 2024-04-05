use core::fmt;
use std::fmt::Formatter;

use sdk::{position::{CastlingKind, Color, Piece, Position}, square::{File, Square}};

pub mod make_move;
pub mod undo_move;

pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Default)]
pub struct Move {
    pub inner: u16,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let from = self.from();
        let to = self.to();

        if let Some(promotion) = self.promotion() {
            write!(f, "from={}, to={}, promotion={}", from, to, promotion)
        } else {
            write!(f, "from={}, to={}", from, to)
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let from = self.from();
        let to = self.to();

        if let Some(promotion) = self.promotion() {
            write!(f, "{}{}{}", from, to, promotion)
        } else {
            write!(f, "{}{}", from, to)
        }
    }
}

#[derive(Debug)]
pub enum MoveKind {
    Quiet,
    DoublePawnPush,
    Capture,
    EnPassant,
    Castling,
    Promotion,
    PromotionCapture,
}

impl Move {
    pub const fn null() -> Move {
        Move { inner: 0 }
    }

    pub fn new(from: Square, to: Square, promotion: Option<Piece>, kind: &MoveKind) -> Move {
        let mut inner = 0;
        inner |= from as u16;
        inner |= (to as u16) << 6;
        let mut mv = Move { inner };

        match kind {
            MoveKind::Capture => {
                mv.set_capture();
            }
            MoveKind::EnPassant => {
                mv.set_enpass_capture();
            }
            MoveKind::Castling => {
                if to.file() == File::C {
                    mv.set_queen_castle();
                } else if to.file() == File::G {
                    mv.set_king_castle();
                }
            }
            MoveKind::Promotion => {
                mv.set_promotion(promotion.expect("BUG: No promotion piece"));
            }
            MoveKind::PromotionCapture => {
                mv.set_promotion_capture(promotion.expect("BUG: No promotion piece"));
            }
            MoveKind::DoublePawnPush => {
                mv.set_double_pawn_push();
            }
            _ => {}
        }

        mv
    }

    pub fn from(&self) -> Square {
        Square::try_from((self.inner & 0b0000000000111111) as u8).expect("Invalid square")
    }

    pub fn to(&self) -> Square {
        Square::try_from(((self.inner & 0b0000111111000000) >> 6) as u8).expect("Invalid square")
    }

    pub fn kind(&self) -> MoveKind {
        if self.promotion().is_some() {
            if self.is_capture() {
                MoveKind::PromotionCapture
            } else {
                MoveKind::Promotion
            }
        } else if self.is_enpass_capture() {
            MoveKind::EnPassant
        } else if self.is_capture() {
            MoveKind::Capture
        } else if self.is_king_castle() || self.is_queen_castle() {
            MoveKind::Castling
        } else if self.is_double_pawn_push() {
            MoveKind::DoublePawnPush
        } else {
            MoveKind::Quiet
        }
    }

    pub fn promotion(&self) -> Option<Piece> {
        match self.inner & 0b1011000000000000 {
            0b1000000000000000 => Some(Piece::Knight),
            0b1001000000000000 => Some(Piece::Bishop),
            0b1010000000000000 => Some(Piece::Rook),
            0b1011000000000000 => Some(Piece::Queen),
            _ => None,
        }
    }

    pub fn is_capture(&self) -> bool {
        self.inner & 0b0100000000000000 != 0
    }

    pub fn is_enpass_capture(&self) -> bool {
        self.inner & 0b1111000000000000 == 0b0101000000000000
    }

    pub fn is_quiet(&self) -> bool {
        self.inner & 0b1111000000000000 == 0
    }

    pub fn is_double_pawn_push(&self) -> bool {
        self.inner & 0b0001000000000000 != 0
    }

    pub fn is_king_castle(&self) -> bool {
        self.inner & 0b1111000000000000 == 0b0010000000000000
    }

    pub fn is_queen_castle(&self) -> bool {
        self.inner & 0b1111000000000000 == 0b0011000000000000
    }

    pub fn castling_kind(&self, color: &Color) -> Option<CastlingKind> {
        if self.is_queen_castle() {
            return Some(match color {
                Color::White => CastlingKind::WhiteQueenside,
                Color::Black => CastlingKind::BlackQueenside,
            });
        }

        if self.is_king_castle() {
            return Some(match color {
                Color::White => CastlingKind::WhiteKingside,
                Color::Black => CastlingKind::BlackKingside,
            });
        }

        None
    }

    pub fn is_irreversible(&self, pos: &Position) -> bool {
        if matches!(self.kind(), MoveKind::Capture | MoveKind::PromotionCapture) {
            return true;
        }

        let (piece, _) = pos.piece_at(&self.from()).unwrap();

        piece == Piece::Pawn
    }

    fn set_promotion(&mut self, promotion: Piece) {
        self.inner |= match promotion {
            Piece::Knight => 0b1000000000000000,
            Piece::Bishop => 0b1001000000000000,
            Piece::Rook => 0b1010000000000000,
            Piece::Queen => 0b1011000000000000,
            _ => panic!("Invalid promotion: {promotion}"),
        }
    }

    fn set_capture(&mut self) {
        self.inner |= 0b0100000000000000;
    }

    fn set_enpass_capture(&mut self) {
        self.inner |= 0b0101000000000000;
    }

    fn set_promotion_capture(&mut self, promotion: Piece) {
        self.set_promotion(promotion);
        self.inner |= 0b0100000000000000;
    }

    fn set_king_castle(&mut self) {
        self.inner |= 0b0010000000000000;
    }

    fn set_queen_castle(&mut self) {
        self.inner |= 0b0011000000000000;
    }

    fn set_double_pawn_push(&mut self) {
        self.inner |= 0b0001000000000000;
    }
}
