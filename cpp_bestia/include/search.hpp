#include "chess.hpp"
#include "heuristic.hpp"

namespace chess {
using I32 = int32_t;
/// @brief    Performs search starting from position with given depth, writes
/// best move to reference
/// @param    board             Position to start the search from
/// @param    depth             Depth of search
/// @param    move              where to write best move to
/// @param    alpha             alpha for alpha-beta pruning
/// @param    beta              beta for alpha-beta pruning
/// @return                     value of position
I32 search(Board board, int depth, Move &move, I32 alpha=INT32_MIN, I32 beta=INT32_MAX);

} // namespace chess
