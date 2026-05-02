//! Command definitions and parsing for JUK2.
#![no_std]

extern crate alloc;

pub mod cmd;
pub mod config;
mod defaults;

use serde::{Deserialize, Serialize};

use config::{Frame, SystemConfig};

/// Errors, which can be encountered when using `juk-cmd`.
#[derive(Debug, defmt::Format, thiserror::Error)]
pub enum Error {
    /// The input value is out of range
    #[error("input value out of range")]
    OutOfRange,
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
            return Err(Error::OutOfRange);
        }

        let steps = match axis {
            Axis::X => libm::roundf(d / cfg.mmps.0),
            Axis::Y => libm::roundf(d / cfg.mmps.1),
            Axis::Z => libm::roundf(d / cfg.mmps.2),
        } as i32;

        match cfg.frame {
            Frame::Absolute => {
                if steps < 0 {
                    Err(Error::OutOfRange)
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
            return Err(Error::OutOfRange);
        }

        match cfg.frame {
            Frame::Absolute => {
                if d < 0 {
                    Err(Error::OutOfRange)
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
    pub fn to_relative(self, axis: Axis, pos: (i32, i32, i32)) -> Self {
        match self {
            Displacement::Relative(d) => Displacement::Relative(d),
            Displacement::Absolute(d) => match axis {
                Axis::X => Displacement::Relative(d - pos.0),
                Axis::Y => Displacement::Relative(d - pos.1),
                Axis::Z => Displacement::Relative(d - pos.2),
            },
        }
    }
}
