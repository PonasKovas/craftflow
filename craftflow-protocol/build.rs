//! This build script will generate the packets for the protocol

#[path = "build/mod.rs"]
mod build;

pub use build::main;
