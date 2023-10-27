use sdk::{
    bitboard::{Bitboard, Direction},
    position::Position,
    square::Square,
};

use crate::xray::XRayGenerator;

use self::simple_move_generator::SimpleMoveGenerator;

use super::movegen::MoveGen;

pub mod king_generator;
pub mod knight_generator;
pub mod pawn_generator;
pub mod simple_move_generator;
pub mod slider_generator;

pub trait PinnerGenerator {
    fn between_pinner_inclusive(&self, sq: Square, king_sq: Square, blockers: Bitboard)
        -> Bitboard;
}

pub fn ray_direction(sq: Square, target: Square) -> Option<Direction> {
    let (first_file, first_rank) = sq.into();
    let (second_file, second_rank) = target.into();

    let file_diff = first_file as i8 - second_file as i8;
    let rank_diff = first_rank as i8 - second_rank as i8;

    if file_diff == 0 {
        if rank_diff > 0 {
            Some(Direction::South)
        } else {
            Some(Direction::North)
        }
    } else if rank_diff == 0 {
        if file_diff > 0 {
            Some(Direction::West)
        } else {
            Some(Direction::East)
        }
    } else if file_diff == rank_diff {
        if file_diff > 0 {
            Some(Direction::NorthEast)
        } else {
            Some(Direction::SouthWest)
        }
    } else if file_diff == -rank_diff {
        if file_diff > 0 {
            Some(Direction::NorthWest)
        } else {
            Some(Direction::SouthEast)
        }
    } else {
        None
    }
}

impl PinnerGenerator for MoveGen {
    fn between_pinner_inclusive(
        &self,
        sq: Square,
        king_sq: Square,
        blockers: Bitboard,
    ) -> Bitboard {
        if let Some(dir) = ray_direction(king_sq, sq) {
            //TODO: Use xrays instead
            let moves = match dir {
                Direction::North | Direction::South | Direction::East | Direction::West => {
                    self.rook_moves(king_sq, blockers ^ sq.bitboard())
                }
                _ => self.bishop_moves(king_sq, blockers ^ sq.bitboard()),
            };

            moves & self.lookups.ray_attacks[dir as usize][king_sq as usize]
        } else {
            Bitboard::empty()
        }
    }
}
