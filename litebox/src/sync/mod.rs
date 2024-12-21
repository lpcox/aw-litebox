//! Higher-level synchronization primitives
//!
//! The implementation in this module is derived from related source files in Rust's `std`, taken
//! from `6fd7e9010db6be7605241c39eab7c5078ee2d5bd`. The files have been modified to support
//! invoking through the [`platform`], rather than through regular system interfaces.

use crate::platform;

mod condvar;
mod mutex;
mod rwlock;

#[cfg(feature = "lock_tracing")]
mod lock_tracing;

pub use condvar::Condvar;
pub use mutex::{Mutex, MutexGuard};

#[cfg(not(feature = "lock_tracing"))]
/// A convenience name for specific requirements from the platform
pub trait RawSyncPrimitivesProvider: platform::RawMutexProvider + 'static {}

#[cfg(feature = "lock_tracing")]
/// A convenience name for specific requirements from the platform
pub trait RawSyncPrimitivesProvider:
    platform::RawMutexProvider + platform::TimeProvider + platform::DebugLogProvider + 'static
{
}

/// The `Synchronization` provides access to all synchronization-related functionality provided by
/// LiteBox.
///
/// A LiteBox `Synchronization` is parametric in the platform it runs on.
pub struct Synchronization<Platform: RawSyncPrimitivesProvider> {
    platform: &'static Platform,

    #[cfg(feature = "lock_tracing")]
    tracker: lock_tracing::LockTracker<Platform>,
}

impl<Platform: RawSyncPrimitivesProvider> Synchronization<Platform> {
    /// Construct a new `Synchronization` instance
    ///
    /// This function is expected to only be invoked once per platform, as an initialization step,
    /// and the created `Synchronization` handle is expected to be shared across all usage over the
    /// system.
    pub fn new(platform: &'static Platform) -> Self {
        Self {
            platform,
            #[cfg(feature = "lock_tracing")]
            tracker: lock_tracing::LockTrackerX::new_from_platform(platform),
        }
    }
}

impl<Platform: RawSyncPrimitivesProvider> Synchronization<Platform> {
    /// Create a new [`Condvar`]
    fn new_condvar(&self) -> Condvar<Platform> {
        Condvar::new_from_platform(self.platform)
    }

    /// Create a new [`Mutex`]
    fn new_mutex<T>(&self, val: T) -> Mutex<Platform, T> {
        Mutex::new_from_synchronization(self, val)
    }
}
