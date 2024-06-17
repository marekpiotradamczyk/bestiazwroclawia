#include "tt.hpp"

#include <gmock/gmock.h>

using chess::TranspositionTable;
using chess::TTData;
using testing::Optional;

TEST(TT, AddsAndGetsTheSameData) {
  auto tt = TranspositionTable(1);
  uint64_t hash = 1;
  TTData data(/*score=*/12, /*move=*/{}, /*depth=*/2, /*age=*/10,
              TTData::Type::EXACT);

  tt.add(hash, data);

  EXPECT_THAT(tt[hash], Optional(data));
}