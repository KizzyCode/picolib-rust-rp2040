//! Implements an el-cheapo cursor type

use crate::error::Error;
use core::{
    cmp::min,
    fmt::{self, Write},
    marker::PhantomData,
};

/// A cursor
pub struct Cursor<B, T> {
    /// The underlying buffer
    buf: B,
    /// The position
    pos: usize,
    /// Captures the element type
    _elements: PhantomData<[T]>,
}
impl<B, T> Cursor<B, T> {
    /// Creates a new cursor
    pub const fn new(buf: B) -> Self {
        Self { buf, pos: 0, _elements: PhantomData }
    }

    /// Gets the cursor position
    pub const fn pos(&self) -> usize {
        self.pos
    }
    /// Sets the cursor position
    pub fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Consumes `self`, returning the underlying buffer and cursor position
    pub fn into_inner(self) -> (B, usize) {
        (self.buf, self.pos)
    }
}
impl<B, T> Cursor<B, T>
where
    B: AsRef<[T]>,
{
    /// Reads the next element
    pub fn read_one(&mut self) -> Result<&T, Error> {
        // Create slice and get the slot
        let slice = self.buf.as_ref();
        let slot = slice.get(self.pos).ok_or(error!("Cannot read beyond end of buffer"))?;

        // Write the value
        self.pos += 1;
        Ok(slot)
    }

    /// The underlying buffer as slice
    pub fn as_slice(&self) -> &[T] {
        self.buf.as_ref()
    }
    /// The underlying buffer, split into `[..self.pos()]` and `[self.pos()..]`
    pub fn as_parts(&self) -> (&[T], &[T]) {
        let slice = self.buf.as_ref();
        let pos = min(slice.len(), self.pos);
        slice.split_at(pos)
    }
}
impl<B, T> Cursor<B, T>
where
    B: AsMut<[T]>,
{
    /// Writes the next element
    pub fn write_one(&mut self, value: T) -> Result<(), Error> {
        // Create slice and get the slot
        let slice = self.buf.as_mut();
        let slot = slice.get_mut(self.pos).ok_or(error!("Cannot write beyond end of buffer"))?;

        // Write the value
        *slot = value;
        self.pos += 1;
        Ok(())
    }

    /// The underlying buffer as slice
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        self.buf.as_mut()
    }
    /// The underlying buffer, split into `[..self.pos()]` and `[self.pos()..]`
    pub fn as_parts_mut(&mut self) -> (&mut [T], &mut [T]) {
        let slice = self.buf.as_mut();
        let pos = min(slice.len(), self.pos);
        slice.split_at_mut(pos)
    }
}
impl<B> Write for Cursor<B, u8>
where
    B: AsMut<[u8]>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &byte in s.as_bytes() {
            self.write_one(byte).map_err(|_| fmt::Error)?;
        }
        Ok(())
    }
}
