#include <iostream>

#include "uci.hpp"
#include "engine.hpp"
#include "search.hpp"

int main(int argc, char **argv) {

  std::cout << "Engine ready\n";
  chess::MinMaxEngine engine; 
  uci::Uci uci(engine);
  uci.loop();

  return 0;
}
