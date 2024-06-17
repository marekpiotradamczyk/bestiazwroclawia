#include "tt.hpp"

#include <gmock/gmock.h>

#include <thread>
#include <vector>

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

TEST(TT, ManyThreadsAddsTheSameValue_EntryIsInvalid) {
  auto tt = TranspositionTable(1);
  uint64_t hash = 1;
  const int NUM_THREADS = 32;

  // Creates threads that will write different data to TT under the same hash.
  std::vector<std::thread> threads;
  for (int i = 0; i < NUM_THREADS; ++i) {
    threads.emplace_back([&]() {
      tt.add(hash, {/*score=*/i, /*move=*/{}, /*depth=*/2, /*age=*/10,
                    TTData::Type::EXACT});
    });
  }
  for (auto& thread : threads) {
    thread.join();
  }

  // Actually this test is not 100% guaranteed to pass.
  // I'm not sure what are the chances for it to fail.
  EXPECT_FALSE(tt[hash].has_value());
}