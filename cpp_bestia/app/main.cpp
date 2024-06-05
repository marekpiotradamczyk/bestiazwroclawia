#include <iostream>

#include "uci.hpp"
#include "engine.hpp"
#include "search.hpp"

int main(int argc, char **argv) {

  std::cout << "Engine ready\n";
  std::unique_ptr<chess::Engine> engine = std::make_unique<chess::MinMaxEngine>(); 
  uci::Uci uci(std::move(engine));
  uci.loop();

  return 0;
}
