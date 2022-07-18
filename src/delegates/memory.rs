//! Implements memory management functions

use crate::sys;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
};

/// Converts an `usize` size argument to an `u32`
fn size_u32(size: usize) -> u32 {
    u32::try_from(size).expect("Size is too large")
}

/// A global allocator
pub struct PicoMalloc;
unsafe impl GlobalAlloc for PicoMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = ptr::null_mut();
        sys::pico_mem_alloc(&mut ptr, size_u32(layout.size()));
        ptr
    }

    unsafe fn dealloc(&self, mut ptr: *mut u8, _layout: Layout) {
        sys::pico_mem_free(&mut ptr);
    }

    unsafe fn realloc(&self, mut ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        sys::pico_mem_realloc(&mut ptr, size_u32(new_size));
        ptr
    }
}
