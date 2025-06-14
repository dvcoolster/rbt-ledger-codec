# ANS-X Adaptive Entropy Coder Design

## 1. Objective

Encode FlowNet latents (approximately Gaussian distributed) into rANS bit-stream to achieve compression efficiency within 1% of theoretical optimum. The encoder targets:

```
bits ≈ −∑ log₂ P(x)
```

Where P(x) is the probability mass function derived from FlowNet's learned distributions.

**Target Performance:**
- Compression ratio: ≥2× over raw FlowNet latents
- Encoding speed: ≥50 MB/s on modern x86_64 CPUs
- Decoding speed: ≥100 MB/s on modern x86_64 CPUs
- Memory overhead: ≤16 KB per encoding thread

## 2. Model

### 2.1 Block Structure
- **Block size**: 16 KB (16,384 bytes)
- **Symbol alphabet**: 16-bit integers (0-65535) for range coder compatibility
- **Adaptive window**: Statistics computed per block for optimal local compression

### 2.2 Probability Mass Function

FlowNet outputs provide μ (mean) and σ (standard deviation) parameters per block. The probability mass function is computed as:

```
p_i = max(exp(−0.5 * ((i - μ) / σ)²), ε)
```

Where:
- `i` ∈ [0, 65535] (16-bit symbol space)
- `μ` = block mean from FlowNet
- `σ` = block standard deviation from FlowNet  
- `ε = 1` = minimum probability to ensure non-zero mass for all symbols

**Normalization**: After computing raw probabilities, renormalize to sum to 2¹⁶ (65536) for efficient 16-bit arithmetic.

### 2.3 Cumulative Frequency Table

Build cumulative frequency table `C[i]` where:
```
C[i] = ∑(j=0 to i) freq[j]
```

This enables O(log n) symbol lookup during decoding via binary search.

## 3. RBT Twist - Parity-Based Block Skipping

### 3.1 Even-Loop Parity Check

For blocks where the cumulative log-determinant has even parity:
```
parity = (⌊∑ log₂(freq[symbol])⌋) mod 2
```

If `parity == 0` (even), the block can be **omitted** from the compressed stream. The ledger records this optimization rule, enabling reconstruction during decode.

### 3.2 Size Savings

This optimization targets highly regular regions (e.g., smooth gradients, solid colors) where FlowNet produces near-uniform distributions. Expected savings: 10-30% on natural images.

## 4. Vectorization Plan

### 4.1 x86_64 AVX2 Implementation

**Encoding Loop:**
- Vectorized probability computation (8x float32 SIMD)
- Parallel frequency table updates
- SIMD-accelerated renormalization

**Decoding Loop:**
- Vectorized binary search for symbol lookup
- Parallel state updates for multiple symbols
- Prefetch optimization for memory access patterns

### 4.2 ARM NEON (Future)

Placeholder implementation using NEON intrinsics for mobile/embedded targets. Initial focus on scalar correctness.

### 4.3 Scalar Fallback

Pure Rust implementation without SIMD for:
- CI compatibility (macOS runners)
- Non-x86_64 architectures
- Debug builds

**Function Multiversioning:**
```rust
#[target_feature(enable = "avx2")]
unsafe fn encode_avx2(data: &[u8]) -> Vec<u8> { ... }

fn encode_scalar(data: &[u8]) -> Vec<u8> { ... }

pub fn encode(data: &[u8]) -> Vec<u8> {
    if is_x86_feature_detected!("avx2") {
        unsafe { encode_avx2(data) }
    } else {
        encode_scalar(data)
    }
}
```

## 5. Container Integration

### 5.1 New Chunk Format

Introduce chunk ID `"ANX1"` within RBT2 container format:

```
ANX1 Chunk Layout:
+------------------+
| chunk_id: "ANX1" | 4 bytes
| chunk_len: u32   | 4 bytes  
| block_size: u16  | 2 bytes (typically 16384)
| cmpr_len: u32    | 4 bytes (compressed data length)
| parity_flag: u8  | 1 byte  (bit 0: parity optimization enabled)
| reserved: [u8;3] | 3 bytes (future extensions)
| compressed_data  | cmpr_len bytes
+------------------+
```

### 5.2 Ledger Integration

The ledger records:
- Block boundaries and their μ/σ parameters
- Parity skip decisions for reconstruction
- Checksum for integrity verification

## 6. Testing & Benchmark

### 6.1 Unit Tests

**Roundtrip Correctness:**
```rust
#[test]
fn test_roundtrip_random_64kb() {
    let data: Vec<u8> = (0..65536).map(|_| rand::random()).collect();
    let encoded = ansx_encode(&data);
    let decoded = ansx_decode(&encoded);
    assert_eq!(data, decoded);
}
```

**Performance Benchmarks:**
- Encoding speed on random data
- Decoding speed on various distributions
- Memory usage profiling

### 6.2 Integration Tests

**FlowNet Latent Compression:**
After FlowNet training checkpoint is merged:

```rust
#[test]
fn test_flownet_latent_compression() {
    let latents = load_flownet_test_data();
    let encoded = ansx_encode_with_flownet_params(&latents);
    
    // Verify compression ratio
    let ratio = latents.len() as f32 / encoded.len() as f32;
    assert!(ratio >= 2.85); // Target: ≤35% of original size
    
    // Verify correctness
    let decoded = ansx_decode(&encoded);
    assert_eq!(latents, decoded);
}
```

### 6.3 Benchmark Targets

**Compression Efficiency:**
- Target: ≤35% of raw FlowNet latent size
- Baseline: Compare against gzip, brotli, zstd on same data

**Speed Targets:**
- Encoding: ≥50 MB/s (x86_64 AVX2)
- Decoding: ≥100 MB/s (x86_64 AVX2)
- Scalar fallback: ≥20 MB/s encoding, ≥40 MB/s decoding

## 7. Implementation Phases

### Phase A: Core rANS (`feature/ansx-core`)
- Scalar rANS state machine
- C FFI interface for cross-language compatibility
- Basic probability modeling
- Unit tests for all platforms

### Phase B: SIMD Optimization (`feature/ansx-simd-avx2`)
- AVX2 vectorized encode/decode
- Runtime feature detection
- Performance benchmarks
- CI integration with `RUSTFLAGS="-C target-cpu=native"`

### Phase C: RBT Integration (`feature/ansx-ledger-parity`)
- Parity-based block skipping
- ANX1 chunk format implementation
- Ledger annotation logic
- Container specification updates

## 8. Success Criteria

**Functional Requirements:**
- [ ] Roundtrip correctness on all test cases
- [ ] Cross-platform compatibility (Linux, macOS, Windows)
- [ ] Integration with existing RBT container format

**Performance Requirements:**
- [ ] Compression ratio ≥2.85× on FlowNet latents
- [ ] Encoding speed ≥50 MB/s (AVX2)
- [ ] Decoding speed ≥100 MB/s (AVX2)
- [ ] Memory overhead ≤16 KB per thread

**Quality Requirements:**
- [ ] Zero memory leaks (valgrind clean)
- [ ] Thread-safe implementation
- [ ] Comprehensive error handling
- [ ] Documentation coverage ≥90%

## 9. Risk Mitigation

**Technical Risks:**
- **SIMD portability**: Maintain scalar fallback for all code paths
- **Numerical stability**: Use double precision for probability computations
- **Memory alignment**: Ensure proper alignment for SIMD operations

**Integration Risks:**
- **Container format changes**: Maintain backward compatibility with RBT1
- **FlowNet coupling**: Design interface to be agnostic to FlowNet internals
- **CI complexity**: Isolate SIMD tests to specific runners

**Performance Risks:**
- **Cache efficiency**: Profile memory access patterns
- **Branch prediction**: Minimize conditional branches in hot paths
- **Compiler optimization**: Verify release builds achieve target performance 