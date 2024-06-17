#include "search.hpp"

namespace chess {

const I32 winValue = 20000;
const I32 loseValue = -winValue;
const I32 drawValue = 0;

I32 search(Board board, int depth, Move &move, I32 alpha, I32 beta) {
  move = Move(Move::NO_MOVE);
  if (depth == 0) {
    return heuristic(board);
  }
  Movelist moveList;
  movegen::legalmoves(moveList, board);
  auto result = board.isGameOver(moveList);
  switch (result.second) {
  case GameResult::DRAW:
    return drawValue;
  case GameResult::WIN:
    return (depth + 1) * winValue; // prioritize faster wins
  case GameResult::LOSE:
    return (depth + 1) * loseValue; // prioritize slower loses
  case GameResult::NONE:
    break;
  }
  I32 value = INT32_MIN;
  // @todo one can order movelist to make pruning happen earlier thus speeding up the
  //       search
  for (Move m : moveList) {
    // @todo Needs some testing whether copying is faster or making move in the
    //       original board and unmaking it later
    Board newBoard = board;
    newBoard.makeMove(m);
    Move _discard;
    I32 nodeValue = -search(newBoard, depth - 1, _discard, -beta, -alpha);
    if (nodeValue >= value) {
      value = nodeValue;
      move = m;
    }
    alpha = std::max(alpha, value);
    if (alpha >= beta) {
      break; // cutoff
    }
  }
  return value;
}
} // namespace chess
