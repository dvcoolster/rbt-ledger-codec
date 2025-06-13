//! α-Flow entropy coder traits and placeholder implementation.

/// Trait for types that can encode/decode a byte slice using α-Flow.
///
/// Implementations must be *loss-less*: `decode(encode(data)) == data`.
pub trait AlphaFlowEncode {
    /// Encode raw bytes into compressed representation.
    fn encode(&self, input: &[u8]) -> Vec<u8>;

    /// Decode from compressed representation into raw bytes.
    fn decode(&self, compressed: &[u8]) -> Vec<u8>;
}

/// No-op reference implementation used for unit tests.
#[derive(Default)]
pub struct NoopCoder;

impl AlphaFlowEncode for NoopCoder {
    fn encode(&self, input: &[u8]) -> Vec<u8> {
        input.to_vec()
    }

    fn decode(&self, compressed: &[u8]) -> Vec<u8> {
        compressed.to_vec()
    }
} 