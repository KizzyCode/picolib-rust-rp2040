//! Implements GPIO related functions

use crate::delegates;

/// A GPIO direction
#[repr(u8)]
pub enum Direction {
    /// The GPIO is used as digital input
    Read = 0,
    /// The GPIO is used as digital output
    Write = 1,
}

/// A GPIO handle
#[derive(Debug)]
pub struct Gpio {
    /// The GPIO pin
    pin: u32,
}
impl Gpio {
    /// Creates a new GPIO handle
    pub fn new(pin: u32, direction: Direction) -> Self {
        unsafe { delegates::pico_gpio_init(pin, direction as u8) };
        Self { pin }
    }

    /// Gets the GPIO input state where low is `false` and high is `true`
    pub fn get(&self) -> bool {
        let mut state = 0;
        unsafe { delegates::pico_gpio_get(&mut state, self.pin) };
        state != 0
    }
    /// Sets the GPIO output where low is `false` and high is `true`
    pub fn set(&mut self, state: bool) {
        let state = state.then_some(1).unwrap_or(0);
        unsafe { delegates::pico_gpio_put(self.pin, state) };
    }
}
