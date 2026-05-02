/// ! Command definitions
use alloc::string::String;
use alloc::vec::Vec;

use juk_motion::interp::ArcDir;
use serde::{Deserialize, Serialize};

use crate::Displacement;

#[derive(Deserialize, Serialize)]
pub enum Command {
    Move {
        x: Displacement,
        y: Displacement,
        z: Displacement,
        a: f32,
        v: f32,
    },
    Arc {
        x: Displacement,
        y: Displacement,
        z: Displacement,
        r: u32,
        dir: ArcDir,
        a: f32,
        v: f32,
    },
    Home {
        x: bool,
        y: bool,
        z: bool,
    },
    ConfigSet {
        kv: Vec<(String, String)>,
    },
    ConfigGet {
        key: String,
    },
}
