//! FlowNet: Invertible neural flow for RBT compression
//!
//! This crate provides a normalizing flow implementation for learned compression.
//! It supports no-std environments and optional tract integration for inference.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "flownet"), allow(dead_code, unused_imports))]

// Core imports
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
#[cfg(feature = "std")]
use std::boxed::Box;

#[cfg(feature = "flownet")]
use ndarray::{Array3, s};

#[cfg(all(feature = "flownet", feature = "tract"))]
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

/// Coupling layer types
#[cfg(feature = "flownet")]
#[derive(Debug, Clone, Copy)]
pub enum CouplingType {
    /// Additive coupling: y = x + t(x_masked)
    Additive,
    /// Affine coupling: y = x * exp(s(x_masked)) + t(x_masked)
    Affine,
}

/// FiLM (Feature-wise Linear Modulation) parameters
#[cfg(feature = "flownet")]
#[derive(Debug, Clone)]
pub struct FilmParams {
    /// Gamma (scale) parameters for FiLM conditioning
    pub gamma: Array3<f32>,
    /// Beta (shift) parameters for FiLM conditioning  
    pub beta: Array3<f32>,
}

/// A single coupling block in the normalizing flow
#[cfg(feature = "flownet")]
pub struct CouplingBlock {
    /// Type of coupling transformation
    coupling_type: CouplingType,
    /// Mask for splitting channels (true = transform, false = identity)
    mask: Vec<bool>,
    /// Network depth for transformation functions
    depth: usize,
    /// Cached transformation parameters
    cached_params: Option<(Array3<f32>, Option<Array3<f32>>)>, // (translation, scale)
    /// FiLM conditioning lookup table (256 entries for 8-bit phase tags)
    film_table: Vec<FilmParams>,
}

/// FlowNet model for invertible transformations
#[cfg(feature = "flownet")]
pub struct FlowNet {
    /// Number of flow levels
    levels: usize,
    /// Depth of coupling blocks per level
    depth: usize,
    /// Whether a model is loaded
    loaded: bool,
    /// Coupling blocks for each level
    coupling_blocks: Vec<Vec<CouplingBlock>>,
    /// Tract model (only with tract feature)
    #[cfg(feature = "tract")]
    model: Option<SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>>,
}

#[cfg(feature = "flownet")]
impl CouplingBlock {
    /// Create a new coupling block
    pub fn new(coupling_type: CouplingType, channels: usize, depth: usize) -> Self {
        // Create alternating mask pattern
        let mask = (0..channels).map(|i| i % 2 == 0).collect();
        
        // Initialize FiLM table with 256 entries for 8-bit phase tags
        let mut film_table = Vec::with_capacity(256);
        for phase_tag in 0..256 {
            // Generate phase-dependent FiLM parameters
            let phase_norm = phase_tag as f32 / 255.0; // Normalize to [0, 1]
            
            // Linear + sinusoidal modulation to ensure all phase tags are different
            let gamma_val = 1.0 + 0.3 * phase_norm + 0.2 * (phase_norm * 2.0 * core::f32::consts::PI).sin();
            let beta_val = 0.1 * phase_norm + 0.1 * (phase_norm * 4.0 * core::f32::consts::PI).cos();
            
            let gamma = Array3::<f32>::from_elem((channels, 1, 1), gamma_val);
            let beta = Array3::<f32>::from_elem((channels, 1, 1), beta_val);
            
            film_table.push(FilmParams { gamma, beta });
        }
        
        Self {
            coupling_type,
            mask,
            depth,
            cached_params: None,
            film_table,
        }
    }
    
    /// Forward transformation through coupling block
    pub fn forward(&self, input: &Array3<f32>, phase_tag: u8) -> Result<(Array3<f32>, f32)> {
        let (_c, _h, _w) = input.dim();
        let mut output = input.clone();
        let mut log_det = 0.0f32;
        
        // Split channels according to mask
        let (x_id, _x_transform) = self.split_channels(input);
        
        // Compute transformation parameters from identity channels with FiLM conditioning
        let (translation, scale) = self.compute_transform_params(&x_id, phase_tag)?;
        
        // Apply coupling transformation to transform channels
        for (ch_idx, &should_transform) in self.mask.iter().enumerate() {
            if should_transform {
                let mut ch_slice = output.slice_mut(s![ch_idx, .., ..]);
                match self.coupling_type {
                    CouplingType::Additive => {
                        // y = x + t(x_masked)
                        let t_slice = translation.slice(s![ch_idx, .., ..]);
                        ch_slice += &t_slice;
                        // Additive coupling has zero log-determinant
                    }
                    CouplingType::Affine => {
                        // y = x * exp(s(x_masked)) + t(x_masked)
                        if let Some(ref scale_arr) = scale {
                            let s_slice = scale_arr.slice(s![ch_idx, .., ..]);
                            let t_slice = translation.slice(s![ch_idx, .., ..]);
                            
                            // Apply affine transformation
                            ch_slice.zip_mut_with(&s_slice, |y, &s| *y *= s.exp());
                            ch_slice += &t_slice;
                            
                            // Accumulate log-determinant
                            log_det += s_slice.sum();
                        }
                    }
                }
            }
        }
        
        Ok((output, log_det))
    }
    
    /// Inverse transformation through coupling block
    pub fn inverse(&self, input: &Array3<f32>, phase_tag: u8) -> Result<Array3<f32>> {
        let mut output = input.clone();
        
        // Split channels according to mask
        let (x_id, _) = self.split_channels(input);
        
        // Compute transformation parameters from identity channels with FiLM conditioning
        let (translation, scale) = self.compute_transform_params(&x_id, phase_tag)?;
        
        // Apply inverse coupling transformation
        for (ch_idx, &should_transform) in self.mask.iter().enumerate() {
            if should_transform {
                let mut ch_slice = output.slice_mut(s![ch_idx, .., ..]);
                match self.coupling_type {
                    CouplingType::Additive => {
                        // x = y - t(x_masked)
                        let t_slice = translation.slice(s![ch_idx, .., ..]);
                        ch_slice -= &t_slice;
                    }
                    CouplingType::Affine => {
                        // x = (y - t(x_masked)) / exp(s(x_masked))
                        if let Some(ref scale_arr) = scale {
                            let s_slice = scale_arr.slice(s![ch_idx, .., ..]);
                            let t_slice = translation.slice(s![ch_idx, .., ..]);
                            
                            ch_slice -= &t_slice;
                            ch_slice.zip_mut_with(&s_slice, |x, &s| *x /= s.exp());
                        }
                    }
                }
            }
        }
        
        Ok(output)
    }
    
    /// Split channels according to mask
    fn split_channels(&self, input: &Array3<f32>) -> (Array3<f32>, Array3<f32>) {
        let (c, h, w) = input.dim();
        let mut x_id = Array3::<f32>::zeros((c, h, w));
        let mut x_transform = Array3::<f32>::zeros((c, h, w));
        
        for (ch_idx, &is_transform) in self.mask.iter().enumerate() {
            if is_transform {
                x_transform.slice_mut(s![ch_idx, .., ..])
                    .assign(&input.slice(s![ch_idx, .., ..]));
            } else {
                x_id.slice_mut(s![ch_idx, .., ..])
                    .assign(&input.slice(s![ch_idx, .., ..]));
            }
        }
        
        (x_id, x_transform)
    }
    
    /// Compute transformation parameters with FiLM conditioning
    fn compute_transform_params(&self, x_id: &Array3<f32>, phase_tag: u8) -> Result<(Array3<f32>, Option<Array3<f32>>)> {
        let (c, h, w) = x_id.dim();
        
        // Get FiLM parameters for this phase tag
        let film_params = &self.film_table[phase_tag as usize];
        
        // Compute base transformation from input statistics
        let mean = x_id.mean().unwrap_or(0.0);
        let base_translation = Array3::<f32>::from_elem((c, h, w), mean * 0.1);
        
        // Apply FiLM conditioning: gamma * base + beta
        let mut translation = Array3::<f32>::zeros((c, h, w));
        for ch in 0..c {
            let gamma_ch = film_params.gamma[[ch, 0, 0]];
            let beta_ch = film_params.beta[[ch, 0, 0]];
            
            for i in 0..h {
                for j in 0..w {
                    translation[[ch, i, j]] = gamma_ch * base_translation[[ch, i, j]] + beta_ch;
                }
            }
        }
        
        let scale = match self.coupling_type {
            CouplingType::Additive => None,
            CouplingType::Affine => {
                let std = ((x_id - mean).mapv(|x| x * x).mean().unwrap_or(1.0)).sqrt();
                let base_scale = Array3::<f32>::from_elem((c, h, w), std * 0.01);
                
                // Apply FiLM conditioning to scale as well
                let mut conditioned_scale = Array3::<f32>::zeros((c, h, w));
                for ch in 0..c {
                    let gamma_ch = film_params.gamma[[ch, 0, 0]];
                    let beta_ch = film_params.beta[[ch, 0, 0]] * 0.1; // Smaller beta for scale
                    
                    for i in 0..h {
                        for j in 0..w {
                            conditioned_scale[[ch, i, j]] = gamma_ch * base_scale[[ch, i, j]] + beta_ch;
                        }
                    }
                }
                Some(conditioned_scale)
            }
        };
        
        Ok((translation, scale))
    }
}

#[cfg(feature = "flownet")]
impl FlowNet {
    /// Create a new FlowNet instance
    pub fn new(levels: usize, depth: usize) -> Self {
        // Initialize coupling blocks for each level
        let mut coupling_blocks = Vec::new();
        for _level in 0..levels {
            let mut level_blocks = Vec::new();
            for block_idx in 0..depth {
                // Alternate between additive and affine coupling
                let coupling_type = if block_idx % 2 == 0 {
                    CouplingType::Additive
                } else {
                    CouplingType::Affine
                };
                
                // Default to 3 channels (RGB)
                let block = CouplingBlock::new(coupling_type, 3, depth);
                level_blocks.push(block);
            }
            coupling_blocks.push(level_blocks);
        }
        
        Self {
            levels,
            depth,
            loaded: false,
            coupling_blocks,
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
    /// Encoded latent representation and log-determinant
    pub fn encode(&self, input: &Array3<f32>, phase_tag: u8) -> Result<(Array3<f32>, f32)> {
        if !self.loaded {
            return Err(FlowNetError::ModelNotLoaded);
        }
        
        let mut z = input.clone();
        let mut total_log_det = 0.0f32;
        
        // Apply coupling blocks level by level
        for level_blocks in &self.coupling_blocks {
            for block in level_blocks {
                let (z_new, log_det) = block.forward(&z, phase_tag)?;
                z = z_new;
                total_log_det += log_det;
            }
        }
        
        Ok((z, total_log_det))
    }

    /// Decode latent representation through inverse flow
    ///
    /// # Arguments
    /// * `latent` - Latent tensor of shape [C, H, W]
    /// * `phase_tag` - 8-bit phase conditioning tag
    ///
    /// # Returns
    /// Reconstructed data
    pub fn decode(&self, latent: &Array3<f32>, phase_tag: u8) -> Result<Array3<f32>> {
        if !self.loaded {
            return Err(FlowNetError::ModelNotLoaded);
        }
        
        let mut x = latent.clone();
        
        // Apply coupling blocks in reverse order
        for level_blocks in self.coupling_blocks.iter().rev() {
            for block in level_blocks.iter().rev() {
                x = block.inverse(&x, phase_tag)?;
            }
        }
        
        Ok(x)
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

/// Stub FlowNet when feature is disabled
#[cfg(not(feature = "flownet"))]
pub struct FlowNet {
    _dummy: u8,
}

#[cfg(not(feature = "flownet"))]
impl FlowNet {
    pub fn new(_levels: usize, _depth: usize) -> Self {
        Self { _dummy: 0 }
    }
    
    pub fn default() -> Self {
        Self::new(4, 4)
    }
    
    pub fn encode(&self, _input: &[f32], _phase_tag: u8) -> core::result::Result<(Vec<f32>, f32), FlowNetError> {
        Err(FlowNetError::ModelNotLoaded)
    }
    
    pub fn decode(&self, _latent: &[f32], _phase_tag: u8) -> core::result::Result<Vec<f32>, FlowNetError> {
        Err(FlowNetError::ModelNotLoaded)
    }
}

/// Windows FlowNet workaround
#[cfg(all(windows, feature = "flownet"))]
compile_error!("FlowNet feature not yet supported on Windows.");

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
        #[cfg(feature = "flownet")]
        {
            assert_eq!(flow.levels, 4);
            assert_eq!(flow.depth, 4);
            assert!(!flow.loaded);
        }
        #[cfg(not(feature = "flownet"))]
        {
            // Just test that it doesn't panic
            assert_eq!(flow._dummy, 0);
        }
    }

    #[test]
    #[cfg(feature = "flownet")]
    fn test_encode_decode_roundtrip() {
        use ndarray::Array3;
        use approx::assert_relative_eq;
        
        let mut flow = FlowNet::default();
        // Simulate loading weights
        flow.loaded = true;
        
        let input = Array3::<f32>::from_elem((3, 8, 8), 0.5);
        let phase_tag = 0;
        
        let (encoded, log_det) = flow.encode(&input, phase_tag).unwrap();
        let decoded = flow.decode(&encoded, phase_tag).unwrap();
        
        // Should recover input within tolerance
        assert_eq!(input.shape(), decoded.shape());
        
        // Check that roundtrip preserves data (within numerical precision)
        for (_i, (&orig, &rec)) in input.iter().zip(decoded.iter()).enumerate() {
            assert_relative_eq!(orig, rec, epsilon = 1e-5, 
                               max_relative = 1e-4);
        }
        
        // Log determinant should be finite
        assert!(log_det.is_finite());
    }
    
    #[test]
    #[cfg(feature = "flownet")]
    fn test_coupling_block_invertibility() {
        use ndarray::Array3;
        use approx::assert_relative_eq;
        
        let block = CouplingBlock::new(CouplingType::Additive, 3, 4);
        let input = Array3::<f32>::from_elem((3, 4, 4), 1.0);
        
        let phase_tag = 42; // Test with specific phase tag
        let (encoded, _log_det) = block.forward(&input, phase_tag).unwrap();
        let decoded = block.inverse(&encoded, phase_tag).unwrap();
        
        // Should recover input exactly for additive coupling
        for (_i, (&orig, &rec)) in input.iter().zip(decoded.iter()).enumerate() {
            assert_relative_eq!(orig, rec, epsilon = 1e-6);
        }
    }
    
    #[test]
    #[cfg(feature = "flownet")]
    fn test_film_conditioning() {
        use ndarray::Array3;
        use approx::assert_relative_eq;
        
        let block = CouplingBlock::new(CouplingType::Affine, 3, 4);
        let input = Array3::<f32>::from_elem((3, 4, 4), 0.5);
        
        // Test with different phase tags
        let phase_tag_1 = 0;
        let phase_tag_2 = 128;
        let phase_tag_3 = 255;
        
        let (encoded_1, _log_det_1) = block.forward(&input, phase_tag_1).unwrap();
        let (encoded_2, _log_det_2) = block.forward(&input, phase_tag_2).unwrap();
        let (encoded_3, _log_det_3) = block.forward(&input, phase_tag_3).unwrap();
        
        // Different phase tags should produce different outputs
        let diff_12 = (&encoded_1 - &encoded_2).mapv(|x| x.abs()).sum();
        let diff_13 = (&encoded_1 - &encoded_3).mapv(|x| x.abs()).sum();
        let diff_23 = (&encoded_2 - &encoded_3).mapv(|x| x.abs()).sum();
        
        assert!(diff_12 > 1e-6, "Phase tags 0 and 128 should produce different outputs");
        assert!(diff_13 > 1e-6, "Phase tags 0 and 255 should produce different outputs");
        assert!(diff_23 > 1e-6, "Phase tags 128 and 255 should produce different outputs");
        
        // But each should still be invertible
        let decoded_1 = block.inverse(&encoded_1, phase_tag_1).unwrap();
        let decoded_2 = block.inverse(&encoded_2, phase_tag_2).unwrap();
        let decoded_3 = block.inverse(&encoded_3, phase_tag_3).unwrap();
        
        // All should recover the original input
        for (&orig, &rec) in input.iter().zip(decoded_1.iter()) {
            assert_relative_eq!(orig, rec, epsilon = 1e-5);
        }
        for (&orig, &rec) in input.iter().zip(decoded_2.iter()) {
            assert_relative_eq!(orig, rec, epsilon = 1e-5);
        }
        for (&orig, &rec) in input.iter().zip(decoded_3.iter()) {
            assert_relative_eq!(orig, rec, epsilon = 1e-5);
        }
    }
    
    #[test]
    #[cfg(feature = "flownet")]
    fn test_film_broadcast_shapes() {
        use ndarray::Array3;
        
        let block = CouplingBlock::new(CouplingType::Additive, 3, 2);
        
        // Test that FiLM parameters broadcast correctly across spatial dimensions
        let input_small = Array3::<f32>::from_elem((3, 2, 2), 1.0);
        let input_large = Array3::<f32>::from_elem((3, 8, 8), 1.0);
        
        let phase_tag = 100;
        
        // Should work with different spatial sizes
        let (encoded_small, _) = block.forward(&input_small, phase_tag).unwrap();
        let (encoded_large, _) = block.forward(&input_large, phase_tag).unwrap();
        
        assert_eq!(encoded_small.shape(), &[3, 2, 2]);
        assert_eq!(encoded_large.shape(), &[3, 8, 8]);
        
        // Verify invertibility for both sizes
        let decoded_small = block.inverse(&encoded_small, phase_tag).unwrap();
        let decoded_large = block.inverse(&encoded_large, phase_tag).unwrap();
        
        assert_eq!(decoded_small.shape(), input_small.shape());
        assert_eq!(decoded_large.shape(), input_large.shape());
    }
} 