#include <cstdlib>
#include <fstream>
#include <vector>
#include <cassert>
#include <iostream>

#ifndef RZP_BIN_PATH
#define RZP_BIN_PATH "rzp"
#endif

static const unsigned char PNG_1x1[] = {
    0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A,0x00,0x00,0x00,0x0D,0x49,0x48,0x44,0x52,
    0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x08,0x06,0x00,0x00,0x00,0x1F,0x15,0xC4,
    0x89,0x00,0x00,0x00,0x0A,0x49,0x44,0x41,0x54,0x78,0x9C,0x63,0x60,0x00,0x00,0x00,
    0x02,0x00,0x01,0xE2,0x21,0xBC,0x33,0x00,0x00,0x00,0x00,0x49,0x45,0x4E,0x44,0xAE,
    0x42,0x60,0x82
};

int main() {
    const char* in_png = "test_input.png";
    const char* out_rbt = "test_output.rbt";
    const char* out_png = "test_roundtrip.png";

    std::ofstream ofs(in_png, std::ios::binary);
    ofs.write(reinterpret_cast<const char*>(PNG_1x1), sizeof(PNG_1x1));
    ofs.close();

    std::string cmd_encode = std::string(RZP_BIN_PATH) + " encode " + in_png + " " + out_rbt;
    std::string cmd_decode = std::string(RZP_BIN_PATH) + " decode " + out_rbt + " " + out_png;

    assert(std::system(cmd_encode.c_str()) == 0);

    // Corrupt one byte in the container
    std::fstream cfile(out_rbt, std::ios::in | std::ios::out | std::ios::binary);
    cfile.seekp(16); // somewhere in the payload
    char b;
    cfile.read(&b, 1);
    cfile.seekp(16);
    b ^= 0xFF; // flip bits
    cfile.write(&b, 1);
    cfile.close();

    int ret = std::system(cmd_decode.c_str());
    assert(ret != 0 && "Decoder should fail on corrupted file");

    std::cout << "Integrity check test passed\n";
    return 0;
}
