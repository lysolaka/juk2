//! Interpolators

mod arc;
mod line;

pub use juk_cmd::ArcDir;

pub use arc::ArcGenerator;
pub use line::LineGenerator;

use esp_hal::gpio::Level;

/// Describes whether to do a step, and if yes, what direction the step should be.
///
/// The `repr` of this type is [`i32`] so it can be used to modify the step position variable directly.
#[derive(Clone, Copy, PartialEq, Eq, defmt::Format)]
#[repr(i32)]
pub enum Step {
    Positive = 1,
    None = 0,
    Negative = -1,
}

impl Step {
    /// Get the [`Step`] ternary for the given displacement.
    pub const fn from_displacement(d: i32) -> Self {
        if d < 0 {
            Step::Negative
        } else if d > 0 {
            Step::Positive
        } else {
            Step::None
        }
    }

    /// Whether a step should be made.
    pub const fn step(&self) -> bool {
        match self {
            Step::Positive | Step::Negative => true,
            Step::None => false,
        }
    }

    /// What direction the step should be.
    pub const fn dir(&self) -> Level {
        match self {
            Step::Positive | Step::None => Level::Low,
            Step::Negative => Level::High,
        }
    }
}
