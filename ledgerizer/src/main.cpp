#include "ledgerizer/ledgerizer.h"

#include <fstream>
#include <iostream>
#include <iterator>
#include <string>

namespace {

std::vector<uint8_t> read_file(const std::string& path) {
    std::ifstream ifs(path, std::ios::binary);
    if (!ifs) {
        throw std::runtime_error("Failed to open file: " + path);
    }
    return std::vector<uint8_t>(std::istreambuf_iterator<char>(ifs), {});
}

void write_file(const std::string& path, const std::vector<uint8_t>& data) {
    std::ofstream ofs(path, std::ios::binary);
    if (!ofs) {
        throw std::runtime_error("Failed to write file: " + path);
    }
    ofs.write(reinterpret_cast<const char*>(data.data()), data.size());
}

void print_usage(const char* exe) {
    std::cerr << "Usage: " << exe << " [c|d] <input> <output>\n";
    std::cerr << "  c: compress (encode)\n";
    std::cerr << "  d: decompress (decode)\n";
}

} // unnamed namespace

int main(int argc, char** argv) {
    if (argc != 4) {
        print_usage(argv[0]);
        return 1;
    }
    std::string mode = argv[1];
    std::string input_path = argv[2];
    std::string output_path = argv[3];

    try {
        if (mode == "c") {
            auto data = read_file(input_path);
            auto ledger = ledgerizer::encode(data);
            auto buffer = ledgerizer::serialize(ledger);
            write_file(output_path, buffer);
        } else if (mode == "d") {
            auto buffer = read_file(input_path);
            auto ledger = ledgerizer::deserialize(buffer);
            auto data = ledgerizer::decode(ledger);
            write_file(output_path, data);
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