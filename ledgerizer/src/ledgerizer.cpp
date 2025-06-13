#include "ledgerizer/ledgerizer.h"

#include <stdexcept>

namespace ledgerizer {

Ledger encode(const std::vector<uint8_t>& data) {
    Ledger ledger;
    if (data.empty()) {
        return ledger;
    }

    uint8_t current = data[0];
    uint32_t count = 1;

    for (size_t i = 1; i < data.size(); ++i) {
        if (data[i] == current && count < std::numeric_limits<uint32_t>::max()) {
            ++count;
        } else {
            ledger.runs.emplace_back(count, current);
            current = data[i];
            count = 1;
        }
    }
    ledger.runs.emplace_back(count, current);
    return ledger;
}

std::vector<uint8_t> decode(const Ledger& ledger) {
    std::vector<uint8_t> data;
    for (const auto& [count, value] : ledger.runs) {
        data.insert(data.end(), count, value);
    }
    return data;
}

std::vector<uint8_t> serialize(const Ledger& ledger) {
    // Serialize as: [num_runs: uint32_t][count1:uint32][value1:uint8]...[countN][valueN]
    std::vector<uint8_t> buffer;
    uint32_t num_runs = static_cast<uint32_t>(ledger.runs.size());
    buffer.resize(sizeof(uint32_t) + num_runs * (sizeof(uint32_t) + sizeof(uint8_t)));

    size_t offset = 0;
    auto write32 = [&](uint32_t v) {
        for (int i = 0; i < 4; ++i) {
            buffer[offset++] = static_cast<uint8_t>((v >> (i * 8)) & 0xFF);
        }
    };
    write32(num_runs);
    for (const auto& [count, value] : ledger.runs) {
        write32(count);
        buffer[offset++] = value;
    }
    return buffer;
}

Ledger deserialize(const std::vector<uint8_t>& buffer) {
    Ledger ledger;
    if (buffer.size() < sizeof(uint32_t)) {
        throw std::runtime_error("Buffer too small to contain Ledger");
    }

    size_t offset = 0;
    auto read32 = [&](uint32_t& out) {
        out = 0;
        for (int i = 0; i < 4; ++i) {
            out |= static_cast<uint32_t>(buffer[offset++]) << (i * 8);
        }
    };

    uint32_t num_runs = 0;
    read32(num_runs);

    for (uint32_t i = 0; i < num_runs; ++i) {
        uint32_t count = 0;
        if (offset + 4 > buffer.size()) {
            throw std::runtime_error("Malformed Ledger buffer");
        }
        read32(count);
        if (offset >= buffer.size()) {
            throw std::runtime_error("Malformed Ledger buffer (value)");
        }
        uint8_t value = buffer[offset++];
        ledger.runs.emplace_back(count, value);
    }

    return ledger;
}

} // namespace ledgerizer 