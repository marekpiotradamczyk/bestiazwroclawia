#include <iostream>

#include "uci.hpp"
#include "chess.hpp"

int main(int argc, char **argv) {

  std::cout << "Engine ready\n";
  chess::Engine engine;
  uci::Uci uci(engine);
  uci.loop();

  return 0;
}