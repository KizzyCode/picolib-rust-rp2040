//! Implements the panic handler

use crate::{sys, delegates::stdio::FmtTarget};
use core::{
    fmt::{self, Display, Formatter, Write},
    panic::PanicInfo,
};

/// An error
///
/// # Note
/// This type should be constructed using the `error`-macro
#[derive(Debug)]
pub struct Error {
    /// The error message
    #[doc(hidden)]
    pub message: &'static str,
    /// The file
    #[doc(hidden)]
    pub file: &'static str,
    /// The line
    #[doc(hidden)]
    pub line: u32,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.file, self.line)
    }
}
/// Creates a new error
#[macro_export]
macro_rules! error {
    ($message:expr) => {{
        $crate::error::Error { message: $message, file: file!(), line: line!() }
    }};
}

/// Handles a panic
pub fn panic_handler(panic: &PanicInfo) -> ! {
    // Create a panic message
    let _buf = unsafe { &mut sys::pico_panic_buf };
    let mut buf = FmtTarget::new(_buf);
    let _ = write!(&mut buf, "{}", panic);

    // Call the delegate
    unsafe { sys::pico_panic() };

    // Loop forever (the delegate should never return but we cannot prove it to rust)
    #[allow(clippy::empty_loop)]
    loop { /* Loop forever */ }
}
