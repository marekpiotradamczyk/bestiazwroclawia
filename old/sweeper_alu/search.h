#ifndef SEARCH_H
#define SEARCH_H

#include <chrono>

#include "position.h"
#include "types.h"

class Search{
public:
  Search() {}
  MoveType BestMove(Position &position);

  int time_left;

private:
  int NegaMax(Position &position, int depth, int alpha, int beta, unsigned int position_hash, unsigned int long_hash, int last_dst);
  int Quiescence(Position &position, int alpha, int beta, int last_dst);
  bool TimeIsOver();

  int eval_nodes;
  int cuttoffs;
  int transposition_usage;
  bool level_completed;
  std::chrono::time_point<std::chrono::system_clock> start_time;
  std::chrono::duration<int, std::centi> target_time;
  int moves_made;
  int control_len;
};

#endif // SEARCH_H
