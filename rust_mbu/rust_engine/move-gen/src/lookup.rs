use std::{io::Read, path::Path};

use sdk::{
    bitboard::Bitboard,
    lookup::{
        in_between::generate_in_between_squares,
        king::gen_king_attacks,
        knights::gen_knight_attacks,
        pawns::{gen_double_pawn_moves, gen_pawn_attacks, gen_single_pawn_moves},
        rays::generate_rays_attacks,
        squares_near_king::generate_square_close_to_king,
    },
};

use anyhow::Result;

pub struct LookupTables {
    pub rook_magics: Vec<MagicEntry>,
    pub rook_moves: Vec<Vec<Bitboard>>,
    pub bishop_magics: Vec<MagicEntry>,
    pub bishop_moves: Vec<Vec<Bitboard>>,
    pub knight_attacks: Vec<Bitboard>,
    pub king_attacks: Vec<Bitboard>,
    pub pawn_attacks: Vec<Vec<Bitboard>>,
    pub pawn_single_moves: Vec<Vec<Bitboard>>,
    pub pawn_double_moves: Vec<Vec<Bitboard>>,
    pub in_between: Vec<Vec<Bitboard>>,
    pub ray_attacks: Vec<Vec<Bitboard>>,
    pub squares_near_king: Vec<Vec<Bitboard>>,
}

#[derive(Clone, Copy)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: u64,
    pub index_bits: u8,
}

pub fn load_lookup_tables() -> Result<LookupTables> {
    let (rook_magics, rook_moves) = load_rook_magics()
        .map_err(|err| anyhow::format_err!("Couldn't load rook magics: {err:?}"))?;
    let (bishop_magics, bishop_moves) = load_bishop_magics()
        .map_err(|err| anyhow::format_err!("Couldn't load bishop magics: {err:?}"))?;
    let pawn_single_moves = gen_single_pawn_moves();
    let pawn_double_moves = gen_double_pawn_moves();
    let pawn_attacks = gen_pawn_attacks();
    let knight_attacks = gen_knight_attacks();
    let king_attacks = gen_king_attacks();
    let in_between = generate_in_between_squares();
    let ray_attacks = generate_rays_attacks();
    let squares_near_king = generate_square_close_to_king();

    Ok(LookupTables {
        rook_magics,
        rook_moves,
        bishop_magics,
        bishop_moves,
        knight_attacks,
        king_attacks,
        pawn_attacks,
        pawn_single_moves,
        pawn_double_moves,
        in_between,
        ray_attacks,
        squares_near_king,
    })
}

pub fn load_rook_magics() -> Result<(Vec<MagicEntry>, Vec<Vec<Bitboard>>)> {
    let mut magics = vec![
        MagicEntry {
            mask: Bitboard(0),
            magic: 0,
            index_bits: 0,
        };
        64
    ];

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("rook_magics.bin");

    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => std::fs::File::open("./rook_magics.bin")?,
    };

    for magic in magics.iter_mut() {
        let mut mask_bytes = [0u8; 8];
        file.read_exact(&mut mask_bytes)?;
        let mask = Bitboard(u64::from_be_bytes(mask_bytes));

        let mut magic_bytes = [0u8; 8];
        file.read_exact(&mut magic_bytes)?;
        let magic_number = u64::from_be_bytes(magic_bytes);

        let mut index_bits_bytes = [0u8; 1];
        file.read_exact(&mut index_bits_bytes)?;
        let index_bits = u8::from_be_bytes(index_bits_bytes);

        magic.mask = mask;
        magic.magic = magic_number;
        magic.index_bits = index_bits;
    }

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut moves = vec![vec![Bitboard(0); 1 << 12]; 64];

    let size = 1 << 12;

    for (idx, elem) in buffer.chunks_exact(8).enumerate() {
        moves[idx / size][idx % size] = Bitboard(u64::from_be_bytes(elem.try_into().unwrap()));
    }

    Ok((magics, moves))
}

pub fn load_bishop_magics() -> Result<(Vec<MagicEntry>, Vec<Vec<Bitboard>>)> {
    let mut magics = vec![
        MagicEntry {
            mask: Bitboard(0),
            magic: 0,
            index_bits: 0,
        };
        64
    ];

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("bishop_magics.bin");

    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => std::fs::File::open("./bishop_magics.bin")?,
    };

    for magic in magics.iter_mut() {
        let mut mask_bytes = [0u8; 8];
        file.read_exact(&mut mask_bytes)?;
        let mask = Bitboard(u64::from_be_bytes(mask_bytes));

        let mut magic_bytes = [0u8; 8];
        file.read_exact(&mut magic_bytes)?;
        let magic_number = u64::from_be_bytes(magic_bytes);

        let mut index_bits_bytes = [0u8; 1];
        file.read_exact(&mut index_bits_bytes)?;
        let index_bits = u8::from_be_bytes(index_bits_bytes);

        magic.mask = mask;
        magic.magic = magic_number;
        magic.index_bits = index_bits;
    }

    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    let size = 1 << 9;

    let mut moves = vec![vec![Bitboard(0); 1 << 9]; 64];

    for (idx, elem) in buffer.chunks_exact(8).enumerate() {
        moves[idx / size][idx % size] = Bitboard(u64::from_be_bytes(elem.try_into().unwrap()));
    }

    Ok((magics, moves))
}
