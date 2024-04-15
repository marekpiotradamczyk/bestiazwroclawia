use move_gen::r#move::Move;
use sdk::position::{Color, Position};

use crate::engine::search::parallel::SearchData;

/// Less valuable victim (LVA) and more valuable victim (MVV) tables
/// Effectively this is a set of priorities for moves.
/// For example Queen capturing a pawn would score lower (101) than a pawn capturing a pawn (105).
static MVV_LVA: [[i32; 6]; 6] = [
    [105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600],
];

pub trait MoveUtils {
    fn score_move(&self, mv: &Move, pos: &Position) -> i32;
    fn order_moves(&self, moves: &mut [Move], pos: &Position, best_move: Option<Move>) {
        let mut scores = [i32::MIN; 128];
        for i in 0..moves.len() {
            let mov = &moves[i];
            let score = if best_move.as_ref().is_some_and(|best| best == mov) {
                -3_000_000
            } else {
                -self.score_move(mov, pos)
            };

            scores[i] = score;

            // Sort the moves by score
            let mut c = i;
            let mut j = i;
            loop {
                if c > 0 {
                    c -= 1;
                } else {
                    break;
                }

                if scores[j] < scores[c] {
                    unsafe {
                        std::ptr::swap(
                            std::ptr::addr_of_mut!(moves[j]),
                            std::ptr::addr_of_mut!(moves[c]),
                        );
                        std::ptr::swap(
                            std::ptr::addr_of_mut!(scores[j]),
                            std::ptr::addr_of_mut!(scores[c]),
                        );
                    };
                    j -= 1;
                } else {
                    break;
                }
            }
        }
    }
}

impl MoveUtils for SearchData {
    fn score_move(&self, mv: &Move, pos: &Position) -> i32 {
        // We prioritze captures
        if mv.is_capture() {
            let attacker = pos.piece_at(mv.from()).unwrap().0;
            let victim = if mv.is_enpass_capture() {
                let rank_offset = if pos.turn == Color::White { -1 } else { 1 };
                let sq = mv.to().offset(rank_offset, 0).unwrap();
                pos.piece_at(sq).unwrap().0
            } else {
                pos.piece_at(mv.to()).unwrap().0
            };

            return MVV_LVA[attacker as usize][victim as usize] + 1_000_000;
        }

        // Then we prioritize killer moves, that is moves that caused a beta cutoff in the past.
        if self.killer_moves[0][self.ply].is_some_and(|killer| killer == *mv) {
            500_000
        } else if self.killer_moves[1][self.ply].is_some_and(|killer| killer == *mv) {
            490_000
        } else if self.ply > 1
            && self.counter_moves[0][self.ply - 1].is_some_and(|counter| counter == *mv)
        {
            480_000
        } else if self.ply > 1
            && self.counter_moves[1][self.ply - 1].is_some_and(|counter| counter == *mv)
        {
            470_000
        } else if self.ply > 2 && self.pair_moves[0][self.ply - 2].is_some_and(|pair| pair == *mv) {
            460_000
        } else if self.ply > 2 && self.pair_moves[1][self.ply - 2].is_some_and(|pair| pair == *mv) {
            450_000
        } else {
            let (piece, color) = pos.piece_at(mv.from()).unwrap();

            self.history_moves[color as usize][piece as usize][mv.to() as usize]
        }
    }
}
