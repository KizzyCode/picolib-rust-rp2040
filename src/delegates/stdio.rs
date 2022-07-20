//! Implements some stdio routines

use crate::{delegates::sync::Lock, sys};
use core::fmt::{self, Write};

/// The stdin lock
static STDIN_LOCK: Lock = Lock::new();
/// The stdout lock
static STDOUT_LOCK: Lock = Lock::new();

/// A stdio handle
pub struct Stdio {
    _private: (),
}
impl Stdio {
    /// Creates a new stdio handle
    pub const fn new() -> Self {
        Self { _private: () }
    }

    /// Fills `buf` with bytes from stdin
    ///
    /// # Note
    /// This method blocks until `buf` has been filled completely
    pub fn read(&mut self, buf: &mut [u8]) {
        STDIN_LOCK.synchronized(|| {
            for byte in buf.iter_mut() {
                unsafe { sys::pico_stdio_getc(byte) }
            }
        })
    }
    /// Reads one byte from stdin
    ///
    /// # Note
    /// This method blocks until the byte has been read
    pub fn read_one(&mut self) -> u8 {
        STDIN_LOCK.synchronized(|| {
            let mut byte = 0;
            unsafe { sys::pico_stdio_getc(&mut byte) };
            byte
        })
    }

    /// Writes `data` to stdout
    ///
    /// # Note
    /// This method blocks until `data` has been written completely
    pub fn write(&mut self, data: &[u8]) {
        STDOUT_LOCK.synchronized(|| {
            for &byte in data {
                unsafe { sys::pico_stdio_putc(byte) }
            }
        })
    }
    /// Writes one byte to stdout
    ///
    /// # Note
    /// This method blocks until the byte has been written
    pub fn write_one(&mut self, byte: u8) {
        STDOUT_LOCK.synchronized(|| unsafe { sys::pico_stdio_putc(byte) })
    }
}

/// A target for `core::fmt::Write`
pub struct FmtTarget<T> {
    /// The underlying buffer
    buf: T,
    /// The position within the buffer
    pos: usize,
}
impl<T> FmtTarget<T> {
    /// Creates a new format-target-wrapper over the given buffer
    pub const fn new(buf: T) -> Self {
        Self { buf, pos: 0 }
    }

    /// Returns the part that has been written by `core::fmt::Write`
    pub fn written(&self) -> &[u8]
    where
        T: AsRef<[u8]>,
    {
        let slice = self.buf.as_ref();
        &slice[..self.pos]
    }
}
impl<T> Write for FmtTarget<T>
where
    T: AsMut<[u8]>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Prepare the target slice and write all bytes
        let slice = self.buf.as_mut();
        for byte in s.bytes() {
            // Write the next byte
            let slot = slice.get_mut(self.pos).ok_or(fmt::Error)?;
            *slot = byte;
            self.pos += 1;
        }
        Ok(())
    }
}

/// Prints to stdout
#[macro_export]
#[doc(hidden)]
macro_rules! print_impl {
    (eol: $eol:expr, args: $($arg:tt)*) => {{
        use ::core::fmt;
        use $crate::{alloc::format, stdio::Stdio};

        let _print = || -> fmt::Result {
            // Format the message
            let mut string = format!($($arg)*);
            string.push_str($eol);

            // Write the message to stdout
            Stdio::new().write(string.as_bytes());
            Ok(())
        };
        _print()
    }};
}
/// Prints to stdout
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{ print_impl!(eol: "", args: $($arg)*) }};
}
/// Prints a line to stdout
#[macro_export]
macro_rules! println {
    () => {{ print_impl!(eol: "\n", args: "") }};
    ($($arg:tt)*) => {{ print_impl!(eol: "\n", args: $($arg)*) }};
}
