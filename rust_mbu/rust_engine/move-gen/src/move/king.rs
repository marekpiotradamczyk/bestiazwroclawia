use arrayvec::ArrayVec;
use sdk::bitboard::Direction;
use sdk::lookup::sliders::Slider;
use sdk::position::{CastlingKind, Color};
use sdk::square::{File, Rank, Square};
use sdk::{bitboard::Bitboard, position::Piece};

use super::move_list::{CachedPosInfo, MoveList, MoveType, MOVE_GEN};
use super::{Move, MoveKind};
use crate::generators::pieces::simple_move_generator::SimpleMoveGenerator;
use crate::generators::pieces::PinnerGenerator;

impl<'a> MoveList<'a> {
    pub(crate) fn generate_king_moves(&mut self, move_type: MoveType) {
        let CachedPosInfo {
            pinned,
            enemy_occ,
            friendly_occ,
            color,
            ..
        } = self.cached;
        let pos = self.pos;

        let bb = pos.pieces[color as usize][Piece::King as usize] & !pinned;

        bb.into_iter().for_each(|from_square| {
            let attacks = match move_type {
                MoveType::Capture => MOVE_GEN.king_attacks(from_square) & !friendly_occ & enemy_occ,
                MoveType::NonCapture => {
                    MOVE_GEN.king_attacks(from_square) & !friendly_occ & !enemy_occ
                }
            };

            attacks.into_iter().for_each(|target_square| {
                // Move is illegal if it leaves the king in check
                if MOVE_GEN
                    .attacks_to_square(pos, target_square, pos.enemy(), pos.occupied & !bb)
                    .is_empty()
                {
                    let kind = match move_type {
                        MoveType::Capture => MoveKind::Capture,
                        MoveType::NonCapture => MoveKind::Quiet,
                    };

                    let mv = Move::new(from_square, target_square, None, &kind);
                    self.moves.push(mv);
                }
            })
        });

        if matches!(move_type, MoveType::NonCapture) {
            self.generate_all_castlings();
        }
    }

    fn generate_all_castlings(&mut self) {
        let pos = self.pos;

        if !MOVE_GEN.is_check(pos) {
            for castling_kind in &[
                CastlingKind::WhiteKingside,
                CastlingKind::WhiteQueenside,
                CastlingKind::BlackKingside,
                CastlingKind::BlackQueenside,
            ] {
                if let Some(mv) = self.generate_castling(castling_kind) {
                    self.moves.push(mv);
                }
            }
        }
    }

    fn generate_castling(&self, castling_kind: &CastlingKind) -> Option<Move> {
        let CachedPosInfo {
            blockers: occ,
            king_sq: king_square,
            color,
            ..
        } = self.cached;
        let pos = self.pos;

        let castling_color = match castling_kind {
            CastlingKind::WhiteKingside | CastlingKind::WhiteQueenside => Color::White,
            CastlingKind::BlackKingside | CastlingKind::BlackQueenside => Color::Black,
        };

        if color != castling_color {
            return None;
        }

        match castling_kind {
            CastlingKind::WhiteKingside | CastlingKind::WhiteQueenside => {
                if king_square != Square::E1 {
                    return None;
                }
            }
            CastlingKind::BlackKingside | CastlingKind::BlackQueenside => {
                if king_square != Square::E8 {
                    return None;
                }
            }
        };

        let between_bb = match castling_kind {
            CastlingKind::WhiteKingside => Square::F1.bitboard() | Square::G1.bitboard(),
            CastlingKind::WhiteQueenside => {
                Square::B1.bitboard() | Square::C1.bitboard() | Square::D1.bitboard()
            }
            CastlingKind::BlackKingside => Square::F8.bitboard() | Square::G8.bitboard(),
            CastlingKind::BlackQueenside => {
                Square::B8.bitboard() | Square::C8.bitboard() | Square::D8.bitboard()
            }
        };

        if !(between_bb & occ).is_empty() {
            return None;
        }

        let target_square: Square = match castling_kind {
            CastlingKind::WhiteKingside | CastlingKind::BlackKingside => {
                Square::from_u8(king_square as u8 + 2)
            }
            CastlingKind::WhiteQueenside | CastlingKind::BlackQueenside => {
                Square::from_u8(king_square as u8 - 2)
            }
        };

        if pos.castling.has_castling_kind(castling_kind)
            && pos.turn == castling_color
            && (between_bb & !File::B.bitboard()).into_iter().all(|sq| {
                MOVE_GEN
                    .attacks_to_square(pos, sq, pos.enemy(), pos.occupied)
                    .is_empty()
            })
        {
            return Some(Move::new(
                king_square,
                target_square,
                None,
                &MoveKind::Castling,
            ));
        }

        None
    }
}
