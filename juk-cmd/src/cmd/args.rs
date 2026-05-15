/// ! Types used for constructing commands
use serde::{Deserialize, Serialize};

use crate::{
    ParseError,
    config::{Frame, SystemConfig},
    defaults,
};

/// Arc direction for the [`juk_interp::ArcGenerator`].
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "export", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ArcDir {
    /// Clockwise (negative angle)
    Neg,
    /// Anti-clockwise (positive angle)
    Pos,
}

/// Axis marker for displacement calculation.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "export", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Displacement unit, always in steps.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "export", derive(Debug))]
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Displacement {
    Relative(i32),
    Absolute(i32),
}

impl Displacement {
    /// Convert millimeters to a displacement. Returns an error if the input is out of range.
    pub fn from_mm(d: f32, axis: Axis, cfg: &SystemConfig) -> Result<Self, ParseError> {
        if !defaults::MM_DISP_RANGE.contains(&d) {
            return Err(ParseError::DisplacementRange);
        }

        let steps = match axis {
            Axis::X => libm::roundf(d / cfg.mmps.0),
            Axis::Y => libm::roundf(d / cfg.mmps.1),
            Axis::Z => libm::roundf(d / cfg.mmps.2),
        } as i32;

        match cfg.frame {
            Frame::Absolute => {
                if steps < 0 {
                    Err(ParseError::DisplacementRange)
                } else {
                    Ok(Displacement::Absolute(steps))
                }
            }
            Frame::Relative => Ok(Displacement::Relative(steps)),
        }
    }

    /// Convert steps to a displacement. Returns an error if the input is out of range.
    pub fn from_steps(d: i32, cfg: &SystemConfig) -> Result<Self, ParseError> {
        if !defaults::STEP_DISP_RANGE.contains(&d) {
            return Err(ParseError::DisplacementRange);
        }

        match cfg.frame {
            Frame::Absolute => {
                if d < 0 {
                    Err(ParseError::DisplacementRange)
                } else {
                    Ok(Displacement::Absolute(d))
                }
            }
            Frame::Relative => Ok(Displacement::Relative(d)),
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
