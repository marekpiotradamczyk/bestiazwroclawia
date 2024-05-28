#pragma once

#include <stdint.h>

#include <vector>

#include "chess.hpp"

namespace chess {

struct TTEntry {
  enum class Type : uint8_t {
    NONE,
    UPPER,
    LOWER,
    EXACT,
  };

  int32_t score;
  Move move;
  uint8_t depth;
  Type type = Type::NONE;
};

class TranspositionTable {
 public:
  TranspositionTable(uint64_t size_in_mb);

  const TTEntry* operator[](uint64_t hash) const;
  void add(uint64_t hash, int32_t score, Move move, uint8_t depth,
           TTEntry::Type type);

 private:
  std::vector<TTEntry> entries_;
};

}  // namespace chess