pub mod random;

use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use random::random_bitboard;
use sdk::{bitboard::Bitboard, lookup::sliders::Slider, square::Square};

fn main() {
    let path = Path::new(".");
    let rook_magic_path = path.join("rook_magics.bin");
    let bishop_magic_path = path.join("bishop_magics.bin");

    println!("Generating magics for rooks...");
    save_magics(Slider::Rook, rook_magic_path).unwrap();
    println!("Generating magics for bishops...");
    save_magics(Slider::Bishop, bishop_magic_path).unwrap();
    println!("Done!");
}

#[derive(Debug, Clone, Copy)]
struct MagicEntry {
    mask: Bitboard,
    magic: u64,
    index_bits: u8,
}

impl Default for MagicEntry {
    fn default() -> Self {
        Self {
            mask: Bitboard::empty(),
            magic: 0,
            index_bits: 0,
        }
    }
}

fn save_magics(slider: Slider, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let (magics, moves) = generate_magics(slider);
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

    for magic in magics.iter() {
        file.write_all(&magic.mask.0.to_be_bytes())?;
        file.write_all(&magic.magic.to_be_bytes())?;
        file.write_all(&magic.index_bits.to_be_bytes())?;
    }

    for move_set in moves.iter() {
        file.write_all(&move_set.0.to_be_bytes())?;
    }

    Ok(())
}

fn generate_magics(slider: Slider) -> ([MagicEntry; 64], Vec<Bitboard>) {
    let index_bits = match slider {
        Slider::Bishop => 9,
        Slider::Rook => 12,
        Slider::Queen => unreachable!(),
    };
    let slider_name = slider.to_string().to_lowercase();
    let mut magics = [MagicEntry::default(); 64];
    let mut moves: Vec<Bitboard> = Vec::new();
    for sq in Square::iter() {
        let (magic_entry, table) = find_magic(&slider, sq, index_bits);
        println!(
            "Generated table for {} of size: {}",
            slider_name,
            table.len()
        );
        magics[sq as usize] = magic_entry;
        moves.extend(table.clone());
    }

    println!("Generated {} {} moves.", moves.len(), slider_name);
    (magics, moves)
}

fn find_magic(slider: &Slider, sq: Square, index_bits: u8) -> (MagicEntry, Vec<Bitboard>) {
    let mask = slider.relevant_blockers(sq);

    loop {
        let magic_number = {
            let mut magic = random_bitboard();

            for _ in 0..2 {
                magic &= random_bitboard()
            }

            magic
        };

        let magic_entry = MagicEntry {
            mask,
            magic: *magic_number,
            index_bits,
        };

        if let Ok(table) = try_make_table(slider, sq, &magic_entry) {
            return (magic_entry, table);
        }
    }
}

fn try_make_table(
    slider: &Slider,
    square: Square,
    magic_entry: &MagicEntry,
) -> Result<Vec<Bitboard>, ()> {
    let mut table = vec![Bitboard::empty(); 1 << magic_entry.index_bits];
    // Iterate all configurations of blockers
    for blockers in magic_entry.mask.subsets() {
        let moves = slider.moves(square, blockers);
        let idx = magic_index(magic_entry, blockers);
        if table[idx].is_empty() {
            table[idx] = moves;
        } else if table[idx] != moves {
            // Having two different move sets in the same slot is a hash collision
            return Err(());
        }
    }

    Ok(table)
}

fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
    let blockers = blockers & entry.mask;
    let hash = blockers.0.wrapping_mul(entry.magic);
    (hash >> (64 - entry.index_bits)) as usize
}
