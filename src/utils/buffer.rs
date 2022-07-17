//! Implements a stack-allocated buffer

use crate::error::Error;
use core::{
    fmt::{self, Write},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    slice,
};

/// An array-backed buffer
///
/// # Warning
/// The buffer size must not be `0`
#[derive(Debug)]
pub struct Buffer<T, const SIZE: usize> {
    /// The array backing
    backing: [MaybeUninit<T>; SIZE],
    /// The amount of bytes used
    len: usize,
}
impl<T, const SIZE: usize> Buffer<T, SIZE> {
    /// Creates a new empty buffer
    pub const fn new() -> Self {
        // Validate the size
        assert!(SIZE != 0, "Buffer size must not be 0");

        // The `assume_init` is safe because `MaybeUninit` does not require initialization
        // See: https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let backing = unsafe { MaybeUninit::uninit().assume_init() };
        Self { backing, len: 0 }
    }

    /// The amount of elements within the buffer
    pub const fn len(&self) -> usize {
        self.len
    }
    /// If the buffer is empty or not
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// If the buffer is full or not
    pub const fn is_full(&self) -> bool {
        self.len == SIZE
    }

    /// Pushes a value to the back of the buffer
    pub fn push(&mut self, element: T) -> Result<(), Error> {
        // Validate position
        if self.is_full() {
            return Err(error!("Cannot exceed maximum buffer capacity"));
        }

        // Push element
        self.backing[self.len].write(element);
        self.len += 1;
        Ok(())
    }
    /// Pops an element from the back of the buffer
    pub fn pop(&mut self) -> Result<T, Error> {
        // Validate position
        if self.is_empty() {
            return Err(error!("Cannot remove element from empty buffer"));
        }

        // Remove element
        self.len -= 1;
        let element = unsafe { self.backing[self.len].assume_init_read() };
        Ok(element)
    }
    /// Empties the buffer
    pub fn clear(&mut self) {
        for i in 0..self.len {
            unsafe { self.backing[i].assume_init_drop() }
        }
        self.len = 0;
    }

    /// The buffer as slice
    pub fn as_slice(&self) -> &[T] {
        // This is safe because `MaybeUninit<T>` and `T` have the same layout
        // See: https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#layout
        let ptr: *const MaybeUninit<T> = self.backing.as_ptr();
        unsafe { slice::from_raw_parts(ptr as *const T, self.len) }
    }
    /// The buffer as slice
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        // This is safe because `MaybeUninit<T>` and `T` have the same layout
        // See: https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#layout
        let ptr: *mut MaybeUninit<T> = self.backing.as_mut_ptr();
        unsafe { slice::from_raw_parts_mut(ptr as *mut T, self.len) }
    }
}
impl<T, const SIZE: usize> Drop for Buffer<T, SIZE> {
    fn drop(&mut self) {
        self.clear();
    }
}
impl<T, const SIZE: usize> Default for Buffer<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, const SIZE: usize> AsRef<[T]> for Buffer<T, SIZE> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T, const SIZE: usize> AsMut<[T]> for Buffer<T, SIZE> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}
impl<T, const SIZE: usize> Deref for Buffer<T, SIZE> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<T, const SIZE: usize> DerefMut for Buffer<T, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}
impl<const SIZE: usize> Write for Buffer<u8, SIZE> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.push(byte).map_err(|_| fmt::Error)?;
        }
        Ok(())
    }
}
