//! Interpolators

mod arc;
mod line;

pub use line::LineGenerator;
pub use arc::ArcGenerator;

use esp_hal::gpio::Level;

/// Describes whether to do a step, and if yes, what direction the step should be.
#[derive(Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum Step {
    Positive,
    None,
    Negative,
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

/// Arc direction for the [`ArcGenerator`].
#[derive(defmt::Format)]
pub enum ArcDir {
    /// Clockwise
    CW,
    /// Counter-clockwise
    CCW,
}
