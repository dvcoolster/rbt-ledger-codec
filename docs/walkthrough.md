# RBT Codec Walkthrough

This document provides a comprehensive walkthrough of the RBT (Reversible Binary Tree) codec implementation.

## 🏗️ Architecture Overview

The RBT codec is implemented as a multi-language project with the following components:

### Core Components

1. **`ledgerizer/`** (C++17) - L0 compression layer
   - Run-length encoding implementation
   - Byte-stream ↔ even/odd loop graph conversion
   - Static library with CMake build system

2. **`ansx/`** (Rust) - ANS-X compression stub
   - Identity encode/decode functions (placeholder for future ANS implementation)
   - C FFI interface for integration with C++ components
   - Comprehensive test suite

3. **`rzp/`** (C++) - CLI tool
   - Command-line interface for encoding/decoding
   - PNG ↔ .rbt container format support
   - Integration point for ledgerizer and ansx components

4. **`rbtcore/`** (Rust) - Core algorithms
   - Alpha flow module (placeholder)
   - Version management
   - Workspace foundation

5. **`rbtcli/`** (Rust) - CLI interface
   - Clap-based command-line parsing
   - WASM build target for browser usage

## 🔄 Data Flow

```
PNG Input → Ledgerizer (RLE) → ANS-X (identity) → .rbt Container → Decode → Identical PNG
```

### Encoding Process
1. **Input**: PNG file is read as byte stream
2. **Ledgerizer**: Applies run-length encoding to compress repeated bytes
3. **ANS-X**: Currently identity function (future: adaptive entropy coding)
4. **Container**: Wraps compressed data in .rbt format with "RBT1" magic header
5. **Output**: .rbt file ready for storage/transmission

### Decoding Process
1. **Input**: .rbt container file
2. **Container**: Extracts compressed data and validates magic header
3. **ANS-X**: Decodes data (currently identity function)
4. **Ledgerizer**: Applies run-length decoding to restore original bytes
5. **Output**: Byte-identical PNG file

## 🛠️ Build System

### Prerequisites
- **Rust** (1.87.0+) with Cargo
- **CMake** (3.10+)
- **C++17** compatible compiler (GCC 13+, Clang, MSVC)
- **Python 3** (for test utilities)

### Building

```bash
# Full workspace build (Rust + C++)
cargo test --workspace --release

# CMake build (C++ components)
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel

# Run all tests
cd build && ctest --output-on-failure
```

### CI/CD Pipeline

The project uses GitHub Actions with a matrix build:
- **Ubuntu** (x86_64-unknown-linux-gnu)
- **macOS** (x86_64-apple-darwin) 
- **Windows** (x86_64-pc-windows-msvc)
- **WASM** (wasm32-unknown-unknown) - for browser usage

Artifacts:
- WASM binary (`rbtcli.wasm`) uploaded for browser integration

## 🧪 Testing Strategy

### Unit Tests
- **Rust**: `cargo test` runs all crate tests
- **C++**: CTest integration with CMake
- **FFI**: Roundtrip tests between Rust and C++

### Integration Tests
- **CLI roundtrip**: PNG → .rbt → PNG with byte-identical verification
- **Cross-language**: C++ calls Rust FFI functions
- **Build verification**: All targets compile on all platforms

### Test Coverage
- `ledgerizer_basic`: Core RLE functionality
- `rzp_cli_roundtrip`: End-to-end CLI workflow
- `roundtrip_identity`: ANS-X FFI interface
- `version_not_empty`: Basic sanity checks

## 📁 File Structure

```
rzip/
├── ledgerizer/          # C++17 compression library
│   ├── include/         # Public headers
│   ├── src/             # Implementation
│   └── tests/           # Unit tests
├── ansx/                # Rust ANS-X stub
│   ├── src/lib.rs       # FFI interface
│   ├── include/ansx.h   # C header
│   └── tests/           # API tests
├── rzp/                 # C++ CLI tool
│   ├── src/rzp.cpp      # Main implementation
│   └── tests/           # CLI tests
├── rbtcore/             # Rust core algorithms
├── rbtcli/              # Rust CLI interface
├── docs/                # Documentation
├── .github/workflows/   # CI configuration
└── Cargo.toml           # Rust workspace
```

## 🔧 Development Workflow

### Adding New Features
1. Create feature branch: `git checkout -b feat/feature-name`
2. Implement changes with tests
3. Ensure all tests pass locally
4. Push and create PR
5. Wait for CI to pass on all platforms
6. Squash merge to main
7. Delete feature branch

### Branch Discipline
- `main`: Stable, always green CI
- `dev/*`: Feature branches
- `fix/*`: Bug fix branches
- All changes go through PR review

### Version Management
- Semantic versioning (MAJOR.MINOR.PATCH)
- CHANGELOG.md tracks all changes
- Git tags for releases

## 🎯 Current Status (v0.2.0)

### ✅ Completed
- [x] CLI skeleton with PNG roundtrip
- [x] Ledgerizer C++17 library
- [x] ANS-X Rust stub with C FFI
- [x] Multi-platform CI/CD
- [x] WASM build support
- [x] Comprehensive test suite

### 🚧 In Progress
- [ ] Real ANS entropy coding (S₂ sprint)
- [ ] FlowNet PyTorch integration (S₁ sprint)
- [ ] Perceptual enhancement (S₃ sprint)

### 🔮 Future
- [ ] FFmpeg plugin integration
- [ ] Browser demo with drag-drop
- [ ] Lean formal verification
- [ ] Performance benchmarking

## 📊 Performance Targets

| Metric | Current | Target |
|--------|---------|--------|
| Compression Ratio | 1:1 (identity) | 10-100x |
| Encode Speed | ~1 MB/s | >100 MB/s |
| Decode Speed | ~1 MB/s | >100 MB/s |
| Memory Usage | ~2x input | <1.5x input |

## 🤝 Contributing

1. **Setup**: Follow build instructions above
2. **Code Style**: 
   - Rust: `cargo fmt` and `cargo clippy`
   - C++: Follow existing style (C++17 standard)
3. **Testing**: All new code must have tests
4. **Documentation**: Update relevant docs
5. **CI**: All platforms must pass

## 📚 References

- [RBT Theory Paper](../specs/) (when available)
- [Compression Benchmarks](results.md)
- [API Documentation](../README.md)
- [Build Instructions](../README.md#quick-start) 