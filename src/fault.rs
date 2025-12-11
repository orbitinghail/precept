use std::{
    collections::HashSet,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

use crate::ENABLED;

/// Catalog of all Precept faults
#[cfg(feature = "enabled")]
#[linkme::distributed_slice]
pub static FAULT_CATALOG: [FaultEntry];

#[cfg(not(feature = "enabled"))]
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

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
    }

    pub fn set_pending(&self, count: u32) {
        self.pending_trips.store(count, Ordering::Release);
    }

    pub fn count_pending(&self) -> u32 {
        self.pending_trips.load(Ordering::Acquire)
    }
}

pub fn enable_all() {
    assert!(ENABLED, "Precept is disabled");
    for entry in FAULT_CATALOG {
        entry.enable()
    }
}

pub fn disable_all() {
    tracing::warn!("Precept Faults disabled");
    for entry in FAULT_CATALOG {
        entry.disable();
    }
}

pub fn get_fault_by_name(name: &str) -> Option<&'static FaultEntry> {
    FAULT_CATALOG.into_iter().find(|&entry| entry.name == name)
}
