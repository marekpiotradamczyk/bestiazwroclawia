#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "uci.hpp"
#include "engine.hpp"
#include "position.hpp"

namespace uci {

    Uci::Uci(int argc, char** argv) {
        engine = engine::Engine();
    }

    void Uci::loop() {
        std::string input;
    
        while (std::getline(std::cin, input)) {
            std::stringstream iss(input);
            std::vector<std::string> tokens;
            std::string token;
            while (iss >> token) {
                tokens.push_back(token);
            }

            if (tokens.empty()) {
                continue;
            }
            if (tokens[0] == "isready") {
                std::cout<<"readyok\n";
            } else if (tokens[0] == "uci") {
                std::cout<<"BestiaZWroclawia v0.1\n";
            } else if (tokens[0] == "quit") {
                exit(0);
            } else if (tokens[0] == "stop") {
                std::cout<<"stop\n";
            } else if (tokens[0] == "position") {
                parsePosition(tokens);
            } else if (tokens[0] == "go") {
                parseGo(tokens);
            } else if (tokens[0] == "setoption") {
                parseEngineOption(tokens);
            } else {
                std::cout << "Unknown command\n";
            }
        }
    }

    void Uci::parseGo(std::vector<std::string> tokens) {
        // TODO: to run this we need to parse position first
        // then parse search options
    }

    void Uci::parsePosition(std::vector<std::string> tokens) {
        // TODOD: parse position (FEN or startpos)
    }

    void Uci::parseEngineOption(std::vector<std::string> tokens) {
        // TODO: parse options and set them for engine
    }

}