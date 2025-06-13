#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]

//! ANS-X pass-through stub (identity coder)

use libc::{c_uint, c_uchar, c_void};
use std::slice;

/// SAFETY: Caller must guarantee `input` points to `len` bytes of readable memory and
/// `out_len` is a valid, writable pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ansx_encode(input: *const c_uchar, len: c_uint, out_len: *mut c_uint) -> *mut c_uchar {
    if input.is_null() || out_len.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: The invariants are upheld by the caller (see function safety contract).
    unsafe {
        let data = slice::from_raw_parts(input, len as usize);
        let mut vec = data.to_vec();
        let ptr = vec.as_mut_ptr();
        let length = vec.len() as c_uint;
        std::mem::forget(vec);
        *out_len = length;
        ptr
    }
}

/// SAFETY: Same as `ansx_encode`; caller must provide valid buffer pointers/lengths.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ansx_decode(input: *const c_uchar, len: c_uint, out_len: *mut c_uint) -> *mut c_uchar {
    // Identity decode
    unsafe { ansx_encode(input, len, out_len) }
}

/// SAFETY: `ptr` must be a pointer returned by `ansx_encode/decode` with the same `len`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ansx_free(ptr: *mut c_void, len: c_uint) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: Caller ensures pointer/length come from previous allocation in this crate.
    unsafe {
        Vec::<u8>::from_raw_parts(ptr as *mut u8, len as usize, len as usize);
    }
} 