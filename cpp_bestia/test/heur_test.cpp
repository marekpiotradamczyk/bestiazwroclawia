#include "heuristic.hpp"

#include <gmock/gmock.h>


TEST(MATERIAL, MaterialCounted) {
    auto board = chess::Board("1k6/8/8/8/8/8/8/6QK w - - 0 1");
    auto queenW = chess::materialHeuristic(board);
    board = chess::Board("1k6/8/8/8/8/8/8/6QK b - - 0 1");
    auto queenB = chess::materialHeuristic(board);
    EXPECT_THAT(queenW, 900);
    EXPECT_THAT(queenB, -900);
    board = chess::Board("1k2nn2/8/8/8/8/8/4NN2/4NN1K w - - 0 1");
    auto knights = chess::materialHeuristic(board);
    board = chess::Board("1k6/8/8/4p3/8/8/8/4B2K b - - 0 1");
    auto bishops = chess::materialHeuristic(board);
    EXPECT_THAT(knights, 600);
    EXPECT_THAT(bishops, -225);
}