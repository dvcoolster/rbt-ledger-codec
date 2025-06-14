#include "ledgerizer/ledgerizer.h"
#include "ansx.h"
#include "crypto_utils.h"

#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>
#include <cstdint>
#include <cstring>

namespace {

constexpr const char MAGIC[4] = {'R', 'B', 'T', '1'};

void write_file(const std::string& path, const std::vector<uint8_t>& data) {
    std::ofstream ofs(path, std::ios::binary);
    if (!ofs) {
        throw std::runtime_error("Failed to write file: " + path);
    }
    ofs.write(reinterpret_cast<const char*>(data.data()), data.size());
}

std::vector<uint8_t> read_file(const std::string& path) {
    std::ifstream ifs(path, std::ios::binary);
    if (!ifs) {
        throw std::runtime_error("Failed to open file: " + path);
    }
    return std::vector<uint8_t>(std::istreambuf_iterator<char>(ifs), {});
}

std::vector<uint8_t> encode_container(const std::vector<uint8_t>& raw) {
    auto ledger = ledgerizer::encode(raw);
    auto ser = ledgerizer::serialize(ledger);

    uint32_t ansx_len = 0;
    uint8_t* ansx_ptr = ansx_encode(ser.data(), static_cast<uint32_t>(ser.size()), &ansx_len);
    std::vector<uint8_t> ansx_bytes(ansx_ptr, ansx_ptr + ansx_len);
    ansx_free(ansx_ptr, ansx_len);

    uint32_t crc = rzp::crc32(ansx_bytes);
    rzp::Sha256 hasher;
    hasher.update(raw.data(), raw.size());
    auto digest = hasher.finish();

    uint32_t len = static_cast<uint32_t>(ansx_bytes.size());
    std::vector<uint8_t> out;
    out.reserve(4 + 8 + len + digest.size());
    out.insert(out.end(), MAGIC, MAGIC + 4);
    for (int i = 0; i < 4; ++i) {
        out.push_back(static_cast<uint8_t>((len >> (i * 8)) & 0xFF));
    }
    for (int i = 0; i < 4; ++i) {
        out.push_back(static_cast<uint8_t>((crc >> (i * 8)) & 0xFF));
    }
    out.insert(out.end(), ansx_bytes.begin(), ansx_bytes.end());
    out.insert(out.end(), digest.begin(), digest.end());
    return out;
}

std::vector<uint8_t> decode_container(const std::vector<uint8_t>& container) {
    if (container.size() < 8 || std::memcmp(container.data(), MAGIC, 4) != 0) {
        throw std::runtime_error("Invalid container: bad magic");
    }
    uint32_t len = 0;
    for (int i = 0; i < 4; ++i) {
        len |= static_cast<uint32_t>(container[4 + i]) << (i * 8);
    }
    uint32_t crc = 0;
    for (int i = 0; i < 4; ++i) {
        crc |= static_cast<uint32_t>(container[8 + i]) << (i * 8);
    }
    size_t offset = 12;
    if (container.size() < offset + len + 32) {
        throw std::runtime_error("Invalid container: length mismatch");
    }
    std::vector<uint8_t> ansx_bytes(container.begin() + offset,
                                    container.begin() + offset + len);
    uint32_t crc_calc = rzp::crc32(ansx_bytes);
    if (crc != crc_calc) {
        throw std::runtime_error("CRC mismatch");
    }
    uint32_t ser_len = 0;
    uint8_t* ser_ptr = ansx_decode(ansx_bytes.data(), len, &ser_len);
    std::vector<uint8_t> ser(ser_ptr, ser_ptr + ser_len);
    ansx_free(ser_ptr, ser_len);
    auto ledger = ledgerizer::deserialize(ser);
    auto raw = ledgerizer::decode(ledger);
    rzp::Sha256 hasher;
    hasher.update(raw.data(), raw.size());
    auto digest = hasher.finish();
    std::array<uint8_t,32> stored{};
    std::copy(container.begin() + offset + len, container.begin() + offset + len + 32, stored.begin());
    if (!std::equal(digest.begin(), digest.end(), stored.begin())) {
        throw std::runtime_error("SHA-256 mismatch");
    }
    return raw;
}

void print_usage(const char* exe) {
    std::cerr << "Usage: " << exe << " encode <in.png> <out.rbt>\n";
    std::cerr << "       " << exe << " decode <in.rbt> <out.png>\n";
}

} // namespace

int main(int argc, char** argv) {
    if (argc != 4) {
        print_usage(argv[0]);
        return 1;
    }
    std::string mode = argv[1];
    std::string in = argv[2];
    std::string out = argv[3];

    try {
        if (mode == "encode") {
            auto raw = read_file(in);
            auto cont = encode_container(raw);
            write_file(out, cont);
        } else if (mode == "decode") {
            auto cont = read_file(in);
            auto raw = decode_container(cont);
            write_file(out, raw);
        } else {
            print_usage(argv[0]);
            return 1;
        }
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }
    return 0;
} 