#ifndef __UCI__
#define __UCI__

#include <string>
#include <vector>

#include "chess.hpp"
#include "engine.hpp"

namespace uci {

class Uci {
public:
  Uci(chess::Engine& engine);
  void loop();

private:
  chess::Engine engine;
  void parseEngineOption(std::vector<std::string> tokens);
  void parseGo(std::vector<std::string> tokens);
  void parsePosition(std::vector<std::string> tokens);
  bool isFENValid(const std::string& fen);
};
} // namespace uci
#endif // __UCI__