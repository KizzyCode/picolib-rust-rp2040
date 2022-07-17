//! Implements some stdio routines

use crate::{delegates, pico::sync::Lock};

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
                unsafe { delegates::pico_stdio_getc(byte) }
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
            unsafe { delegates::pico_stdio_getc(&mut byte) };
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
                unsafe { delegates::pico_stdio_putc(byte) }
            }
        })
    }
    /// Writes one byte to stdout
    ///
    /// # Note
    /// This method blocks until the byte has been written
    pub fn write_one(&mut self, byte: u8) {
        STDOUT_LOCK.synchronized(|| unsafe { delegates::pico_stdio_putc(byte) })
    }
}
