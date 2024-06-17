#include "chess.hpp"

namespace chess {

using I16 = int16_t;
I16 materialHeuristic(const Board &board);
I16 pieceSquareHeuristic(const Board &board);
I16 heuristic(const chess::Board &board);

} // namespace chess
