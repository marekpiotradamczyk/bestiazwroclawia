use rand::Rng;
use sdk::bitboard::Bitboard;

pub(crate) fn random_bitboard() -> Bitboard {
    let mut rng = rand::thread_rng();

    let random_u64: u64 = rng.gen();

    Bitboard(random_u64)
}
