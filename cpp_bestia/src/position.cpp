#include <regex>

#include "position.hpp"
// TODO: all this stuff
namespace position {

    Position::Position(std::vector<std::string> tokens) {
        if (tokens.size() == 1) { 
            throw std::invalid_argument("Missing startpos or FEN");
        }
        if (tokens[1] == "startpos") {
            // set default position
        }
        std::regex fenRegex("^([rnbqkpRNBQKP1-8]{1,8}/){7}[rnbqkpRNBQKP1-8]{1,8} (w|b) (-|[a-h][1-8]) (-|([1-9][0-9]*)) ([1-9][0-9]*)$");
        //...
    }
}