use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use sdk::{
    bitboard::Bitboard, fen::Fen, position::{Color, Piece, Position}
};

const ENTRY_SIZE: usize = 836;

#[derive(Debug)]
pub struct TrainDataEntry {
    pub position: [u8; ENTRY_SIZE],
    pub eval: f64,
}

pub fn parse_data(file: File) -> impl Iterator<Item = TrainDataEntry> {
    let reader = BufReader::new(file);

    reader.lines().map(|line| {
        let line = line.unwrap();
        let mut split = line.split(',');
        let position = Position::from_fen(split.next().unwrap().to_string())
            .unwrap()
            .to_bytes();
        let eval: f64 = split.next().unwrap().parse().unwrap();

        TrainDataEntry { position, eval }
    })
}

trait ToBytes {
    fn to_bytes(&self) -> [u8; ENTRY_SIZE];
}

impl ToBytes for Position {
    fn to_bytes(&self) -> [u8; ENTRY_SIZE] {
        let mut bytes = [0; ENTRY_SIZE];

        let mut idx = 0;
        for color in Color::all() {
            for piece in Piece::all() {
                let bb = self.pieces[color as usize][piece as usize].0;

                for bb_idx in 0..64 {
                    bytes[idx] = (bb >> bb_idx) as u8 & 1;
                    idx += 1;
                }
            }
        }

        // En Passant Square
        let ep_square = self.en_passant.map(|sq| sq.bitboard()).unwrap_or(Bitboard::empty());
        for bb_idx in 0..64 {
            bytes[idx] = (ep_square.0 >> bb_idx) as u8 & 1;
            idx += 1;
        }

        // Castling Rights
        let cr = self.castling.inner & 0b00001111;
        for bb_idx in 0..4 {
            bytes[idx] = (cr >> bb_idx) & 1;
            idx += 1;
        }

        bytes
    }
}
