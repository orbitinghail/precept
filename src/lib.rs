pub mod dispatch;
pub mod fault;
pub mod random;

#[doc(hidden)]
pub mod catalog;

#[doc(hidden)]
pub mod function_name;

#[doc(hidden)]
pub mod deps {
    pub use linkme;
    pub use serde_json;
}

#[cfg(feature = "enabled")]
pub mod macros;

#[cfg(not(feature = "enabled"))]
pub mod macros_stubs;

use dispatch::{Dispatch, SetDispatchError};

/// If ENABLED is false, all precept macros and faults are disabled
pub const ENABLED: bool = cfg!(feature = "enabled");

/// Initializes the precept library with a static dispatcher reference.
///
/// This function sets up the global dispatcher, registers all catalog entries,
/// and initializes faults. It should be called once at application startup.
///
/// Returns an error if a dispatcher has already been set.
pub fn init(dispatcher: &'static dyn Dispatch) -> Result<(), SetDispatchError> {
    if cfg!(feature = "enabled") {
        dispatch::set_dispatcher(dispatcher)?;
        catalog::init_catalog();
        fault::init_faults();
    }
    Ok(())
}

/// Initializes the precept library with a boxed dispatcher.
///
/// This is a convenience wrapper around [`init`] that accepts a boxed dispatcher.
/// The box is leaked to create a static reference.
///
/// Returns an error if a dispatcher has already been set.
pub fn init_boxed(dispatcher: Box<dyn Dispatch>) -> Result<(), SetDispatchError> {
    if cfg!(feature = "enabled") {
        init(Box::leak(dispatcher))
    } else {
        Ok(())
    }
}
