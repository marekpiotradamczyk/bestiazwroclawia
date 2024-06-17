#ifndef __UCI__
#define __UCI__

#include <memory>
#include <string>
#include <vector>

#include "engine.hpp"

namespace uci {

class Uci {
public:
  // Use (at least from what I understand about unique pointers):
  // unique_ptr<SomeEngine> engine(new SomeEngine(...));
  // Uci uci(std::move(engine));
  Uci(std::unique_ptr<chess::Engine> engine);
  void loop();

private:
  std::unique_ptr<chess::Engine> engine;
  void parseEngineOption(std::vector<std::string> tokens);
  void parseGo(std::vector<std::string> tokens);
  void parsePosition(std::vector<std::string> tokens);
  bool isFENValid(std::string fen);
};
} // namespace uci
#endif // __UCI__
