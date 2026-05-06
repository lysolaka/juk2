//! Default values and range limits definitions
use core::ops::RangeInclusive;

use crate::config::{Frame, SystemConfig, Unit};

pub const MM_DISP_RANGE: RangeInclusive<f32> = -1_000.0..=1_000.0;
pub const STEP_DISP_RANGE: RangeInclusive<i32> = -50_000..=50_000;
pub const LED_RANGE: RangeInclusive<u32> = 0x000000..=0xffffff;
pub const MMPS_RANGE: RangeInclusive<f32> = 0.0..=1.0;
pub const POS_RANGE: RangeInclusive<i32> = 0..=50_000;
pub const MOTION_ARG_RANGE: RangeInclusive<f32> = 0.0..=50_000.0;

// we need a const default, sorry ...
impl SystemConfig {
    pub const fn const_default() -> Self {
        Self {
            accel: 12500.0,
            vel: 6000.0,
            frame: Frame::Relative,
            unit: Unit::Steps,
            mmps: (0.0125625, 0.0125625, 0.0125625),
            mode: juk_com::InterfaceMode::Text,
            led: 0x00ff00,
        }
    }
}

impl Default for SystemConfig {
    fn default() -> Self {
        SystemConfig::const_default()
    }
}
