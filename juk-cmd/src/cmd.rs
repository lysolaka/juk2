/// ! Command definitions
use alloc::string::String;
use alloc::vec::Vec;

use juk_motion::interp::ArcDir;
use serde::{Deserialize, Serialize};

use crate::Displacement;

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
    Home {
        x: bool,
        y: bool,
        z: bool,
    },
    // /// Movement kill switch
    // Cancel,
    /// Set a configuration variable
    ConfigSet {
        kv: Vec<(String, String)>,
    },
    /// Read a configuration variable
    ConfigGet {
        key: String,
    },
}
