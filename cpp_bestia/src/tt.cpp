#include "tt.hpp"

namespace chess {

TranspositionTable::TranspositionTable(uint64_t size_in_mb) {
  entries_.resize(1024 * 1024 * size_in_mb / sizeof(TTEntry));
}

const TTEntry* TranspositionTable::operator[](uint64_t hash) const {
  const auto index = hash % entries_.size();
  return (entries_[index].type != TTEntry::Type::NONE ? &entries_[index]
                                                      : nullptr);
}

void TranspositionTable::add(uint64_t hash, int32_t score, Move move,
                             uint8_t depth, TTEntry::Type type) {
  const auto index = hash % entries_.size();
  auto& old_entry = entries_[index];

  if (old_entry.type == TTEntry::Type::NONE || old_entry.depth < depth) {
    old_entry = {score, move, depth, type};
  }
}

}  // namespace chess