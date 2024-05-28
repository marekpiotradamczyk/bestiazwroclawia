#ifndef __ENGINE__
#define __ENGINE__
#include "chess.hpp"
namespace chess {
namespace engine {

class Engine {
public:
  virtual ~Engine() = default;
  virtual Move search() = 0;
  virtual void makeMove(Move a) = 0;
  virtual void setPosition(Board board) = 0;
};
} // namespace engine
} // namespace chess
#endif // __ENGINE__
