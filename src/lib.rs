#![no_std]
#![doc = include_str!("../README.md")]

use core::sync::atomic::{AtomicUsize, Ordering};

/// The type of an event handler.
///
/// Currently, only no arguments and return values are supported.
pub type Handler = fn();

/// A lock-free table of event handlers.
///
/// Internally stores up to `N` function pointers in an array of `AtomicUsize`.
/// All operations are O(1), and are safe for concurrent use in `no_std`.
///
/// # Type Parameters
/// - `N`: Number of handler slots (must be > 0).
pub struct HandlerTable<const N: usize> {
    handlers: [AtomicUsize; N],
}

impl<const N: usize> HandlerTable<N> {
    /// Creates a new `HandlerTable` with all slots empty.
    pub const fn new() -> Self {
        Self {
            handlers: [const { AtomicUsize::new(0) }; N],
        }
    }

    /// Attempts to register `handler` in slot `idx`.
    ///
    /// - `idx`: Slot index (0 ≤ `idx` < `N`).
    /// - `handler`: Function pointer to register.
    ///
    /// # Returns
    /// - `true` if the slot was empty and registration succeeded.
    /// - `false` if `idx` is out of range or the slot was already occupied.
    pub fn register_handler(&self, idx: usize, handler: Handler) -> bool {
        if idx >= N {
            return false;
        }
        self.handlers[idx]
            .compare_exchange(0, handler as usize, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    /// Unregisters and returns the handler in slot `idx`.
    ///
    /// # Parameters
    /// - `idx`: Slot index (0 ≤ `idx` < `N`).
    ///
    /// # Returns
    /// - `Some(handler)` if a handler was registered.
    /// - `None` if `idx` is out of range or the slot was empty.
    ///
    /// # Concurrency
    /// Lock-free and thread-safe: uses atomic swap.
    pub fn unregister_handler(&self, idx: usize) -> Option<Handler> {
        if idx >= N {
            return None;
        }
        let handler = self.handlers[idx].swap(0, Ordering::Acquire);
        if handler != 0 {
            Some(unsafe { core::mem::transmute::<usize, fn()>(handler) })
        } else {
            None
        }
    }

    /// Invokes the handler in slot `idx`.
    ///
    /// # Parameters
    /// - `idx`: Slot index (0 ≤ `idx` < `N`).
    ///
    /// # Returns
    /// - `true` if a handler was found and called.
    /// - `false` if `idx` is out of range or the slot was empty.
    ///
    /// # Concurrency
    /// Lock-free and thread-safe: uses atomic load.
    ///
    /// # Panics
    /// Panics if the handler itself panics.
    pub fn handle(&self, idx: usize) -> bool {
        if idx >= N {
            return false;
        }
        let handler = self.handlers[idx].load(Ordering::Acquire);
        if handler != 0 {
            let handler: Handler = unsafe { core::mem::transmute(handler) };
            handler();
            true
        } else {
            false
        }
    }
}

impl<const N: usize> Default for HandlerTable<N> {
    fn default() -> Self {
        Self::new()
    }
}
