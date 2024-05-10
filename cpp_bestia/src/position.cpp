#include <regex>

#include "position.hpp"

namespace position {

    Position::Position(std::vector<std::string> tokens) {
        if (tokens.size() == 1) { 
            throw std::invalid_argument("Missing startpos or FEN");
        }
        if (tokens[1] == "startpos") {
            // set default position
        }
        std::regex fenRegex("^([rnbqkpRNBQKP1-8]{1,8}/){7}[rnbqkpRNBQKP1-8]{1,8} (w|b) (-|[a-h][1-8]) (-|([1-9][0-9]*)) ([1-9][0-9]*)$");
        bool isValid = std::regex_match(tokens[1], fenRegex);
        if (!isValid) {
            throw std::invalid_argument("FEN is invalid");
        }
        // ...
    }
}