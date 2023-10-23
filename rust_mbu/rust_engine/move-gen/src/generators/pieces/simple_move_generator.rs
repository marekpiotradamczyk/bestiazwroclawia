use sdk::{bitboard::Bitboard, lookup::sliders::Slider, position::Color, square::Square};

use crate::{lookup::MagicEntry, generators::movegen::MoveGen};

pub trait SimpleMoveGenerator {
    fn knight_attacks(&self, square: Square) -> Bitboard;
    fn king_attacks(&self, square: Square) -> Bitboard;
    fn pawn_attacks(&self, color: Color, square: Square) -> Bitboard;
    fn pawn_single_moves(&self, color: Color, square: Square) -> Bitboard;
    fn pawn_double_moves(&self, color: Color, square: Square) -> Bitboard;
    fn rook_moves(&self, square: Square, blockers: Bitboard) -> Bitboard;
    fn bishop_moves(&self, square: Square, blockers: Bitboard) -> Bitboard;
    fn queen_moves(&self, square: Square, blockers: Bitboard) -> Bitboard;
    fn slider_moves(&self, slider: Slider, square: Square, blockers: Bitboard) -> Bitboard;
}

impl SimpleMoveGenerator for MoveGen {
    fn knight_attacks(&self, square: Square) -> Bitboard {
        self.lookups.knight_attacks[square as usize]
    }

    fn king_attacks(&self, square: Square) -> Bitboard {
        self.lookups.king_attacks[square as usize]
    }

    fn pawn_attacks(&self, color: Color, square: Square) -> Bitboard {
        self.lookups.pawn_attacks[color as usize][square as usize]
    }

    fn pawn_single_moves(&self, color: Color, square: Square) -> Bitboard {
        self.lookups.pawn_single_moves[color as usize][square as usize]
    }

    fn pawn_double_moves(&self, color: Color, square: Square) -> Bitboard {
        self.lookups.pawn_double_moves[color as usize][square as usize]
    }

    fn rook_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let entry = self.lookups.rook_magics[square as usize];
        self.lookups.rook_moves[square as usize][magic_index(&entry, blockers)]
    }

    fn bishop_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let entry = self.lookups.bishop_magics[square as usize];
        self.lookups.bishop_moves[square as usize][magic_index(&entry, blockers)]
    }

    fn queen_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        self.rook_moves(square, blockers) | self.bishop_moves(square, blockers)
    }

    fn slider_moves(&self, slider: Slider, square: Square, blockers: Bitboard) -> Bitboard {
        match slider {
            Slider::Rook => self.rook_moves(square, blockers),
            Slider::Bishop => self.bishop_moves(square, blockers),
            Slider::Queen => self.queen_moves(square, blockers),
        }
    }
}

fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
    let blockers = blockers & entry.mask;
    let hash = blockers.0.wrapping_mul(entry.magic);
    (hash >> (64 - entry.index_bits)) as usize
}
