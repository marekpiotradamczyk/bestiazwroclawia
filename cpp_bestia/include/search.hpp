#ifndef __SEARCH__
#define __SEARCH__
#include "chess.hpp"
#include "engine.hpp"
#include "heuristic.hpp"

namespace chess {
using I32 = int32_t;
class MinMaxEngine : public engine::Engine {
public:
  /// @brief Starts the search from position remembered in instance
  Move search();
  // Change constructor and destructor when implementing TT
  MinMaxEngine() = default; 
  ~MinMaxEngine() = default;
  /// @breaf Makes move in the position remembered in instance
  void makeMove(Move a);
  void setPosition(Board b);
  void setDepth(int d);
  Board getPosition();
  int getDepth();

private:
  Board board;
  int depth;
  const I32 winValue = 20000;
  const I32 loseValue = -winValue;
  const I32 drawValue = 0;
  I32 _search(Board board, int depth, Move &move, I32 alpha, I32 beta);
  I32 quiescenceSearch(Board board, I32 alpha, I32 beta);
};
} // namespace chess
#endif
