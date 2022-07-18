#![no_std]
#![doc = include_str!("../README.md")]

// Import alloc
pub extern crate alloc;

#[macro_use]
pub mod error;
pub mod delegates;
pub mod sys;

/// Re-export the pico APIs
pub use delegates::*;
