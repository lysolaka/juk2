#![no_std]

use esp_hal::peripherals::*;

pub mod app_core;
pub mod main_core;

// to keep the code "clean" we need to define the resources struct again for the sake of making it
// visible to the entire crate, not just the main function

/// Resources struct
pub struct Resources<'d> {
    pub com: ComResources<'d>,
    pub led: LedResources<'d>,
    pub limits: LimitsResources<'d>,
    pub motor: MotorResources<'d>,
}

/// User communication interface resources
pub struct ComResources<'d> {
    pub uart: UART0<'d>,
}

/// RGB indicator LED resources
pub struct LedResources<'d> {
    pub r: GPIO17<'d>,
    pub g: GPIO18<'d>,
    pub b: GPIO8<'d>,
    pub ctrl: LEDC<'d>,
}

/// Limit switches resources
pub struct LimitsResources<'d> {
    pub x_p: GPIO4<'d>,
    pub x_m: GPIO5<'d>,
    pub y_p: GPIO6<'d>,
    pub y_m: GPIO7<'d>,
    pub z_p: GPIO15<'d>,
    pub z_m: GPIO16<'d>,
}

/// Motor control resources
pub struct MotorResources<'d> {
    pub en_drv: GPIO45<'d>,
    pub en_led: GPIO21<'d>,
    pub x_step: GPIO10<'d>,
    pub x_dir: GPIO9<'d>,
    pub y_step: GPIO12<'d>,
    pub y_dir: GPIO11<'d>,
    pub z_step: GPIO13<'d>,
    pub z_dir: GPIO14<'d>,
}
