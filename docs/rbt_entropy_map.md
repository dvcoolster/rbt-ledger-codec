# RBT-Aware Entropy Layer Specification

## Overview

This document specifies the entropy coding layer that integrates RBT (Recursive Binary Tree) axioms with adaptive rANS encoding. The design ensures that entropy allocation respects RBT's δ-tick ledger system and leverages even-loop parity for optimal compression.

## Core Axioms

### Axiom 1 – δ-Tick Ledger

Every symbol budget equals the *minimum* action-tick count required to represent the difference between even loops.

**Mathematical Formulation:**
```
bits_B = Σ_i ceil(log₂ Δticks_i)
```

Where:
- `bits_B` = total bits allocated for block B
- `Δticks_i` = tick difference for symbol i between consecutive even loops
- The sum covers all symbols in the block

**Implementation Constraint:**
The entropy coder MUST verify that the actual bits used matches this theoretical minimum within ±1 bit tolerance (due to rounding).

### Axiom 2 – Even-Loop Parity

Blocks where the curvature ledger is parity-even can be represented by *zero* information (skip flag in header).

**Parity Calculation:**
```
parity = (Σ_i curvature_i) mod 2
```

**Encoding Rule:**
```
if parity == 0:  // even parity
    write header with parity_even_flag = 1
    omit bitstream entirely
else:           // odd parity  
    write header with parity_even_flag = 0
    encode full bitstream with rANS
```

**Decoding Rule:**
```
if parity_even_flag == 1:
    reproduce block of zeros (or last known state)
else:
    decode bitstream normally
```

### Axiom 3 – Curvature ⇒ σ

FlowNet outputs μ/σ per latent; σ encodes local curvature. The adaptive coder must build cumulative frequency table using Gaussian CDF.

**Frequency Table Construction:**
```
f(x) = round(65536 * Φ((x-μ)/σ))
```

Where:
- `Φ` = standard normal CDF
- `x` = symbol value (0-65535 for 16-bit symbols)
- `μ, σ` = FlowNet-derived parameters
- `f(x)` = cumulative frequency, clamped to minimum ε = 1

**Normalization:**
```
total_freq = 65536
freq[i] = max(f(i) - f(i-1), 1)  // ensure non-zero frequencies
```

## Header Specification (RBT2/ANX1)

### Container Integration

The ANS-X entropy layer integrates into RBT2 containers as chunk type `"ANX1"`.

### Header Format

```
Offset  Size  Field               Description
------  ----  -----               -----------
0x00    2     block_size          Always 16384 (16 KB blocks)
0x02    4     cmpr_len            Compressed bitstream length
0x06    2     parity_flags        Bit 15: parity_even_flag
                                  Bits 14-0: reserved (must be 0)
0x08    var   freq_table_data     Compressed frequency tables (ζ-coded)
var     var   bitstream           rANS-encoded symbols (if parity_even_flag = 0)
```

### Frequency Table Compression

Frequency tables are compressed using ζ-code (zeta coding) to minimize header overhead:

```
ζ-code(freq):
    if freq == 1:     emit "0"
    else:             emit "1" + binary(freq-1)
```

### Parity Even Flag

- **Bit 15 = 1**: Block has even parity, bitstream omitted
- **Bit 15 = 0**: Block has odd parity, full bitstream follows

## Algorithmic Flow

### Encoding Algorithm

```python
def encode_block(symbols, mu, sigma):
    # Step 1: Compute curvature parity
    curvature_sum = sum(compute_curvature(s) for s in symbols)
    parity_even = (curvature_sum % 2) == 0
    
    # Step 2: Verify δ-tick constraint
    delta_ticks = compute_delta_ticks(symbols)
    theoretical_bits = sum(ceil(log2(dt)) for dt in delta_ticks)
    
    if parity_even:
        # Axiom 2: Even parity blocks encode to header only
        header = create_header(block_size=16384, 
                              cmpr_len=0, 
                              parity_even_flag=True)
        return header  # No bitstream
    else:
        # Axiom 3: Build Gaussian frequency table
        freq_table = build_gaussian_freq_table(mu, sigma)
        
        # Encode with rANS
        bitstream = rans_encode(symbols, freq_table)
        
        # Axiom 1: Verify bit budget
        actual_bits = len(bitstream) * 8
        assert abs(actual_bits - theoretical_bits) <= 1
        
        # Compress frequency table
        compressed_freq = zeta_encode(freq_table)
        
        header = create_header(block_size=16384,
                              cmpr_len=len(bitstream),
                              parity_even_flag=False)
        
        return header + compressed_freq + bitstream
```

### Decoding Algorithm

```python
def decode_block(data):
    header = parse_header(data)
    
    if header.parity_even_flag:
        # Axiom 2: Even parity reproduces zeros
        return zeros(header.block_size)
    else:
        # Extract compressed frequency table and bitstream
        freq_data = data[8:8+freq_table_size]
        bitstream = data[8+freq_table_size:8+freq_table_size+header.cmpr_len]
        
        # Decompress frequency table
        freq_table = zeta_decode(freq_data)
        
        # Decode with rANS
        symbols = rans_decode(bitstream, freq_table)
        
        return symbols
```

## Implementation Requirements

### Core rANS Implementation

1. **Symbol Space**: 16-bit integers (0-65535)
2. **State Size**: 32-bit ANS state with 24-bit lower bound
3. **Renormalization**: Emit bytes when state exceeds safe threshold
4. **Frequency Precision**: 16-bit cumulative frequencies (65536 total)

### δ-Tick Verification

Every encoding operation MUST include:

```rust
fn verify_delta_tick_constraint(symbols: &[u16], actual_bits: usize) -> bool {
    let delta_ticks: Vec<u32> = compute_delta_ticks(symbols);
    let theoretical_bits: usize = delta_ticks.iter()
        .map(|&dt| (dt as f64).log2().ceil() as usize)
        .sum();
    
    (actual_bits as i32 - theoretical_bits as i32).abs() <= 1
}
```

### Parity Computation

```rust
fn compute_curvature_parity(symbols: &[u16]) -> bool {
    let curvature_sum: u64 = symbols.iter()
        .map(|&s| compute_local_curvature(s))
        .sum();
    
    (curvature_sum % 2) == 0
}

fn compute_local_curvature(symbol: u16) -> u32 {
    // Simplified curvature: second derivative approximation
    // In practice, this would use FlowNet's curvature computation
    let x = symbol as f64 / 65535.0;
    let curvature = (x * (1.0 - x) * 65535.0) as u32;
    curvature
}
```

### SIMD Optimization Points

1. **Frequency Table Construction**: Vectorize Gaussian CDF computation
2. **Symbol Encoding**: Batch process multiple symbols per iteration
3. **Parity Computation**: Use SIMD reduction for curvature sum

## Testing Requirements

### Unit Tests

1. **Axiom 1 Verification**: 
   - Generate random 16KB blocks
   - Verify `actual_bits ≈ Σ ceil(log₂ Δticks_i)`

2. **Axiom 2 Validation**:
   - Create blocks with known even parity
   - Verify encoding produces header-only output (≤10 bytes)

3. **Axiom 3 Compliance**:
   - Test Gaussian frequency table construction
   - Verify CDF normalization to 65536

### Integration Tests

1. **FlowNet Pipeline**: End-to-end test with actual FlowNet μ/σ outputs
2. **Container Integration**: Verify ANX1 chunks in RBT2 containers
3. **Cross-Platform**: Test scalar and SIMD paths produce identical results

## Performance Targets

- **Encoding Speed**: ≥50 MB/s (scalar), ≥200 MB/s (AVX2)
- **Decoding Speed**: ≥100 MB/s (scalar), ≥400 MB/s (AVX2)
- **Compression Ratio**: ≥2.85× over raw FlowNet latents
- **Even-Parity Savings**: 99%+ compression for even-parity blocks

## Future Extensions

1. **GPU Kernels**: CUDA/OpenCL implementation for high-throughput scenarios
2. **ARM NEON**: SIMD optimization for ARM64 platforms
3. **Adaptive Block Size**: Dynamic block sizing based on curvature analysis
4. **Multi-Threading**: Parallel encoding of independent blocks

## References

1. Duda, J. "Asymmetric Numeral Systems: entropy coding combining speed of Huffman coding with compression rate of arithmetic coding"
2. RBT Specification v2.0 - Container Format
3. FlowNet Architecture Documentation - μ/σ Parameter Generation 