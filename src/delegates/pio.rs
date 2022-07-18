//! Implements PIO related functions

use crate::{sys, error::Error};

/// A PIO state machine handle
#[derive(Debug)]
pub struct StateMachine {
    /// The PIO index
    pio: u32,
    /// The state machine
    sm: u32,
}
impl StateMachine {
    /// Creates a new state machine instance
    pub fn new(pio: u32, sm: u32) -> Result<Self, Error> {
        unsafe { sys::pico_piosm_start(pio, sm) };
        Ok(Self { pio, sm })
    }

    /// Reads a value from the state machines output queue
    pub fn read(&mut self) -> u32 {
        let mut value = 0;
        unsafe { sys::pico_piosm_get(&mut value, self.pio, self.sm) };
        value
    }
    /// Writes a value to the state machines input queue
    pub fn write(&mut self, value: u32) {
        unsafe { sys::pico_piosm_put(self.pio, self.sm, value) }
    }
}
impl Drop for StateMachine {
    fn drop(&mut self) {
        unsafe { sys::pico_piosm_halt(self.pio, self.sm) }
    }
}
