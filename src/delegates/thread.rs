//! Implements thread/multicore related functions

use crate::sys;
use core::time::Duration;

/// Sleeps for the given amount of time
pub fn sleep(duration: Duration) {
    let micros = u64::try_from(duration.as_micros()).expect("Sleep interval is too large");
    unsafe { sys::pico_sleep_us(micros, 0) };
}
/// Performs a busy wait for the given amount of time
pub fn wait(duration: Duration) {
    let micros = u64::try_from(duration.as_micros()).expect("Sleep interval is too large");
    unsafe { sys::pico_sleep_us(micros, 1) };
}

/// The time inverval that has passed since boot
pub fn uptime() -> Duration {
    let mut micros = 0;
    unsafe { sys::pico_time_us(&mut micros) };
    Duration::from_micros(micros)
}

/// Starts `f` on the second core
pub fn core1_start(f: unsafe extern "C" fn()) {
    unsafe { sys::pico_core1_start(Some(f)) }
}
/// Stops and resets the second core
pub fn core1_halt() {
    unsafe { sys::pico_core1_halt() }
}
