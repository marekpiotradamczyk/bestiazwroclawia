use arrayvec::ArrayVec;
use sdk::{
    bitboard::Bitboard,
    position::{Piece, Position},
    square::Square,
};

use crate::generators::movegen::MoveGen;
use crate::generators::pieces::PinnerGenerator;

use super::{Move, MoveKind};

lazy_static::lazy_static! {
    pub static ref MOVE_GEN: MoveGen = MoveGen::default();
}

#[derive(Default)]
enum Stage {
    #[default]
    PVMove,
    Prepare,
    Captures,
}

pub struct MoveList<'a> {
    position: &'a Position,
    stage: Stage,
    pv_move: Option<Move>,
    checked_moves: ArrayVec<Move, 4>,
    cached_data: CachedPosData,
}

#[derive(Default)]
struct CachedPosData {
    pinned: Bitboard,
    enemy_pieces: Bitboard,
    our_pieces: Bitboard,
    occupied: Bitboard,
    king_square: Square,
}

impl<'a> MoveList<'a> {
    pub fn new(position: &'a Position, pv_move: Option<Move>) -> Self {
        Self {
            position,
            stage: Stage::PVMove,
            pv_move,
            checked_moves: ArrayVec::new(),
            cached_data: CachedPosData::default(),
        }
    }

    fn gen_legal_pawn_captures(&self) -> impl Iterator<Item = Move> + '_ {
        let cached = &self.cached_data;
        let color = self.position.turn;
        let pawns = self.position.pieces[color as usize][Piece::Pawn as usize];

        pawns.into_iter().flat_map(|from| {
            let maybe_pinner_ray = if cached.pinned.has(from) {
                MOVE_GEN.between_pinner_inclusive(from, cached.king_square, cached.occupied)
            } else {
                Bitboard::full()
            };

            let captures = MOVE_GEN.lookups.pawn_attacks[self.position.turn as usize]
                [from as usize]
                & self.cached_data.enemy_pieces;

            let attacks = if let Some(en_passant) = self.position.en_passant {
                MOVE_GEN.lookups.pawn_attacks[color as usize][from as usize]
                    & (cached.enemy_pieces | en_passant)
                    & maybe_pinner_ray
            } else {
                self.pawn_attacks(color, from_square) & enemy_occ & maybe_pinner_ray
            };

            captures
                .into_iter()
                .map(move |to| Move::new(from, to, None, &MoveKind::Capture))
        })
    }
}

impl<'a> Iterator for MoveList<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stage {
                Stage::PVMove => {
                    self.stage = Stage::Prepare;

                    return self.pv_move;
                }
                Stage::Prepare => {
                    self.stage = Stage::Captures;

                    self.cached_data.pinned =
                        MOVE_GEN.pinned_pieces(self.position, self.position.turn);
                    self.cached_data.enemy_pieces =
                        self.position.occupation(&self.position.enemy());
                    self.cached_data.our_pieces = self.position.occupation(&self.position.turn);
                    self.cached_data.occupied = self.position.occupied;
                    self.cached_data.king_square = self.position.pieces
                        [self.position.turn as usize][Piece::King as usize]
                        .msb();
                }
                Stage::Captures => {}
            }
        }
    }
}
