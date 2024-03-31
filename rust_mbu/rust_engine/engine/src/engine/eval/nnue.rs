use std::{collections::HashMap, fs, path::Path};

use lazy_static::lazy_static;
use sdk::{
    position::{CastlingKind, Color, Piece, Position},
    square::Square,
};
use serde::Deserialize;

lazy_static! {
    pub static ref NEURAL_NETWORK: NNUE = NNUE::new();
}

#[derive(Debug)]
pub struct Weights {
    pieces: [[[f64; 64]; 6]; 2],
    castling: [f64; 4],
}

pub struct NNUE {
    pub weights: [Weights; 2],
}

impl NNUE {
    pub fn new() -> NNUE {
        let white_weights = Weights::load("w2move_weights.json".to_string());
        let black_weights = Weights::load("b2move_weights.json".to_string());

        NNUE {
            weights: [white_weights, black_weights],
        }
    }

    pub fn evaluate(&self, pos: &Position) -> i32 {
        let raw = self.raw_eval(pos);

        println!("Raw sum: {raw}");

        let result = (sigmoid(raw / 100.0) - 0.5) * 100.0;

        result as i32
    }

    fn raw_eval(&self, pos: &Position) -> f64 {
        let mut score = 0.0;

        let mut sq = 0;
        while sq < 64 {
            let square = Square::from_u8(sq as u8);
            let piece = pos.piece_at(&square);

            if let Some((piece, color)) = piece {
                score += self.weights[pos.turn as usize].pieces[color as usize][piece as usize][sq];
            }

            sq += 1;
        }

        for castling in pos.castling.all() {
            score += self.weights[pos.turn as usize].castling[castling as usize];
        }

        score
    }
}

pub fn inverse_sigmoid(x: f64) -> f64 {
    -f64::ln((1.0 - x) / x) / 100.0
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x * 100.0))
}

//

#[derive(Deserialize)]
struct RawWeights {
    weight: HashMap<String, f64>,
}

impl Weights {
    pub fn load(file_name: String) -> Weights {
        let path = Path::new("./nets").join(file_name);

        let raw_weights: RawWeights =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        let mut weights = Weights::default();

        for (key, value) in raw_weights.weight.iter() {
            match key.as_str() {
                "K" => weights.castling[CastlingKind::WhiteKingside as usize] = *value,
                "Q" => weights.castling[CastlingKind::WhiteQueenside as usize] = *value,
                "k" => weights.castling[CastlingKind::BlackKingside as usize] = *value,
                "q" => weights.castling[CastlingKind::BlackQueenside as usize] = *value,
                entry => {
                    assert!(entry.len() == 3);
                    let file = entry.chars().nth(0).unwrap() as usize - 'a' as usize;
                    let rank = entry.chars().nth(1).unwrap() as usize - '1' as usize;
                    let square = rank * 8 + file;
                    let piece_str = entry.chars().nth(2).unwrap().to_string();
                    let (piece, color) = match piece_str.as_str() {
                        "p" => (Piece::Pawn, Color::Black),
                        "P" => (Piece::Pawn, Color::White),
                        "n" => (Piece::Knight, Color::Black),
                        "N" => (Piece::Knight, Color::White),
                        "b" => (Piece::Bishop, Color::Black),
                        "B" => (Piece::Bishop, Color::White),
                        "r" => (Piece::Rook, Color::Black),
                        "R" => (Piece::Rook, Color::White),
                        "q" => (Piece::Queen, Color::Black),
                        "Q" => (Piece::Queen, Color::White),
                        "k" => (Piece::King, Color::Black),
                        "K" => (Piece::King, Color::White),
                        _ => panic!("Invalid piece"),
                    };

                    weights.pieces[color as usize][piece as usize][square] = *value;
                }
            }
        }

        weights
    }
}

impl Default for Weights {
    fn default() -> Self {
        Weights {
            pieces: [[[0.0; 64]; 6]; 2],
            castling: [0.0; 4],
        }
    }
}
