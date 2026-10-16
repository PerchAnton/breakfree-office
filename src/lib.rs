//! Rust-native Office launcher. Editors remain independent processes.

pub mod contract;
pub mod gui;
pub mod process;
pub mod registry;
pub mod visual;

pub type Result<T> = std::result::Result<T, String>;
