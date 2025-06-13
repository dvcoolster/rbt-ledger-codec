# Changelog

All notable changes to this project will be documented in this file.

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