//! Reporting of Rust-heap allocations to V8.
//!
//! A `JsDataFrame`/`JsSeries` is a small wrapper around data owned by the Rust
//! heap. V8 only sees the wrapper, so on its own it has no reason to collect
//! one, the finalizer never runs, and a long-lived process that builds a frame
//! per request grows without bound.
//!
//! `napi_adjust_external_memory` is how we tell V8 about those bytes: we report
 //! a size when the value crosses into JS and withdraw the same reported size from the finalizer.
 //!
 //! `estimated_size` is re-measured at finalize time only to record drift (how
 //! much the value grew/shrank after it was reported) so the accounting error can
 //! be inspected from JS.

use napi::Env;
use std::sync::atomic::{AtomicI64, Ordering};

/// Net bytes reported to V8 and not yet withdrawn.
static REPORTED: AtomicI64 = AtomicI64::new(0);
/// Accumulated difference between bytes reported and bytes withdrawn.
static DRIFT: AtomicI64 = AtomicI64::new(0);

/// Report `size` bytes of Rust-heap data to V8.
pub fn report(env: &Env, size: i64) {
    if size <= 0 {
        return;
    }
    // A failure here only costs us GC accuracy, so it must not turn into a
    // thrown error on an otherwise successful conversion.
    if env.adjust_external_memory(size).is_ok() {
        REPORTED.fetch_add(size, Ordering::Relaxed);
    }
}

/// Withdraw `size` bytes previously reported by [`report`].
pub fn withdraw(env: &Env, size: i64) {
    if size <= 0 {
        return;
    }
    if env.adjust_external_memory(-size).is_ok() {
        REPORTED.fetch_sub(size, Ordering::Relaxed);
    }
}

/// Record that a value reported `reported` bytes but withdrew `withdrawn`.
pub fn record_drift(reported: i64, withdrawn: i64) {
    let delta = withdrawn - reported;
    if delta != 0 {
        DRIFT.fetch_add(delta, Ordering::Relaxed);
    }
}

/// Net bytes currently reported to V8 by this addon.
#[napi]
pub fn reported_external_memory() -> i64 {
    REPORTED.load(Ordering::Relaxed)
}

/// Total accounting error from sizes that changed between report and finalize.
///
/// A non-zero value means V8's external-memory counter is skewed by this many
/// bytes and will not self-correct.
#[napi]
pub fn external_memory_drift() -> i64 {
    DRIFT.load(Ordering::Relaxed)
}
