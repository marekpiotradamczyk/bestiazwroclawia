use sdk::position::{Color, Position};

pub const PIECE_TABLES: [[[i32; 64]; 6]; 2] = [
    [
        // White Pawns
        [
            0, 0, 0, 0, 0, 0, 0, 0, // 1st rank
            5, 10, 10, -20, -20, 10, 10, 5, // 2nd rank
            5, -5, -10, 0, 0, -10, -5, 5, // 3rd rank
            0, 0, 0, 20, 20, 0, 0, 0, // 4th rank
            5, 5, 10, 25, 25, 10, 5, 5, // 5th rank
            10, 10, 20, 30, 30, 20, 10, 10, // 6th rank
            50, 50, 50, 50, 50, 50, 50, 50, // 7th rank
            0, 0, 0, 0, 0, 0, 0, 0, // 8th rank
        ], // White Knights
        [
            -50, -40, -30, -30, -30, -30, -40, -50, // 1st rank
            -40, -20, 0, 0, 0, 0, -20, -40, // 2nd rank
            -30, 0, 10, 15, 15, 10, 0, -30, // 3rd rank
            -30, 5, 15, 20, 20, 15, 5, -30, // 4th rank
            -30, 0, 15, 20, 20, 15, 0, -30, // 5th rank
            -30, 5, 10, 15, 15, 10, 5, -30, // 6th rank
            -40, -20, 0, 5, 5, 0, -20, -40, // 7th rank
            -50, -40, -30, -30, -30, -30, -40, -50, // 8th rank
        ], // White Bishops
        [
            -20, -10, -10, -10, -10, -10, -10, -20, // 1st rank
            -10, 0, 0, 0, 0, 0, 0, -10, // 2nd rank
            -10, 0, 5, 10, 10, 5, 0, -10, // 3rd rank
            -10, 5, 5, 10, 10, 5, 5, -10, // 4th rank
            -10, 0, 10, 10, 10, 10, 0, -10, // 5th rank
            -10, 10, 10, 10, 10, 10, 10, -10, // 6th rank
            -10, 5, 0, 0, 0, 0, 5, -10, // 7th rank
            -20, -10, -10, -10, -10, -10, -10, -20, // 8th rank
        ],
        // White Rooks
        [
            0, 0, 0, 5, 5, 0, 0, 0, // 1st rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 2nd rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 3rd rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 4th rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 5th rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 6th rank
            5, 10, 10, 10, 10, 10, 10, 5, // 7th rank
            0, 0, 0, 0, 0, 0, 0, 0, // 8th rank
        ],
        // White Queens
        [
            -20, -10, -10, -5, -5, -10, -10, -20, // 1st rank
            -10, 0, 0, 0, 0, 0, 0, -10, // 2nd rank
            -10, 0, 5, 5, 5, 5, 0, -10, // 3rd rank
            -5, 0, 5, 5, 5, 5, 0, -5, // 4th rank
            0, 0, 5, 5, 5, 5, 0, -5, // 5th rank
            -10, 5, 5, 5, 5, 5, 0, -10, // 6th rank
            -10, 0, 5, 0, 0, 0, 0, -10, // 7th rank
            -20, -10, -10, -5, -5, -10, -10, -20, // 8th rank
        ],
        // White Kings
        [
            0, 0, 0, 0, 0, 0, 0, 0, // 1st rank
            0, 0, 0, 0, 0, 0, 0, 0, // 2nd rank
            0, 0, 0, 0, 0, 0, 0, 0, // 3rd rank
            0, 0, 0, 0, 0, 0, 0, 0, // 4th rank
            0, 0, 0, 0, 0, 0, 0, 0, // 5th rank
            0, 0, 0, 0, 0, 0, 0, 0, // 6th rank
            0, 0, 0, 0, 0, 0, 0, 0, // 7th rank
            0, 0, 0, 0, 0, 0, 0, 0, // 8th rank
        ],
    ],
    [
        // Black pawns
        [
            0, 0, 0, 0, 0, 0, 0, 0, // 1st rank
            50, 50, 50, 50, 50, 50, 50, 50, // 2nd rank
            10, 10, 20, 30, 30, 20, 10, 10, // 3rd rank
            5, 5, 10, 25, 25, 10, 5, 5, // 4th rank
            0, 0, 0, 20, 20, 0, 0, 0, // 5th rank
            5, -5, -10, 0, 0, -10, -5, 5, // 6th rank
            5, 10, 10, -20, -20, 10, 10, 5, // 7th rank
            0, 0, 0, 0, 0, 0, 0, 0, // 8th rank
        ], // Black knights
        [
            -50, -40, -30, -30, -30, -30, -40, -50, // 1st rank
            -40, -20, 0, 5, 5, 0, -20, -40, // 2nd rank
            -30, 5, 10, 15, 15, 10, 5, -30, // 3rd rank
            -30, 0, 15, 20, 20, 15, 0, -30, // 4th rank
            -30, 5, 15, 20, 20, 15, 5, -30, // 5th rank
            -30, 0, 10, 15, 15, 10, 0, -30, // 6th rank
            -40, -20, 0, 0, 0, 0, -20, -40, // 7th rank
            -50, -40, -30, -30, -30, -30, -40, -50, // 8th rank
        ],
        // Black bishops
        [
            -20, -10, -10, -5, -5, -10, -10, -20, // 1st rank
            -10, 5, 0, 0, 0, 0, 5, -10, // 2nd rank
            -10, 10, 10, 10, 10, 10, 10, -10, // 3rd rank
            -10, 0, 10, 10, 10, 10, 0, -10, // 4th rank
            -10, 5, 5, 10, 10, 5, 5, -10, // 5th rank
            -10, 0, 5, 10, 10, 5, 0, -10, // 6th rank
            -10, 0, 0, 0, 0, 0, 0, -10, // 7th rank
            -20, -10, -10, -5, -5, -10, -10, -20, // 8th rank
        ],
        // Black rooks
        [
            0, 0, 0, 0, 0, 0, 0, 0, // 1st rank
            5, 10, 10, 10, 10, 10, 10, 5, // 2nd rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 3rd rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 4th rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 5th rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 6th rank
            -5, 0, 0, 0, 0, 0, 0, -5, // 7th rank
            0, 0, 0, 5, 5, 0, 0, 0, // 8th rank
        ],
        // Black queens
        [
            -20, -10, -10, -5, -5, -10, -10, -20, // 1st rank
            -10, 0, 5, 0, 0, 0, 0, -10, // 2nd rank
            -10, 5, 5, 5, 5, 5, 0, -10, // 3rd rank
            0, 0, 5, 5, 5, 5, 0, -5, // 4th rank
            -5, 0, 5, 5, 5, 5, 0, -5, // 5th rank
            -10, 0, 5, 5, 5, 5, 0, -10, // 6th rank
            -10, 0, 0, 0, 0, 0, 0, -10, // 7th rank
            -20, -10, -10, -5, -5, -10, -10, -20, // 8th rank
        ],
        // Black kings
        [
            0, 0, 0, 0, 0, 0, 0, 0, // 1st rank
            0, 0, 0, 0, 0, 0, 0, 0, // 2nd rank
            0, 0, 0, 0, 0, 0, 0, 0, // 3rd rank
            0, 0, 0, 0, 0, 0, 0, 0, // 4rd rank
            0, 0, 0, 0, 0, 0, 0, 0, // 5rd rank
            0, 0, 0, 0, 0, 0, 0, 0, // 6rd rank
            0, 0, 0, 0, 0, 0, 0, 0, // 7rd rank
            0, 0, 0, 0, 0, 0, 0, 0, // 8rd rank
        ],
    ],
];

pub fn positional_bonus(position: &Position) -> i32 {
    let mut score = 0;

    let mut piece_type = 0;

    while piece_type < 6 {
        let mut white_pieces = position.pieces[Color::White as usize][piece_type];
        let mut black_pieces = position.pieces[Color::Black as usize][piece_type];

        while !white_pieces.is_empty() {
            let square = white_pieces.lsb();
            white_pieces.0 ^= square.bitboard().0;

            score += PIECE_TABLES[Color::White as usize][piece_type][square as usize];
        }

        while !black_pieces.is_empty() {
            let square = black_pieces.lsb();
            black_pieces.0 ^= square.bitboard().0;

            score -= PIECE_TABLES[Color::Black as usize][piece_type][square as usize];
        }

        piece_type += 1;
    }

    score
}
