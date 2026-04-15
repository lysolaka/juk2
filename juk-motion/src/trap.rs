//! Trapezoidal motion profiles

use crate::Profile;

/// Representation of the current motion phase
#[derive(defmt::Format)]
enum RampPhase {
    /// Not moving
    Idle,
    /// Accelerating
    Accel,
    /// Constant velocity phase
    Hold,
    /// Decelerating (braking)
    Decel,
}

/// "Fast" trapezoidal profile.
///
/// It is only useful if the initial and final velocities are both zero. The calculations for step
/// delays are much faster and the struct itself takes less memory space. The deceleration is equal
/// in magnitude to the acceleration.
#[derive(defmt::Format)]
pub struct FastTrap {
    a: f32,
    d_min: f32,
    d_init: f32,
    d_prev: f32,
    steps: u32,
}

impl FastTrap {
    /// Construct a new trapezoidal profile with the desired acceleration (deceleration), maximum velocity and
    /// steps to travel.
    pub fn new(accel: f32, v_max: f32, steps: u32) -> Self {
        let d_min = 1.0 / v_max;
        let d_init = 1.0 / libm::sqrtf(2.0 * accel);

        Self {
            a: accel,
            d_min,
            d_init,
            d_prev: d_init,
            steps,
        }
    }

    /// Compute the current motion phase.
    fn phase(&self) -> RampPhase {
        if (self.steps == 0) && (self.d_prev >= self.d_init) {
            return RampPhase::Idle;
        }

        let v = 1.0 / self.d_prev;
        let s_stop = (v * v) / (2.0 * self.a);
        let s_stop = libm::ceilf(s_stop) as u32;

        if self.steps <= s_stop {
            return RampPhase::Decel;
        }

        if self.d_prev < self.d_min {
            RampPhase::Decel
        } else if (self.d_prev - self.d_min).abs() < 1e-6 {
            RampPhase::Hold
        } else {
            RampPhase::Accel
        }
    }
}

impl Profile for FastTrap {
    fn next_delay(&mut self) -> Option<f32> {
        let q = self.a * self.d_prev * self.d_prev;
        let addend = 1.5 * q * q;

        let d_next = match self.phase() {
            RampPhase::Idle => return None,
            RampPhase::Accel => {
                let d_next = self.d_prev * (1.0 - q + addend);
                f32::max(d_next, self.d_min)
            }
            RampPhase::Hold => self.d_prev,
            RampPhase::Decel => self.d_prev * (1.0 + q + addend),
        };

        let d_next = f32::min(d_next, self.d_init);

        self.d_prev = d_next;
        self.steps = self.steps.saturating_sub(1);

        Some(d_next)
    }
}

// TODO: docs
#[derive(defmt::Format)]
pub struct Trap {
    a: f32,
    v_init: f32,
    v_max: f32,
    v_end: f32,
    current_step: u32,
    total_steps: u32,
    accel_steps: u32,
    hold_steps: u32,
    decel_steps: u32,
}

impl Trap {
    pub fn new(accel: f32, v_init: f32, v_max: f32, v_end: f32, steps: u32) -> Self {
        let total_steps = steps as f32;

        let accel_dist = (v_max * v_max - v_init * v_init) / (2.0 * accel);
        let decel_dist = (v_max * v_max - v_end * v_end) / (2.0 * accel);

        let (v_max, accel_steps, hold_steps, decel_steps) =
            if accel_dist + decel_dist <= total_steps {
                // trapezoidal profile
                let accel_steps = libm::floorf(accel_dist) as u32;
                let decel_steps = libm::floorf(decel_dist) as u32;
                let hold_steps = steps - accel_steps - decel_steps;

                (v_max, accel_steps, hold_steps, decel_steps)
            } else {
                // triangular profile
                let v_peak = (2.0 * total_steps * accel * accel
                    + accel * v_init * v_init
                    + accel * v_end * v_end)
                    / (2.0 * accel);

                let v_peak = libm::sqrtf(v_peak);

                let accel_steps =
                    libm::floorf((v_peak * v_peak - v_init * v_init) / (2.0 * accel)) as u32;

                let decel_steps = steps - accel_steps;

                (v_peak, accel_steps, 0, decel_steps)
            };

        Self {
            a: accel,
            v_init,
            v_max,
            v_end,
            current_step: 1,
            total_steps: steps,
            accel_steps,
            hold_steps,
            decel_steps,
        }
    }

    fn phase(&self) -> RampPhase {
        if self.current_step > self.total_steps {
            return RampPhase::Idle;
        }

        if self.current_step < self.accel_steps {
            RampPhase::Accel
        } else if self.current_step < self.accel_steps + self.hold_steps {
            RampPhase::Hold
        } else {
            RampPhase::Decel
        }
    }
}

impl Profile for Trap {
    fn next_delay(&mut self) -> Option<f32> {
        let v = match self.phase() {
            RampPhase::Idle => return None,
            RampPhase::Accel => {
                libm::sqrtf(self.v_init * self.v_init + 2.0 * self.a * self.current_step as f32)
            }
            RampPhase::Hold => self.v_max,
            RampPhase::Decel => {
                let remaining = (self.total_steps - self.current_step) as f32;

                libm::sqrtf(self.v_end * self.v_end + 2.0 * self.a * remaining)
            }
        };

        self.current_step += 1;

        Some(1.0 / v)
    }
}
