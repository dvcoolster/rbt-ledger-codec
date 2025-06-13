# Ledgerizer – Even/Odd Loop Graph Compressor (Prototype)

This C++17 library provides a **minimal reference implementation** of the *ledger* layer (L0) described in the RBT-Ledger-Codec roadmap.

It shows how raw byte streams can be mapped to an **invertible even/odd loop graph** representation and back.  For the very first prototype we treat *consecutive identical bytes* as an "even loop" and encode them as simple `(count, value)` runs (a form of run-length coding).  The goal is **correctness and round-trip fidelity** rather than high compression – more advanced loop parsers will follow.

## Build (macOS/Linux)

```bash
# Inside the workspace root
mkdir -p build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . -j $(nproc)
ctest
```

The build produces:

* `libledgerizer.a` – static library
* `ledgerizer-bin` – CLI utility
* `ledgerizer_tests` – unit tests

## CLI usage

```bash
# Compress
./ledgerizer-bin c input.raw output.led

# Decompress
./ledgerizer-bin d output.led roundtrip.raw
```

`roundtrip.raw` should be byte-identical to `input.raw`.

## Next steps

1. Replace the trivial run-length parser with a *true even/odd loop* detector that handles:
   * Mirror macro-blocks in RGB / YUV frames
   * Symmetric DCT coefficients
   * Palindromic strings / patterns
2. Expose a **C API** so the codec can be plugged into FFmpeg.
3. Add **SIMD-accelerated** encode/decode paths.
4. Provide Python bindings via **pybind11** for rapid experimentation. 