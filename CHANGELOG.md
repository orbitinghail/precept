# Changelog

All notable changes will be documented in this file.

## 0.4.1 - 2026-07-13

- Expectation macros (`expect_*!`, `observe!`) now type- and borrow-check their arguments even when the `enabled` feature is off, so errors surface in every configuration and expectation-only temporaries no longer trigger `unused_variables`/`unused_assignments` warnings in downstream crates. Arguments are still never evaluated at runtime when disabled, preserving the crate's zero runtime overhead.

## 0.4.0 - 2026-07-13

- Added `observe!`, a property block that may call any precept API and may optionally borrow one or more `GhostState`s; its `Fn` bound makes it easier to avoid unintentionally mutating the system under test.
- Added `GhostState<T>`, opaque auxiliary state for expressing test properties, readable only via `observe!` and mutable only via `GhostState::mutate`, compiled out entirely when the `enabled` feature is off.

## 0.3.0 - 2026-02-13

- Updated to `rand` 0.10, migrating the internal RNG usage accordingly.
- Dependency updates.

## 0.2.0 - 2025-12-10

- Added new fault system
- Better doc comments

## 0.1.0 - 2025-03-29

- Initial release
