#pragma once

#include <cstdint>
#include <vector>
#include <utility>

namespace ledgerizer {

// A simple representation of a ledger (even/odd loop graph) for demonstration.
// For the prototype we encode runs of duplicated bytes as pairs (count, value).
using Run = std::pair<uint32_t, uint8_t>; // (count, value)

struct Ledger {
    std::vector<Run> runs;
};

// Convert raw bytes into a Ledger representation (compression).
// The algorithm groups consecutive identical bytes (even loops) together.
Ledger encode(const std::vector<uint8_t>& data);

// Reconstruct original bytes from a Ledger (decompression).
std::vector<uint8_t> decode(const Ledger& ledger);

// Helper utilities to read/write Ledger to a binary buffer so that it can be stored to disk.
std::vector<uint8_t> serialize(const Ledger& ledger);
Ledger deserialize(const std::vector<uint8_t>& buffer);

} // namespace ledgerizer 