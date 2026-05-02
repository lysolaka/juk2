//! System configuration

/// Configuration structure for the system.
#[derive(defmt::Format)]
pub struct SystemConfig {
    pub accel: f32,
    pub vel: f32,
    pub frame: Frame,
    pub unit: Unit,
    pub mmps: (f32, f32, f32),
    pub led: u32,
}

/// Reference frame for movements.
#[derive(defmt::Format)]
pub enum Frame {
    Absolute,
    Relative,
}

/// Unit of measurement for displacement.
#[derive(defmt::Format)]
pub enum Unit {
    Steps,
    Millimeters,
}

