use std::sync::OnceLock;

use noop::NoopDispatch;

use crate::catalog::CatalogEntry;

pub mod noop;
pub mod test;

#[cfg(feature = "antithesis")]
pub mod antithesis;

/// Events that can be emitted through the dispatcher.
pub enum Event {
    /// Registers a new catalog entry with the dispatcher.
    RegisterEntry(&'static CatalogEntry),
    /// Emits an assertion evaluation result.
    EmitEntry {
        /// The catalog entry being evaluated.
        entry: &'static CatalogEntry,
        /// Whether the assertion condition passed.
        condition: bool,
        /// Additional context about the assertion.
        details: serde_json::Value,
    },
    /// Signals that application setup is complete.
    SetupComplete {
        /// Additional context about the setup.
        details: serde_json::Value,
    },
    /// A custom user-defined event.
    Custom {
        /// The event name.
        name: &'static str,
        /// The event payload.
        value: serde_json::Value,
    },
}

/// Trait for event dispatchers that handle precept events and random number generation.
///
/// Implementors receive events from precept assertions and provide random numbers
/// for fault injection decisions.
pub trait Dispatch: Sync + Send {
    /// Handles an incoming event.
    fn emit(&self, event: Event);
    /// Returns a random u64 value for decision making.
    fn random(&self) -> u64;
}

static DISPATCHER: OnceLock<&'static dyn Dispatch> = OnceLock::new();

/// Error returned when attempting to set a dispatcher that has already been set.
#[derive(Debug)]
pub struct SetDispatchError;

impl std::error::Error for SetDispatchError {}

impl std::fmt::Display for SetDispatchError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("attempted to set the dispatcher after it was already set")
    }
}

/// Set the DISPATCHER to the provided dispatcher instance. This function will
/// fail if called multiple times.
pub fn set_dispatcher(dispatcher: &'static dyn Dispatch) -> Result<(), SetDispatchError> {
    DISPATCHER.set(dispatcher).map_err(|_| SetDispatchError)
}

/// Returns a reference to the dispatcher.
///
/// If a dispatcher has not been set, a no-op implementation is returned.
pub fn dispatcher() -> &'static dyn Dispatch {
    match DISPATCHER.get() {
        Some(dispatch) => *dispatch,
        None => {
            static NOOP: NoopDispatch = NoopDispatch;
            &NOOP
        }
    }
}

/// Generate a random u64 using the dispatcher
pub fn get_random() -> u64 {
    dispatcher().random()
}

/// Choose a random value from a slice of options using the dispatcher
pub fn choose<T>(options: &[T]) -> Option<&T> {
    if options.is_empty() {
        None
    } else {
        let idx: usize = (get_random() as usize) % options.len();
        Some(&options[idx])
    }
}

/// Emit an event using the dispatcher
pub fn emit(event: Event) {
    dispatcher().emit(event);
}
