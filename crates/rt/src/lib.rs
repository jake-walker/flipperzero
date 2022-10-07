//! Runtime for Flipper Zero Rust binaries.
//!
//! This provides the following features:
//! - Panic handler that calls `furi_crash`

#![no_std]

pub mod panic_handler;
