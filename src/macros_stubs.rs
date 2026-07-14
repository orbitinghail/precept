// This file must be kept in sync with macros.rs.
//
// When the `enabled` feature is off these macros register no catalog entries
// and perform no dispatch — precept compiles away to nothing. But "nothing"
// must not mean "discard the arguments": the condition and details expressions
// still have to be **type-checked**, and every temporary they reference has to
// count as **used**, so that
//
//   1. the same source compiles identically with and without `enabled` (a typo
//      or type error in an expectation is caught in every build, not just the
//      instrumented one — the guarantee that makes `observe!` blocks safe to
//      leave inline), and
//   2. a temporary that exists only to feed an expectation does not trip
//      `unused_variables` / `unused_assignments` in a downstream crate built
//      with `-D warnings`.
//
// Each expansion therefore places its arguments inside `if false { … }`. That
// block is fully type- and borrow-checked (dead code is still checked), the
// contained expressions mark their captures as used, and the branch is provably
// unreachable so it is eliminated before codegen — keeping the crate's
// zero-runtime-overhead promise: the arguments are never evaluated at runtime.
//
// `$condition` and `json!($details)` are consumed by value to mirror exactly how
// the enabled path in macros.rs evaluates them (same moves, same borrows).

#[doc(hidden)]
#[macro_export]
macro_rules! define_entry {
    ($expectation:expr, $property:expr) => {
        if false {
            let _ = $expectation;
            let _ = &$property;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! emit_entry {
    ($entry:expr, $condition:expr) => {
        if false {
            let _ = $entry;
            let _ = $condition;
        }
    };
    ($entry:expr, $condition:expr, $($details:tt)+) => {
        if false {
            let _ = $entry;
            let _ = $condition;
            let _ = $crate::deps::serde_json::json!($($details)+);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_and_emit_entry {
    ($expectation:expr, $property:expr, $condition:expr) => {
        if false {
            let _ = $expectation;
            let _ = &$property;
            let _ = $condition;
        }
    };
    ($expectation:expr, $property:expr, $condition:expr, $($details:tt)+) => {
        if false {
            let _ = $expectation;
            let _ = &$property;
            let _ = $condition;
            let _ = $crate::deps::serde_json::json!($($details)+);
        }
    };
}

#[macro_export]
macro_rules! emit_event {
    ($name:expr, $($details:tt)+) => {
        if false {
            let _ = &$name;
            let _ = $crate::deps::serde_json::json!($($details)+);
        }
    };
}

#[macro_export]
macro_rules! setup_complete {
    () => {};
    ($($details:tt)+) => {
        if false {
            let _ = $crate::deps::serde_json::json!($($details)+);
        }
    };
}

#[macro_export]
macro_rules! expect_always {
    ($condition:expr, $property:expr$(, $($details:tt)+)?) => {
        $crate::define_and_emit_entry!(
            (), $property, $condition $(, $($details)+)?
        )
    };
}

#[macro_export]
macro_rules! expect_always_or_unreachable {
    ($condition:expr, $property:expr$(, $($details:tt)+)?) => {
        $crate::define_and_emit_entry!(
            (), $property, $condition $(, $($details)+)?
        )
    };
}

#[macro_export]
macro_rules! expect_sometimes {
    ($condition:expr, $property:expr$(, $($details:tt)+)?) => {
        $crate::define_and_emit_entry!(
            (), $property, $condition $(, $($details)+)?
        )
    };
}

#[macro_export]
macro_rules! expect_reachable {
    ($property:expr$(, $($details:tt)+)?) => {
        $crate::define_and_emit_entry!(
            (), $property, true $(, $($details)+)?
        )
    };
}

#[macro_export]
macro_rules! expect_unreachable {
    ($property:expr$(, $($details:tt)+)?) => {
        $crate::define_and_emit_entry!(
            (), $property, false $(, $($details)+)?
        )
    };
}

#[macro_export]
macro_rules! sometimes_fault {
    ($name:expr, $fault:expr) => {
        if false {
            let _ = &$name;
            let _ = $fault;
        }
    };
    ($name:expr, $fault:expr, $($details:tt)+) => {
        if false {
            let _ = &$name;
            let _ = $fault;
            let _ = $crate::deps::serde_json::json!($($details)+);
        }
    };
}
