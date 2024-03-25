use core::array::from_fn;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::position::{Color, Position};

lazy_static! {
    pub static ref ZOBRIST_KEYS: ZobristKeys = ZobristKeys::default();
}

pub struct ZobristKeys {
    pub pieces: [[[u64; 64]; 6]; 2],
    pub castling_rights: [u64; 16],
    pub en_passant: [u64; 64],
    pub side_to_move: u64,
}

impl Default for ZobristKeys {
    fn default() -> Self {
        let mut rng: StdRng = SeedableRng::seed_from_u64(316_662);

        let pieces = from_fn(|_| from_fn(|_| from_fn(|_| rng.gen())));
        let castling_rights = from_fn(|_| rng.gen());
        let en_passant = from_fn(|_| rng.gen());
        let side_to_move = rng.gen();

        Self {
            pieces,
            castling_rights,
            en_passant,
            side_to_move,
        }
    }
}

impl Position {
    #[must_use]
    pub fn calc_hash(&self) -> u64 {
        let mut hash = 0;

        for color in 0..2 {
            for piece in 0..6 {
                for sq in self.pieces[color][piece] {
                    hash ^= ZOBRIST_KEYS.pieces[color][piece][sq as usize];
                }
            }
        }

        hash ^= ZOBRIST_KEYS.castling_rights[self.castling.inner as usize];
        if let Some(en_passant) = self.en_passant {
            hash ^= ZOBRIST_KEYS.en_passant[en_passant as usize];
        }
        if self.turn == Color::Black {
            hash ^= ZOBRIST_KEYS.side_to_move;
        }

        hash
    }
}
