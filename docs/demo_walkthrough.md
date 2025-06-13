# Demo Walkthrough - RBT Codec v0.2.0

**v0.2.0 provides a working lossless PNG compression pipeline with multi-language integration (C++17 + Rust) and cross-platform CI support.**

## Quick Demo

Here's a complete encode/decode roundtrip demonstration:

```bash
# Build the project
$ cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
$ cmake --build build --parallel

# Navigate to test directory
$ cd build/rzp/tests

# Encode a PNG file to RBT container
$ ../rzp encode cat.png demo.rbt
# Encoded 66 bytes → 272 bytes (with container overhead)

# Decode back to PNG
$ ../rzp decode demo.rbt roundtrip.png
# Decoding complete

# Verify byte-identical roundtrip
$ cmp cat.png roundtrip.png && echo "✅ Byte-identical!"
✅ Byte-identical!
```

## Container Format

The `.rbt` files use a custom container format with the "RBT1" magic header:

```bash
$ xxd -l 16 demo.rbt
00000000: 5242 5431 0801 0000 3400 0000 0100 0000  RBT1....4.......
```

**Header breakdown:**
- `52 42 54 31` = "RBT1" magic signature
- `08 01 00 00` = Version and flags
- `34 00 00 00` = Compressed data size (52 bytes)
- `01 00 00 00` = Number of chunks

## Current Implementation Status

### ✅ Working Components
- **Ledgerizer L0**: Run-length encoding compression layer (C++17)
- **ANS-X stub**: Identity pass-through with C FFI interface (Rust)
- **rzp CLI**: Complete encode/decode pipeline (C++)
- **Container format**: RBT1 header with proper serialization
- **Cross-platform builds**: Linux, macOS, Windows, WASM

### 🚧 Pass-Through Layers (v0.2.0)
- **FlowNet**: Currently bypassed (planned for S₁ sprint)
- **ANS entropy coding**: Identity function (planned for S₂ sprint)
- **Perceptual enhancement**: Not yet implemented (planned for S₃ sprint)

## Data Flow

```
PNG Input (66 bytes)
    ↓
Ledgerizer RLE (52 bytes compressed)
    ↓
ANS-X Identity (52 bytes unchanged)
    ↓
RBT1 Container (272 bytes with headers)
    ↓
Decode Pipeline (reverse)
    ↓
Identical PNG Output (66 bytes)
```

## Architecture Highlights

- **Multi-language integration**: Seamless C++/Rust interop via FFI
- **Static linking**: Rust libraries compiled to static `.a`/`.lib` files
- **Cross-platform**: CMake + Cargo workspace with unified CI
- **Test coverage**: Unit tests, integration tests, and CLI roundtrip verification

## Next Steps

The v0.2.0 foundation enables the next development sprints:
- **S₁**: FlowNet PyTorch invertible ResNet integration
- **S₂**: Real ANS entropy coding with adaptive tables
- **S₃**: Perceptual enhancement with dual-head processing

For detailed architecture information, see [walkthrough.md](walkthrough.md). 