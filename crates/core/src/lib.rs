#![forbid(unsafe_code)]
#![allow(dead_code)]

//! # wiradesk-core
//! Platform-agnostic, sans-IO core domain engine for Wira Desk.
//!
//! Provides pure geometry planners, layout calculation models, state machines,
//! and command representations without OS handles or I/O syscalls.

pub mod arrangement;
pub mod commands;
pub mod switcher;
pub mod update;
pub mod window;

pub use commands::Command;
pub use window::WindowId;
