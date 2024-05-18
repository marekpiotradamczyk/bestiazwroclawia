#include <iostream>

#include "uci.hpp"

int main(int argc, char **argv){

    std::cout << "Engine ready\n";
    uci::Uci uci(argc, argv);
    uci.loop();

    return 0;
}