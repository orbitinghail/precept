pub mod catalog;
pub mod dispatch;
pub mod fault;
pub mod random;

#[doc(hidden)]
pub mod function_name;

#[doc(hidden)]
pub mod deps {
    pub use linkme;
    pub use serde_json;
}

#[cfg(not(feature = "disabled"))]
pub mod macros;

#[cfg(feature = "disabled")]
pub mod macros_stubs;

use dispatch::{Dispatch, SetDispatchError};

/// If ENABLED is false, all precept macros and faults are disabled
pub const ENABLED: bool = cfg!(not(feature = "disabled"));

pub fn init(dispatcher: &'static dyn Dispatch) -> Result<(), SetDispatchError> {
    if cfg!(not(feature = "disabled")) {
        dispatch::set_dispatcher(dispatcher)?;
        catalog::init_catalog();
        fault::init_faults();
    }
    Ok(())
}

pub fn init_boxed(dispatcher: Box<dyn Dispatch>) -> Result<(), SetDispatchError> {
    if cfg!(not(feature = "disabled")) {
        init(Box::leak(dispatcher))
    } else {
        Ok(())
    }
}
