//! FlowNet: Invertible neural flow for RBT compression
//!
//! This crate provides a normalizing flow implementation for learned compression.
//! It supports no-std environments and optional tract integration for inference.

#![cfg_attr(not(feature = "std"), no_std)]

// Core imports
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
#[cfg(feature = "std")]
use std::boxed::Box;

use ndarray::Array3;

#[cfg(feature = "tract")]
use tract_onnx::prelude::*;

/// Error types for FlowNet operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowNetError {
    /// Invalid input dimensions
    InvalidDimensions,
    /// Model not loaded
    ModelNotLoaded,
    /// Tract inference error (only with tract feature)
    #[cfg(feature = "tract")]
    TractError,
}

/// Result type for FlowNet operations
pub type Result<T> = core::result::Result<T, FlowNetError>;

/// FlowNet model for invertible transformations
pub struct FlowNet {
    /// Number of flow levels
    levels: usize,
    /// Depth of coupling blocks per level
    depth: usize,
    /// Whether a model is loaded
    loaded: bool,
    /// Tract model (only with tract feature)
    #[cfg(feature = "tract")]
    model: Option<SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>>,
}

impl FlowNet {
    /// Create a new FlowNet instance
    pub fn new(levels: usize, depth: usize) -> Self {
        Self {
            levels,
            depth,
            loaded: false,
            #[cfg(feature = "tract")]
            model: None,
        }
    }

    /// Default configuration: 4 levels, depth 4
    pub fn default() -> Self {
        Self::new(4, 4)
    }

    /// Encode input data through the flow
    ///
    /// # Arguments
    /// * `input` - Input tensor of shape [C, H, W]
    /// * `phase_tag` - 8-bit phase conditioning tag
    ///
    /// # Returns
    /// Encoded latent representation
    pub fn encode(&self, input: &Array3<f32>, _phase_tag: u8) -> Result<Array3<f32>> {
        if !self.loaded {
            return Err(FlowNetError::ModelNotLoaded);
        }
        
        // TODO: Implement forward flow transformation
        // For now, return identity
        Ok(input.clone())
    }

    /// Decode latent representation through inverse flow
    ///
    /// # Arguments
    /// * `latent` - Latent tensor of shape [C, H, W]
    /// * `phase_tag` - 8-bit phase conditioning tag
    ///
    /// # Returns
    /// Reconstructed data
    pub fn decode(&self, latent: &Array3<f32>, _phase_tag: u8) -> Result<Array3<f32>> {
        if !self.loaded {
            return Err(FlowNetError::ModelNotLoaded);
        }
        
        // TODO: Implement inverse flow transformation
        // For now, return identity
        Ok(latent.clone())
    }

    /// Load model weights from bytes
    pub fn load_weights(&mut self, _weights: &[u8]) -> Result<()> {
        // TODO: Implement weight loading
        self.loaded = true;
        Ok(())
    }

    /// Load ONNX model (only available with tract feature)
    #[cfg(feature = "tract")]
    pub fn load_onnx(&mut self, model_bytes: &[u8]) -> Result<()> {
        // TODO: Implement ONNX loading via tract
        self.loaded = true;
        Ok(())
    }
}

/// C FFI interface
#[no_mangle]
pub extern "C" fn flownet_new(levels: usize, depth: usize) -> *mut FlowNet {
    Box::into_raw(Box::new(FlowNet::new(levels, depth)))
}

#[no_mangle]
pub extern "C" fn flownet_free(ptr: *mut FlowNet) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn flownet_encode(
    _ptr: *const FlowNet,
    _input_ptr: *const f32,
    _input_len: usize,
    _phase_tag: u8,
    _output_ptr: *mut f32,
) -> i32 {
    // TODO: Implement C FFI encode
    0
}

#[no_mangle]
pub extern "C" fn flownet_decode(
    _ptr: *const FlowNet,
    _latent_ptr: *const f32,
    _latent_len: usize,
    _phase_tag: u8,
    _output_ptr: *mut f32,
) -> i32 {
    // TODO: Implement C FFI decode
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flownet_creation() {
        let flow = FlowNet::new(4, 4);
        assert_eq!(flow.levels, 4);
        assert_eq!(flow.depth, 4);
        assert!(!flow.loaded);
    }

    #[test]
    fn test_encode_decode_identity() {
        let mut flow = FlowNet::default();
        // Simulate loading weights
        flow.loaded = true;
        
        let input = Array3::<f32>::zeros((3, 32, 32));
        let phase_tag = 0;
        
        let encoded = flow.encode(&input, phase_tag).unwrap();
        let decoded = flow.decode(&encoded, phase_tag).unwrap();
        
        // For now, should be identity
        assert_eq!(input.shape(), decoded.shape());
    }
} 