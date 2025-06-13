# Changelog

All notable changes to this project will be documented in this file.

## [v0.1.1] - CLI Skeleton
### Added
- `rzp` command-line tool capable of encoding a PNG into an `.rbt` container and decoding back to byte-identical PNG using Ledgerizer L0.
- Round-trip unit test (`rzp/tests/cli_roundtrip.cpp`).
- Integrated new target into root CMake build & CI matrices. 