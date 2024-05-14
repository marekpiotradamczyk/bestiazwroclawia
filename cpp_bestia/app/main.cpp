#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <regex>

#include "position.hpp"

void parse_go() {
    // something here
    return;
}

int main(){

    std::string input;
    std::cout << "Engine ready\n";
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
            position::Position position = position::Position(tokens);
        } else if (tokens[0] == "go") {
            parse_go();
        } else if (tokens[0] == "setoption") {
            
        } else {
            std::cout << "Unknown command\n";
        }
    }

    return 0;
}