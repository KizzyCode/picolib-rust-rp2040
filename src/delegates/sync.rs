//! Implements synchronization primitives

use crate::sys;
use core::cell::Cell;

/// A threadsafe, shared lock
pub struct Lock {
    /// The underlying lock flag
    flag: Cell<bool>,
}
impl Lock {
    /// Creates a new lock
    pub const fn new() -> Self {
        Self { flag: Cell::new(false) }
    }

    /// Acquires the lock
    pub fn acquire(&self) {
        /// Tries to acquire the lock, returns `true` on success or `false` on error
        fn try_lock(flag: &Cell<bool>) -> bool {
            // Lock the hardware spinlock
            let mut irq = 0;
            unsafe { sys::pico_spinlock_lock(&mut irq) };

            // Set the lock flag
            let state = flag.get();
            if !state {
                flag.set(true);
            }

            // Unlock the hardware spinlock
            unsafe { sys::pico_spinlock_unlock(irq) };
            !state
        }

        // Spin-loop until we can acquire the lock
        while !try_lock(&self.flag) {
            unsafe { sys::pico_sleep_us(1) }
        }
    }
    /// Releases the lock
    ///
    /// # Info/Warning
    /// This function can be called from every thread and even if the lock is not acquired without causing an error
    pub fn release(&self) {
        // Lock the hardware spinlock
        let mut irq = 0;
        unsafe { sys::pico_spinlock_lock(&mut irq) };

        // Set the flag and unlock the hardware spinlock
        self.flag.set(false);
        unsafe { sys::pico_spinlock_unlock(irq) };
    }

    /// Provides a scope-based synchronized access (i.e. acquires the lock, calls `f` and releases the lock afterwards)
    pub fn synchronized<F, FR>(&self, f: F) -> FR
    where
        F: FnOnce() -> FR,
    {
        // Acquire lock and call the function
        self.acquire();
        let result = f();

        // Release lock and return result
        self.release();
        result
    }
}
unsafe impl Sync for Lock {
    /* Marker trait */
}

/// A mutex
pub struct Mutex<'a, T> {
    /// The lock
    lock: &'a Lock,
    /// The value
    value: Cell<Option<T>>,
}
impl<'a, T> Mutex<'a, T> {
    /// Creates a new mutex
    pub const fn new(lock: &'a Lock, value: T) -> Self {
        let value = Cell::new(Some(value));
        Self { lock, value }
    }

    /// Provides exclusive, scoped access to the underlying value
    pub fn synchronized<F, FR>(&self, f: F) -> FR
    where
        F: FnOnce(&mut T) -> FR,
    {
        self.lock.synchronized(|| {
            // Take the value and call the function
            let mut value = self.value.take().expect("Mutex has no underlying value?!");
            let result = f(&mut value);

            // Re-insert the value and return the result
            self.value.set(Some(value));
            result
        })
    }
}
unsafe impl<'a, T> Sync for Mutex<'a, T> {
    /* Marker trait */
}
