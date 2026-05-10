//! Command definitions and parsing for JUK2.
#![no_std]

extern crate alloc;

pub mod cmd;
pub mod config;
pub mod defaults;
mod parser;

pub use parser::parse_cmd;

use serde::{Deserialize, Serialize};

use crate::config::{Frame, SystemConfig};

/// Errors, which can be encountered when using `juk-cmd`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The displacement value is out of range
    #[error("displacement value out of range")]
    DisplacementRange,
    /// The motion value is out of range
    #[error("motion value out of range")]
    MotionRange,
    /// The command parser failed due to empty input
    #[error("empty input")]
    EmptyInput,
    /// There is no such command
    #[error("unknown command")]
    UnknownCommand,
    /// The command received an invalid argument
    #[error("invalid argument")]
    InvalidArgument,
    /// Argument relation was not satisfied.
    #[error("argument relation not satisfied")]
    ArgumentRelation,
    /// Couldn't parse a floating-point number
    #[error("expected a floating-point number, {0}")]
    FloatParse(#[from] core::num::ParseFloatError),
    /// Couldn't parse an integer number
    #[error("expected an integer number, {0}")]
    IntParse(#[from] core::num::ParseIntError),
}

/// Errors, which can be encountered when using `juk-motion`.
#[derive(Debug, defmt::Format, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum MotionError {
    /// The movement is a no-op
    #[error("the movement is a no-op")]
    ZeroDisplacement,
    /// The velocity is 0: division by 0 is imminent
    #[error("the velocity is 0, division by 0 is imminent")]
    ZeroVelocity,
    /// The acceleration is 0: division by 0 is imminent
    #[error("the acceleration is 0, division by 0 is imminent")]
    ZeroAcceleration,
    /// The arc is impossible
    #[error("the arc is impossible")]
    ImpossibleGeometry,
    /// The radius is 0: this is not an arc
    #[error("the radius is 0, this is not an arc")]
    ZeroRadius,
}

/// Arc direction for the [`ArcGenerator`].
#[derive(defmt::Format, Deserialize, Serialize)]
pub enum ArcDir {
    /// Clockwise (negative angle)
    Neg,
    /// Anti-clockwise (positive angle)
    Pos,
}

/// Axis marker for displacement calculation.
#[derive(defmt::Format, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Displacement unit, always in steps.
#[derive(defmt::Format, Clone, Copy, Deserialize, Serialize)]
pub enum Displacement {
    Relative(i32),
    Absolute(i32),
}

impl Displacement {
    /// Convert millimeters to a displacement. Returns an error if the input is out of range.
    pub fn from_mm(d: f32, axis: Axis, cfg: &SystemConfig) -> Result<Self, Error> {
        if !defaults::MM_DISP_RANGE.contains(&d) {
            return Err(Error::DisplacementRange);
        }

        let steps = match axis {
            Axis::X => libm::roundf(d / cfg.mmps.0),
            Axis::Y => libm::roundf(d / cfg.mmps.1),
            Axis::Z => libm::roundf(d / cfg.mmps.2),
        } as i32;

        match cfg.frame {
            Frame::Absolute => {
                if steps < 0 {
                    Err(Error::DisplacementRange)
                } else {
                    Ok(Self::Absolute(steps))
                }
            }
            Frame::Relative => Ok(Self::Relative(steps)),
        }
    }

    /// Convert steps to a displacement. Returns an error if the input is out of range.
    pub fn from_steps(d: i32, cfg: &SystemConfig) -> Result<Self, Error> {
        if !defaults::STEP_DISP_RANGE.contains(&d) {
            return Err(Error::DisplacementRange);
        }

        match cfg.frame {
            Frame::Absolute => {
                if d < 0 {
                    Err(Error::DisplacementRange)
                } else {
                    Ok(Self::Absolute(d))
                }
            }
            Frame::Relative => Ok(Self::Relative(d)),
        }
    }

    /// Convert the displacement to relative.
    ///
    /// # Arguments
    ///
    /// - `axis`: the axis of the displacement
    /// - `pos`: the current absolute position as a tuple of `(x, y, z)`
    pub fn to_relative(self, axis: Axis, pos: (i32, i32, i32)) -> i32 {
        match self {
            Displacement::Relative(d) => d,
            Displacement::Absolute(d) => match axis {
                Axis::X => d - pos.0,
                Axis::Y => d - pos.1,
                Axis::Z => d - pos.2,
            },
        }
    }
}
