#include "search.hpp"
#include "engine.hpp"
#include "heuristic.hpp"

namespace chess {

Move MinMaxEngine::search() {
  Move a;
  _search(board, depth, INT32_MIN, INT32_MAX, a);
  return a;
}

void MinMaxEngine::makeMove(Move a) { board.makeMove(a); }

void MinMaxEngine::setDepth(int d) { this->depth = d; }

void MinMaxEngine::setPosition(Board b) { this->board = b; }

Board MinMaxEngine::getPosition() { return this->board; }

int MinMaxEngine::getDepth() { return this->depth; }

I32 MinMaxEngine::quiescenceSearch(Board board, I32 alpha, I32 beta) {
  I32 standPat = heuristic(board);
  if (standPat >= beta) {
    return beta;
  }
  if (alpha < standPat) {
    alpha = standPat;
  }
  Movelist moveList;
  movegen::legalmoves<movegen::MoveGenType::CAPTURE>(moveList, board);
  for (Move m : moveList) {
    Board newBoard = board;
    newBoard.makeMove(m);
    I32 score = -quiescenceSearch(board, -beta, -alpha);
    if (score >= beta) {
      return beta;
    }
    alpha = std::max(alpha, score);
  }
  return alpha;
}
I32 MinMaxEngine::_search(Board board, int depth, I32 alpha,
                          I32 beta,  Move &move) {
  move = Move(Move::NO_MOVE);
  if (depth == 0) {
    return quiescenceSearch(board, alpha, beta);
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
  // @todo one can order movelist to make pruning happen earlier thus speeding
  // up the search
  for (Move m : moveList) {
    // @todo Needs some testing whether copying is faster or making move in
    // the original board and unmaking it later
    Board newBoard = board;
    newBoard.makeMove(m);
    Move _discard;
    I32 nodeValue = _search(newBoard, depth - 1, _discard, -beta, -alpha);
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
}; // namespace chess
