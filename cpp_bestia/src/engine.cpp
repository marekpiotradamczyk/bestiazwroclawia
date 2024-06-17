#include "chess.hpp"
#include "engine.hpp"

namespace chess {

Engine::Engine() {};

void Engine::setBoard(Board board) {
    this->board = board;
}

Board Engine::getBoard() {
    return board;
}
} // namespace engine