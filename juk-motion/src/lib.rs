//! Motion profiles and interpolators for JUK2
#![no_std]

pub mod interp;
pub mod prof;

/// Errors, which can be encountered when using `juk-motion`.
#[derive(Debug, defmt::Format, thiserror::Error)]
pub enum Error {
    /// The movement is a no-op
    #[error("the movement is a no-op")]
    ZeroDisplacement,
    /// The velocity is 0: division by 0 is imminent
    #[error("the velocity is 0: division by 0 is imminent")]
    ZeroVelocity,
    /// The acceleration is 0: division by 0 is imminent
    #[error("the acceleration is 0: division by 0 is imminent")]
    ZeroAcceleration,
    /// The arc is impossible
    #[error("the arc is impossible")]
    ImpossibleGeometry,
    /// The radius is 0: this is not an arc
    #[error("the radius is 0: this is not an arc")]
    ZeroRadius,
}
