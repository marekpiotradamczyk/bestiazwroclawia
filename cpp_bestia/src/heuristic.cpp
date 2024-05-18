#include "heuristic.hpp"
#include <bit>

I16 getPieceCount(const chess::Board &board, chess::PieceType pieceType,
                  chess::Color color) {
  return std::popcount(board.pieces(pieceType, color).getBits());
}

I16 heuristic(const chess::Board &board) {
  using namespace chess;
  PieceType type[] = {PieceType::PAWN, PieceType::KNIGHT, PieceType::BISHOP,
                      PieceType::ROOK, PieceType::QUEEN};
  I16 value[] = {100, 300, 325, 500, 900};
  Color color[] = {Color::WHITE, Color::BLACK};
  I16 colorMultiplier[] = {1, -1};
  I16 returnValue = 0;
  for (int i = 0; i < 2; i++) {   // for every color
    for (int j = 0; j < 5; j++) { // for every piece type
      returnValue += colorMultiplier[i] * value[j] *
                     getPieceCount(board, type[j], color[i]);
    }
  }
  return returnValue;
}
