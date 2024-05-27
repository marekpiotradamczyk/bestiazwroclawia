#include <iostream>

#include "uci.hpp"
#include "engine.hpp"

int main(int argc, char **argv) {

  std::cout << "Engine ready\n";
  engine::Engine engine;
  uci::Uci uci(engine);
  uci.loop();

  return 0;
}