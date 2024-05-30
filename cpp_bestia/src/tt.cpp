#include "tt.hpp"

namespace chess {

TTData::TTData(uint64_t data) : data_(data) {}

int32_t TTData::score() const { return data_ & SCORE_MASK; }

Move TTData::move() const { return (data_ >> MOVE_SHIFT) & MOVE_MASK; }

uint8_t TTData::depth() const { return (data_ >> DEPTH_SHIFT) & DEPTH_MASK; }

uint8_t TTData::age() const { return (data_ >> DEPTH_SHIFT) & DEPTH_MASK; }

TTData::Type TTData::type() const {
  return static_cast<Type>((data_ >> DEPTH_SHIFT) & DEPTH_MASK);
}

uint64_t TTData::pack(int32_t score, Move move, uint8_t depth, uint8_t age,
                      Type type) {
  return (static_cast<uint64_t>(score) << SCORE_SHIFT) |
         (static_cast<uint64_t>(move.move()) << MOVE_SHIFT) |
         (static_cast<uint64_t>(depth) << DEPTH_SHIFT) |
         (static_cast<uint64_t>(age) << AGE_SHIFT) |
         (static_cast<uint64_t>(type) << TYPE_SHIFT);
}

TranspositionTable::TranspositionTable(uint64_t size_in_mb) {
  entries_.resize(1024 * 1024 * size_in_mb / sizeof(TTEntry));
}

std::optional<TTData> TranspositionTable::operator[](uint64_t hash) const {
  const uint64_t index = hash % entries_.size();

  auto& entry = entries_[index];
  const uint64_t entry_hash = entry.hash.load(std::memory_order_relaxed);
  const uint64_t entry_data = entry.data.load(std::memory_order_relaxed);

  return ((entry_hash ^ entry_data) == hash ? std::optional{TTData(entry_data)}
                                            : std::nullopt);
}

void TranspositionTable::add(uint64_t hash, int32_t score, Move move,
                             uint8_t depth, uint8_t age, TTData::Type type) {
  const uint64_t index = hash % entries_.size();

  auto& old_entry = entries_[index];
  const uint64_t old_hash = old_entry.hash.load(std::memory_order_relaxed);

  if (old_hash == 0) {
    old_entry.write(hash, TTData::pack(score, move, depth, age, type));
    return;
  }

  auto old_data = TTData(old_entry.data.load(std::memory_order_relaxed));
  if (old_data.age() < age ||
      (old_data.age() == age && old_data.depth() < depth)) {
    old_entry.write(hash, TTData::pack(score, move, depth, age, type));
  }
}

void TranspositionTable::TTEntry::write(uint64_t hash, uint64_t data) {
  this->hash.store(hash ^ data, std::memory_order_relaxed);
  this->data.store(data, std::memory_order_relaxed);
}

}  // namespace chess