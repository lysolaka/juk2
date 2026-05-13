//! System configuration
use serde::{Deserialize, Serialize};

/// Configuration structure for the system.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct SystemConfig {
    pub accel: f32,
    pub vel: f32,
    pub frame: Frame,
    pub unit: Unit,
    pub mmps: (f32, f32, f32),
    pub mode: InterfaceMode,
    pub led: u32,
}

/// Reference frame for movements.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Frame {
    Absolute,
    Relative,
}

/// Unit of measurement for displacement.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Steps,
    Millimeters,
}

/// The operating mode of [`juk_com::Interface`].
///
/// Used to track state of the [`juk_com::Interface`] state machine.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceMode {
    Binary,
    Text,
}

