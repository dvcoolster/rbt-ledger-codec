# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]
### Added
- FlowNet coupling blocks with additive/affine transformations
- Invertible flow architecture with 4 levels, depth 4
- Comprehensive roundtrip tests for coupling block invertibility
- FiLM (Feature-wise Linear Modulation) phase-tag conditioning
- 256-entry lookup table for 8-bit phase tag conditioning
- Tests verifying different phase tags produce different outputs
- ANS-X comprehensive design document with 16-bit symbol space specification
- RBT parity-based block skipping optimization design
- x86_64 AVX2 SIMD implementation plan with scalar fallback
- ANX1 chunk format specification for RBT2 container integration

## [v0.2.0] (2025-06-14) - Complete Multi-Language Codec
### Added
- Cross-platform CLI encode/decode demo with rzp tool
- Static ANS stub linked with C FFI integration
- CI matrix green on Linux/macOS/Windows with artifact publishing
- Added demo_walkthrough.md and results stub documentation
- Comprehensive documentation walkthrough with architecture details
- Complete data flow documentation (PNG → RLE → ANS → .rbt → PNG)
- Development workflow and contribution guidelines
- Performance targets and current status overview
- Demo artifact upload (PNG + RBT files) from Linux CI

### Fixed
- Fixed ledgerizer build failure on GCC 13 (missing <limits> include)
- Fixed Windows MSVC build with portable static library paths
- Fixed Windows linking by adding required system libraries (ws2_32, userenv, ntdll)
- Fixed multi-config generator support with proper TARGET_FILE usage in tests
- Fixed README badge URL to point to correct repository
- Enhanced CI with proper Windows build configuration
- Fixed Windows CI shell syntax by using PowerShell for duplicate test guard

## [v0.2.0-rc3] - CI Build Directory Fix
### Fixed
- Fixed CI build directory mismatch causing red builds
- Added BUILD_DIR environment variable to workflow
- Enhanced duplicate test guard with skip behavior for missing directories
- Added proper CMake configure and build steps for native targets

## [v0.2.0-rc2] - WASM Build & Artifacts
### Added
- WASM32 build target for browser usage (rbtcli.wasm)
- GitHub Actions artifact upload for WASM binaries
- Enhanced CI matrix with native vs WASM build separation

## [v0.2.0-rc1] - CI Green
### Fixed
- Fixed CI duplicate-test guard and Rust E0133 lints
- Fixed CMake RZP_BIN_PATH generator expression issue
- Added missing hex2bin.py script for test data conversion
- Added missing run_test.cmake script for CLI roundtrip testing

## [v0.2.0] - ANS-X Stub Integration
### Added
- `ansx` Rust crate with identity encode/decode functions and C FFI interface
- Integration of ansx static library into rzp CLI via CMake
- Comprehensive FFI roundtrip tests
- Rust 2024 edition compatibility with proper unsafe handling
- Updated CI workflow to build Rust components

## [v0.1.1] - CLI Skeleton
### Added
- `rzp` command-line tool capable of encoding a PNG into an `.rbt` container and decoding back to byte-identical PNG using Ledgerizer L0.
- Round-trip unit test (`rzp/tests/cli_roundtrip.cpp`).
- Integrated new target into root CMake build & CI matrices. 