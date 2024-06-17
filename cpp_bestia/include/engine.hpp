#ifndef __ENGINE__
#define __ENGINE__
#include "chess.hpp"
namespace chess {

class Engine {
public:
  Engine();
  void setBoard(Board board);
  void setDepth(int depth);
  void search();
  Board getBoard();
private:
  Board board;
};
} // namespace chess
#endif // __ENGINE__