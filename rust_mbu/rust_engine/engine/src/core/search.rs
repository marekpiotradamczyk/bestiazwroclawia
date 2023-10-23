use itertools::Itertools;
use move_gen::r#move::{MakeMove, Move};
use sdk::position::{Color, Position};

use super::{evaluate::Evaluate, Engine};

pub trait Search {
    fn search(&mut self, position: &Position, depth: usize) -> Option<(f64, Move)>;
}

impl Search for Engine {
    fn search(&mut self, position: &Position, depth: usize) -> Option<(f64, Move)> {
        let maximizing_player = position.turn == Color::White;
        let (score, mv) = minmax(self, position, depth, maximizing_player);

        Some((score, mv?))
    }
}

fn minmax(
    engine: &mut Engine,
    position: &Position,
    depth: usize,
    maximizing_player: bool,
) -> (f64, Option<Move>) {
    if depth == 0 {
        engine.nodes_evaluated += 1;
        return (engine.evaluate(position), None);
    }

    let moves = engine.move_gen.generate_legal_moves(position).collect_vec();

    if position.halfmove_clock >= 100 {
        return (0.0, None);
    }

    if moves.is_empty() {
        if engine.move_gen.is_check(position) {
            return (if maximizing_player { -1000.0 } else { 1000.0 }, None);
        } else {
            return (0.0, None);
        }
    }

    if maximizing_player {
        let mut best_move = None;
        let mut best_score = f64::MIN;

        for mv in moves {
            let mut pos = position.clone();
            engine.move_list.push(mv.clone());

            pos.make_move(&mv).unwrap();
            let (score, _) = if engine.move_list.count_occurrences(&mv) >= 2 {
                (0.0, None)
            } else {
                minmax(engine, &pos, depth - 1, false)
            };
            engine.move_list.pop();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        (best_score, best_move)
    } else {
        let mut best_move = None;
        let mut best_score = f64::MAX;

        for mv in moves {
            let mut pos = position.clone();
            engine.move_list.push(mv.clone());

            pos.make_move(&mv).unwrap();
            let (score, _) = if engine.move_list.count_occurrences(&mv) >= 2 {
                (0.0, None)
            } else {
                minmax(engine, &pos, depth - 1, true)
            };
            engine.move_list.pop();

            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        (best_score, best_move)
    }
}
