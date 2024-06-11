#include "heuristic.hpp"
#include <algorithm>
#include <bit>
#include <utility>
#include <vector>

// Taken directly from
// https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function
// same values are used by the original rust engine
// clang-format off
int mg_pawn_table[64] = {
      0,   0,   0,   0,   0,   0,  0,   0,
     98, 134,  61,  95,  68, 126, 34, -11,
     -6,   7,  26,  31,  65,  56, 25, -20,
    -14,  13,   6,  21,  23,  12, 17, -23,
    -27,  -2,  -5,  12,  17,   6, 10, -25,
    -26,  -4,  -4, -10,   3,   3, 33, -12,
    -35,  -1, -20, -23, -15,  24, 38, -22,
      0,   0,   0,   0,   0,   0,  0,   0,
};

int eg_pawn_table[64] = {
      0,   0,   0,   0,   0,   0,   0,   0,
    178, 173, 158, 134, 147, 132, 165, 187,
     94, 100,  85,  67,  56,  53,  82,  84,
     32,  24,  13,   5,  -2,   4,  17,  17,
     13,   9,  -3,  -7,  -7,  -8,   3,  -1,
      4,   7,  -6,   1,   0,  -5,  -1,  -8,
     13,   8,   8,  10,  13,   0,   2,  -7,
      0,   0,   0,   0,   0,   0,   0,   0,
};

int mg_knight_table[64] = {
    -167, -89, -34, -49,  61, -97, -15, -107,
     -73, -41,  72,  36,  23,  62,   7,  -17,
     -47,  60,  37,  65,  84, 129,  73,   44,
      -9,  17,  19,  53,  37,  69,  18,   22,
     -13,   4,  16,  13,  28,  19,  21,   -8,
     -23,  -9,  12,  10,  19,  17,  25,  -16,
     -29, -53, -12,  -3,  -1,  18, -14,  -19,
    -105, -21, -58, -33, -17, -28, -19,  -23,
};

int eg_knight_table[64] = {
    -58, -38, -13, -28, -31, -27, -63, -99,
    -25,  -8, -25,  -2,  -9, -25, -24, -52,
    -24, -20,  10,   9,  -1,  -9, -19, -41,
    -17,   3,  22,  22,  22,  11,   8, -18,
    -18,  -6,  16,  25,  16,  17,   4, -18,
    -23,  -3,  -1,  15,  10,  -3, -20, -22,
    -42, -20, -10,  -5,  -2, -20, -23, -44,
    -29, -51, -23, -15, -22, -18, -50, -64,
};

int mg_bishop_table[64] = {
    -29,   4, -82, -37, -25, -42,   7,  -8,
    -26,  16, -18, -13,  30,  59,  18, -47,
    -16,  37,  43,  40,  35,  50,  37,  -2,
     -4,   5,  19,  50,  37,  37,   7,  -2,
     -6,  13,  13,  26,  34,  12,  10,   4,
      0,  15,  15,  15,  14,  27,  18,  10,
      4,  15,  16,   0,   7,  21,  33,   1,
    -33,  -3, -14, -21, -13, -12, -39, -21,
};

int eg_bishop_table[64] = {
    -14, -21, -11,  -8, -7,  -9, -17, -24,
     -8,  -4,   7, -12, -3, -13,  -4, -14,
      2,  -8,   0,  -1, -2,   6,   0,   4,
     -3,   9,  12,   9, 14,  10,   3,   2,
     -6,   3,  13,  19,  7,  10,  -3,  -9,
    -12,  -3,   8,  10, 13,   3,  -7, -15,
    -14, -18,  -7,  -1,  4,  -9, -15, -27,
    -23,  -9, -23,  -5, -9, -16,  -5, -17,
};

int mg_rook_table[64] = {
     32,  42,  32,  51, 63,  9,  31,  43,
     27,  32,  58,  62, 80, 67,  26,  44,
     -5,  19,  26,  36, 17, 45,  61,  16,
    -24, -11,   7,  26, 24, 35,  -8, -20,
    -36, -26, -12,  -1,  9, -7,   6, -23,
    -45, -25, -16, -17,  3,  0,  -5, -33,
    -44, -16, -20,  -9, -1, 11,  -6, -71,
    -19, -13,   1,  17, 16,  7, -37, -26,
};

int eg_rook_table[64] = {
    13, 10, 18, 15, 12,  12,   8,   5,
    11, 13, 13, 11, -3,   3,   8,   3,
     7,  7,  7,  5,  4,  -3,  -5,  -3,
     4,  3, 13,  1,  2,   1,  -1,   2,
     3,  5,  8,  4, -5,  -6,  -8, -11,
    -4,  0, -5, -1, -7, -12,  -8, -16,
    -6, -6,  0,  2, -9,  -9, -11,  -3,
    -9,  2,  3, -1, -5, -13,   4, -20,
};

int mg_queen_table[64] = {
    -28,   0,  29,  12,  59,  44,  43,  45,
    -24, -39,  -5,   1, -16,  57,  28,  54,
    -13, -17,   7,   8,  29,  56,  47,  57,
    -27, -27, -16, -16,  -1,  17,  -2,   1,
     -9, -26,  -9, -10,  -2,  -4,   3,  -3,
    -14,   2, -11,  -2,  -5,   2,  14,   5,
    -35,  -8,  11,   2,   8,  15,  -3,   1,
     -1, -18,  -9,  10, -15, -25, -31, -50,
};

int eg_queen_table[64] = {
     -9,  22,  22,  27,  27,  19,  10,  20,
    -17,  20,  32,  41,  58,  25,  30,   0,
    -20,   6,   9,  49,  47,  35,  19,   9,
      3,  22,  24,  45,  57,  40,  57,  36,
    -18,  28,  19,  47,  31,  34,  39,  23,
    -16, -27,  15,   6,   9,  17,  10,   5,
    -22, -23, -30, -16, -16, -23, -36, -32,
    -33, -28, -22, -43,  -5, -32, -20, -41,
};

int mg_king_table[64] = {
    -65,  23,  16, -15, -56, -34,   2,  13,
     29,  -1, -20,  -7,  -8,  -4, -38, -29,
     -9,  24,   2, -16, -20,   6,  22, -22,
    -17, -20, -12, -27, -30, -25, -14, -36,
    -49,  -1, -27, -39, -46, -44, -33, -51,
    -14, -14, -22, -46, -44, -30, -15, -27,
      1,   7,  -8, -64, -43, -16,   9,   8,
    -15,  36,  12, -54,   8, -28,  24,  14,
};

int eg_king_table[64] = {
    -74, -35, -18, -18, -11,  15,   4, -17,
    -12,  17,  14,  17,  17,  38,  23,  11,
     10,  17,  23,  15,  20,  45,  44,  13,
     -8,  22,  24,  27,  26,  33,  26,   3,
    -18,  -4,  21,  24,  27,  23,   9, -11,
    -19,  -3,  11,  21,  23,  16,   7,  -9,
    -27, -11,   4,  13,  14,   4,  -5, -17,
    -53, -34, -21, -11, -28, -14, -24, -43
};

int KING_SAFETY_TABLE[150] = {
    0, 0, 0, 1, 1, 2, 3, 4, 5, 6, 8, 10, 13, 16, 20, 25, 30, 36, 42, 48, 55, 62, 70, 80, 90, 100,
    110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290,
    300, 310, 320, 330, 340, 350, 360, 370, 380, 390, 400, 410, 420, 430, 440, 450, 460, 470, 480,
    490, 500, 510, 520, 530, 540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
    650, 650, 650, 650, 650, 650, 650, 650, 650, 650,
};

const int PIECE_ATTACK_UNITS[6] = {0, 2, 2, 3, 5, 0};
const int BONUS_FOR_UNIT = 15;
const int MAX_BONUS_FOR_UNIT = 60;


// clang-format on
int *mg_pesto_table[6] = {mg_pawn_table, mg_knight_table, mg_bishop_table,
                          mg_rook_table, mg_queen_table,  mg_king_table};

int *eg_pesto_table[6] = {eg_pawn_table, eg_knight_table, eg_bishop_table,
                          eg_rook_table, eg_queen_table,  eg_king_table};

namespace chess {

I16 getPieceCount(const Board &board, PieceType pieceType, Color color) {
  return std::popcount(board.pieces(pieceType, color).getBits());
}
I16 getPieceCountAll(const Board &board, PieceType pieceType) {
  return std::popcount(board.pieces(pieceType).getBits());
}

PieceType type[] = {PieceType::PAWN, PieceType::KNIGHT, PieceType::BISHOP,
                    PieceType::ROOK, PieceType::QUEEN,  PieceType::KING};

I16 materialHeuristic(const Board &board) {
  I16 value[] = {100, 300, 325, 500, 900};
  Color color[] = {Color::WHITE, Color::BLACK};
  I16 colorMultiplier[] = {1, -1};
  I16 returnValue = 0;
  for (int i = 0; i < 2; i++) {   // for every color
    for (int j = 0; j < 5; j++) { // for every piece type except king
      returnValue += colorMultiplier[i] * value[j] *
                     getPieceCount(board, type[j], color[i]);
    }
  }
  return (board.sideToMove() == Color::WHITE ? 1 : -1) * returnValue;
}

// Phase is high at start of the game and lowers as more and more material gets
// removed from the board
std::pair<I16, I16> getCurrentPhase(const Board &board) {
  I16 value[] = {0, 1, 1, 2, 4};
  // 16 pawns, 4 knights, 4 bishops, 4 rooks and 2 queens in total
  I16 maxPhase =
      value[0] * 16 + value[1] * 4 + value[2] * 4 + value[3] * 4 + value[4] * 2;
  // for every piece type except king
  I16 phase = 0;
  for (int i = 0; i < 5; i++) {
    phase = value[i] * getPieceCountAll(board, type[i]);
  }
  // std::max in case of early promotion
  return {std::max(phase, maxPhase), maxPhase};
}

std::vector<int> fromBitboard(const Bitboard &b) {
  std::vector<int> returnValue;
  for (int i = 0; i < 64; i++) {
    Bitboard current = Bitboard::fromSquare(i);
    if (current & b) {
      returnValue.push_back(i);
    }
  }
  return returnValue;
}

I16 pieceSquareHeuristic(const Board &board) {
  auto [phase, maxPhase] = getCurrentPhase(board);
  I16 middlegame = 0;
  I16 endgame = 0;
  Color color[] = {Color::WHITE, Color::BLACK};
  I16 colorMultiplier[] = {1, -1};
  for (int c = 0; c < 2; c++) {
    for (int i = 0; i < 6; i++) {
      std::vector<int> occupied = fromBitboard(board.pieces(type[i], color[c]));
      for (int sq : occupied) {
        // sq ^ 56 "flips" the square to corresponding square of the other color (for example e1 to e8, and e8 to e1)
        sq = (color[c] == Color::WHITE ? sq ^ 56 : sq);
        middlegame += colorMultiplier[c] * mg_pesto_table[i][sq];
        endgame += colorMultiplier[c] * eg_pesto_table[i][sq];
      }
    }
  }
  return (board.sideToMove() == Color::WHITE ? 1 : -1) *
         (middlegame * phase + endgame * (maxPhase - phase)) / maxPhase;
}

//////////////////////////////////////////////////////////////
/// TODO:
/// both this functions can be used to populate lookup tables like in rust version
/// made to complete Issue #31
Bitboard squares_near_king(Square king_sq, Color king_color)
{
    Bitboard squares = attacks::king(king_sq);
    Direction dir = king_color == Color::WHITE ? Direction::NORTH : Direction::SOUTH;
    Bitboard shifted = squares;
    switch (dir) {
        case Direction::NORTH:
            shifted = shifted << 8;
            break;
        case Direction::SOUTH:
            shifted = shifted >> 8;
            break;
        default:
            break;
    }
    return squares |= shifted;
}

/// @brief function to generate squares near king for both kings and all squares, 0 - WHITE, 1 - BLACK
/// @return 
std::vector<std::vector<Bitboard>> generate_square_close_to_king()
{
    std::vector<std::vector<Bitboard>> squares(2, std::vector<Bitboard> (10, 0));  
    Color color[] = {Color::WHITE, Color::BLACK};
    for (int c = 0; c < 2; c++) 
        for(int i=0; i<64; i++)
            squares[c][i] = squares_near_king(i, c);
    return squares;
}
//////////////////////////////////////////////////////////////


I16 calc_king_safety_units(const Board &board, Color color)
{
    Square king_sq = board.kingSq(color);
    Bitboard near_king = squares_near_king(king_sq, color);
    int bonus = 0;
    for(Square sq : fromBitboard(near_king))
    {
        for(Square piece_sq : fromBitboard(attacks::attackers(board, ~color, sq)))
        {
            Piece piece = board.at(piece_sq);
            bonus += PIECE_ATTACK_UNITS[piece.type()];
        }
    }

    return bonus;
}

I16 pieces_close_to_king_count(const Board &board, Color color)
{
    Square king_sq = board.kingSq(color);
    Bitboard near_king = squares_near_king(king_sq, color);
    int friendly_pieces_count = 0;
    int enemy_pieces_count = 0;
    for (int j = 0; j < 6; j++) { // for every piece type
        friendly_pieces_count += std::popcount((board.pieces(type[j], color) & near_king).getBits());
        enemy_pieces_count    += std::popcount((board.pieces(type[j], ~color) & near_king).getBits());
    }
    friendly_pieces_count--; // friendly king can not be counted 

    return friendly_pieces_count - enemy_pieces_count;
}

I16 calc_king_safety(const Board &board)
{
    I16 white_units = calc_king_safety_units(board, Color::WHITE);
    I16 black_units = calc_king_safety_units(board, Color::BLACK);
    int side_multiplier = board.sideToMove() == Color::WHITE ? 1 : -1;
    return -(KING_SAFETY_TABLE[white_units] - KING_SAFETY_TABLE[black_units])*side_multiplier;
}


I16 bonus_for_pieces_close_to_king(const Board &board)
{
    I16 white_count = pieces_close_to_king_count(board, Color::WHITE);
    I16 black_count = pieces_close_to_king_count(board, Color::BLACK);
    int side_multiplier = board.sideToMove() == Color::WHITE ? 1 : -1;

    return std::clamp(((white_count - black_count) * BONUS_FOR_UNIT), -MAX_BONUS_FOR_UNIT, MAX_BONUS_FOR_UNIT)*side_multiplier;
}




I16 heuristic(const Board &board) {
  // We might want to finetune this so that the engine focuses more on material
  // or more on piece activity
  const I16 materialMultiplier = 1;
  const I16 pieceSquareMultiplier = 1;
  const I16 kingSafetyMultiplier = 1;
  const I16 kingNearFriendsMultiplier = 1;
  return materialMultiplier * materialHeuristic(board) +
         pieceSquareMultiplier * pieceSquareHeuristic(board) + 
         kingSafetyMultiplier * calc_king_safety(board) +
         kingNearFriendsMultiplier * bonus_for_pieces_close_to_king(board);
}

} // namespace chess
