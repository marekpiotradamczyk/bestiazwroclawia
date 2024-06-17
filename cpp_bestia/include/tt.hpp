#pragma once

#include <stdint.h>

#include <atomic>
#include <memory>
#include <optional>

#include "chess.hpp"

namespace chess {

// Implementation based on
// https://www.chessprogramming.org/Shared_Hash_Table#Lockless

class TTData {
 public:
  enum class Type : uint8_t {
    NONE,
    ALPHA,
    BETA,
    EXACT,
  };

  TTData(uint64_t data);
  TTData(int32_t score, Move move, uint8_t depth, uint8_t age, Type type);

  int32_t score() const;
  Move move() const;
  uint8_t depth() const;
  uint8_t age() const;
  Type type() const;
  uint64_t data() const;

  bool operator==(const TTData& other) const;

  static uint64_t pack(int32_t score, Move move, uint8_t depth, uint8_t age,
                       Type type);

 private:
  static constexpr uint64_t SCORE_BITS = 32;
  static constexpr uint64_t MOVE_BITS = 16;
  static constexpr uint64_t DEPTH_BITS = 7;
  static constexpr uint64_t AGE_BITS = 7;
  static constexpr uint64_t TYPE_BITS = 2;

  static_assert(SCORE_BITS + MOVE_BITS + DEPTH_BITS + AGE_BITS + TYPE_BITS <=
                64);

  static constexpr uint64_t SCORE_SHIFT = 0;
  static constexpr uint64_t SCORE_MASK = (1ULL << SCORE_BITS) - 1;
  static constexpr uint64_t MOVE_SHIFT = SCORE_SHIFT + SCORE_BITS;
  static constexpr uint64_t MOVE_MASK = (1ULL << MOVE_BITS) - 1;
  static constexpr uint64_t DEPTH_SHIFT = MOVE_SHIFT + MOVE_BITS;
  static constexpr uint64_t DEPTH_MASK = (1ULL << DEPTH_BITS) - 1;
  static constexpr uint64_t AGE_SHIFT = DEPTH_SHIFT + DEPTH_BITS;
  static constexpr uint64_t AGE_MASK = (1ULL << AGE_BITS) - 1;
  static constexpr uint64_t TYPE_SHIFT = AGE_SHIFT + AGE_BITS;
  static constexpr uint64_t TYPE_MASK = (1ULL << TYPE_BITS) - 1;

  uint64_t data_ = 0;
};

class TranspositionTable {
 public:
  TranspositionTable(uint64_t size_in_mb);

  std::optional<TTData> operator[](uint64_t hash) const;

  void add(uint64_t hash, int32_t score, Move move, uint8_t depth, uint8_t age,
           TTData::Type type);
  // Use only for tests to make them more readable.
  void add(uint64_t hash, TTData data);

 private:
  struct TTEntry {
    std::atomic<uint64_t> hash = 0;
    std::atomic<uint64_t> data = 0;

    void write(uint64_t hash, uint64_t data);
  };

  std::unique_ptr<TTEntry[]> entries_;
  uint64_t entries_size_;
};

}  // namespace chess