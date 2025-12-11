use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    panic::panic_any,
    sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
};

use crate::ENABLED;

#[cfg(feature = "enabled")]
#[doc(hidden)]
#[linkme::distributed_slice]
pub static FAULT_CATALOG: [FaultEntry];

#[cfg(not(feature = "enabled"))]
#[doc(hidden)]
pub static FAULT_CATALOG: [&FaultEntry; 0] = [];

pub(crate) fn init_faults() {
    let mut seen = HashSet::new();
    for entry in FAULT_CATALOG {
        // fail if we have already seen this entry
        if !seen.insert(entry.name) {
            panic!("Duplicate Precept fault: {}", entry.name);
        }
    }
}

/// A fault injection point that can be triggered during testing.
///
/// Faults can be enabled/disabled and can be forced to trigger a specific
/// number of times using the pending trips mechanism.
#[derive(Debug)]
pub struct FaultEntry {
    /// the name of the fault, also serves as its Catalog id
    name: &'static str,

    /// whether or not this fault is enabled
    enabled: AtomicBool,

    /// if this value is > 0, the next call to `trip` will return true and this
    /// value will be decremented
    pending_trips: AtomicU32,
}

impl FaultEntry {
    /// Creates a new fault entry with the given name.
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            enabled: AtomicBool::new(true),
            pending_trips: AtomicU32::new(0),
        }
    }

    /// Returns true when the fault should trip
    pub fn trip(&self) -> bool {
        if self.enabled.load(Ordering::Acquire) {
            if self
                .pending_trips
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                    if count > 0 { Some(count - 1) } else { None }
                })
                .is_ok()
            {
                // forced trigger
                true
            } else {
                let should_fault = crate::dispatch::choose(&[true, false]);
                should_fault.is_some_and(|&t| t)
            }
        } else {
            false
        }
    }

    /// Enables this fault, allowing it to trip.
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
    }

    /// Disables this fault, preventing it from tripping.
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
    }

    /// Sets the number of pending forced trips.
    ///
    /// When pending trips are set, the next `count` calls to [`trip`](Self::trip)
    /// will return `true` regardless of random chance.
    pub fn set_pending(&self, count: u32) {
        self.pending_trips.store(count, Ordering::Release);
    }

    /// Returns the number of pending forced trips remaining.
    pub fn count_pending(&self) -> u32 {
        self.pending_trips.load(Ordering::Acquire)
    }
}

/// Enables all registered faults.
///
/// Panics if precept is disabled.
pub fn enable_all() {
    assert!(ENABLED, "Precept is disabled");
    for entry in FAULT_CATALOG {
        entry.enable()
    }
}

/// Disables all registered faults.
pub fn disable_all() {
    tracing::warn!("Precept Faults disabled");
    for entry in FAULT_CATALOG {
        entry.disable();
    }
}

/// Looks up a fault entry by its name.
///
/// Returns `None` if no fault with the given name exists.
pub fn get_fault_by_name(name: &str) -> Option<&'static FaultEntry> {
    FAULT_CATALOG.into_iter().find(|&entry| entry.name == name)
}

pub enum FaultPowerlossMode {
    Exit { code: i32 },
    Panic,
}

/// The panic that is raised by fault_powerloss when the current
/// FaultPowerlossMode is `Panic`
#[derive(Debug)]
pub struct FaultPowerlossPanic;

impl Display for FaultPowerlossPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

static FAULT_POWERLOSS_MODE: AtomicU64 = AtomicU64::new(0);

pub fn set_fault_powerloss_mode(mode: FaultPowerlossMode) {
    // encode the discriminant into the high 32 bits and the exit code into the
    // low 32 bits
    let encoded = match mode {
        FaultPowerlossMode::Exit { code } => code as u32 as u64,
        FaultPowerlossMode::Panic => 1u64 << 32,
    };
    FAULT_POWERLOSS_MODE.store(encoded, Ordering::Release);
}

pub fn fault_powerloss_mode() -> FaultPowerlossMode {
    let encoded = FAULT_POWERLOSS_MODE.load(Ordering::Acquire);
    let discriminant = (encoded >> 32) as u32;
    match discriminant {
        0 => FaultPowerlossMode::Exit { code: encoded as u32 as i32 },
        1 => FaultPowerlossMode::Panic,
        _ => unreachable!("unexpected FaultPowerlossMode discriminant"),
    }
}

pub fn fault_powerloss() -> ! {
    match fault_powerloss_mode() {
        FaultPowerlossMode::Exit { code } => std::process::exit(code),
        FaultPowerlossMode::Panic => panic_any(FaultPowerlossPanic),
    }
}
