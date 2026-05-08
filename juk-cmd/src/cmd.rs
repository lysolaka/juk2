/// ! Command definitions
use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{ArcDir, Displacement, MotionError, config};

/// JUK2 command set expressed as an enum.
///
/// This enum implements [`serde::Serialize`] and [`serde::Deserialize`] to be used as the binary
/// communication format. Binary messages should be serialized using serde and postcard with COBS
/// encoding.
#[derive(defmt::Format, Deserialize, Serialize)]
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
    /// Read a configuration variable. In binary mode the serialized form of almost the entire
    /// config will be sent back.
    ConfigGet { key: String },
}

/// Binary responses as command results
///
/// This enum implements [`serde::Serialize`] and [`serde::Deserialize`] to be used in the binary
/// mode. Binary messages should be serialized using serde and postcard with COBS encoding.
#[derive(defmt::Format, Deserialize, Serialize)]
pub enum Response {
    /// Useful variables from the system configuration
    Config {
        accel: f32,
        vel: f32,
        frame: config::Frame,
        unit: config::Unit,
        mmps: (f32, f32, f32),
    },
    /// The command was executed successfully
    Ok,
    /// The command is unsupported in binary mode
    Unsupported,
    /// The motion was cancelled
    Cancelled,
    /// Motion error
    Err(MotionError),
}
