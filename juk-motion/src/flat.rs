//! The flat motion profile

use crate::Profile;

/// A constant velocity motion profile.
///
/// This profile is unsuitable for rapid movement, but may be helpful in precise positioning or
/// calibration.
pub struct Flat {
    delay: f32,
    steps: u32,
}

impl Flat {
    /// Construct a new flat profile from the velocity and steps to travel.
    pub fn new(v_max: f32, steps: u32) -> Self {
        Self {
            delay: 1.0 / v_max,
            steps,
        }
    }
}

impl Profile for Flat {
    fn next_delay(&mut self) -> Option<f32> {
        if self.steps > 0 {
            self.steps -= 1;
            Some(self.delay)
        } else {
            None
        }
    }
}
