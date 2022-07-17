//! Implements thread/multicore related functions

use crate::delegates;
use core::time::Duration;

/// Sleeps for the given amount of time
pub fn sleep(duration: Duration) {
    // Split the duration into seconds and remainder
    let secs = duration.as_secs();
    let micros = duration.subsec_micros();

    // Convert the seconds into milliseconds
    let millis = secs.checked_mul(1000).expect("Sleep interval is too large");
    let millis = u32::try_from(millis).expect("Sleep interval is too large");

    // Call delegates
    unsafe { delegates::pico_sleep_ms(millis) };
    unsafe { delegates::pico_sleep_us(micros) };
}

/// Starts `f` on the second core
pub fn core1_start(f: unsafe extern "C" fn()) {
    unsafe { delegates::pico_core1_start(Some(f)) }
}
/// Stops and resets the second core
pub fn core1_halt() {
    unsafe { delegates::pico_core1_halt() }
}
