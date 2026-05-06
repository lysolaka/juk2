//! System configuration
use serde::{Deserialize, Serialize};

/// Configuration structure for the system.
#[derive(defmt::Format)]
pub struct SystemConfig {
    pub accel: f32,
    pub vel: f32,
    pub frame: Frame,
    pub unit: Unit,
    pub mmps: (f32, f32, f32),
    pub mode: juk_com::InterfaceMode,
    pub led: u32,
}

/// Reference frame for movements.
#[derive(defmt::Format, Deserialize, Serialize, Clone, Copy)]
pub enum Frame {
    Absolute,
    Relative,
}

/// Unit of measurement for displacement.
#[derive(defmt::Format, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Steps,
    Millimeters,
}
