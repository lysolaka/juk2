//! JUK2 command definitions and parsing
#![no_std]

extern crate alloc;

pub mod cmd;
pub mod config;
pub mod defaults;

use serde::{Deserialize, Serialize};

/// Errors, which can be encountered when parsing commands.
#[cfg(not(feature = "export"))]
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, thiserror::Error, Deserialize, Serialize)]
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
