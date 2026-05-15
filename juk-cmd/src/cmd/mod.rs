//! Command definitions

mod args;
#[cfg(not(feature = "export"))]
pub mod parser;

pub use args::{ArcDir, Axis, Displacement};

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::{MotionError, config::SystemConfig};

/// JUK2 command set expressed as an enum.
///
/// This enum implements [`serde::Serialize`] and [`serde::Deserialize`] to be used as the binary
/// communication format. Binary messages should be serialized using serde and postcard with COBS
/// encoding.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "export", derive(Debug))]
#[derive(Deserialize, Serialize)]
pub enum Command {
    /// Linear movement
    Move {
        x: Displacement,
        y: Displacement,
        z: Displacement,
        a: f32,
        v: f32,
    },
    /// Arc movement
    Arc {
        x: Displacement,
        y: Displacement,
        z: Displacement,
        r: u32,
        dir: ArcDir,
        a: f32,
        v: f32,
    },
    /// Homing
    Home { x: bool, y: bool, z: bool },
    /// Movement kill switch
    Cancel,
    /// Set a configuration variable
    ConfigSet { kv: Vec<(String, String)> },
    /// Read a configuration variable. In binary mode the serialized form of the entire
    /// config will be sent back.
    ConfigGet { key: String },
}

/// Binary responses as command results
///
/// This enum implements [`serde::Serialize`] and [`serde::Deserialize`] to be used in the binary
/// mode. Binary messages should be serialized using serde and postcard with COBS encoding.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "export", derive(Debug))]
#[derive(Deserialize, Serialize)]
pub enum Response {
    /// Part of the system configuration contained in [`SystemConfig`]
    Config(SystemConfig),
    /// The command was executed successfully
    Ok,
    /// The command is unsupported in binary mode
    Unsupported,
    /// The motion was cancelled
    Cancelled,
    /// Motion error
    Err(MotionError),
}
