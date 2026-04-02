//! A simple RGB LED controller, which allows for setting a color.
//!
//! # Example
//!
//! ```no_run
//! use esp_hal::{
//!     ledc::{
//!         LSGlobalClkSource,
//!         Ledc,
//!         LowSpeed,
//!         timer,
//!         timer::TimerIFace,
//!     },
//!     time::Rate,
//! };
//! 
//! use juk_led::LEDAdapter;
//! 
//! // configure the LEDC peripheral
//! let mut ledc = Ledc::new(peripheral.LEDC);
//! ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
//! 
//! // configure the timer
//! let mut tim0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
//! tim0.configure(timer::config::Config {
//!     duty: timer::config::Duty::Duty8Bit,
//!     clock_source: timer::LSClockSource::APBClk,
//!     frequency: Rate::from_khz(25),
//! })?;
//! 
//! // construct the adapter
//! let mut rgb_led = LEDAdapter::new(&ledc, &tim0, red_pin, green_pin, blue_pin)?;
//! 
//! // use it
//! rgb_led.set_color(0xff, 0x80, 0x80);
//! ```
#![no_std]

use esp_hal::{
    gpio::{DriveMode, interconnect::PeripheralOutput},
    ledc::{
        Ledc,
        LowSpeed,
        channel,
        channel::{Channel, ChannelHW, ChannelIFace},
        timer::{Timer, TimerHW, TimerIFace},
    },
    time::Rate,
};

/// An RGB LED adapter. Designed to control a common cathode LED.
pub struct LEDAdapter<'d> {
    r: Channel<'d, LowSpeed>,
    g: Channel<'d, LowSpeed>,
    b: Channel<'d, LowSpeed>,
}

impl<'d> LEDAdapter<'d> {
    /// Construct the [`LEDAdapter`]. Before calling this function ensure that the timer is
    /// configured.
    pub fn new(
        ledc: &Ledc<'d>,
        tim: &'d Timer<'d, LowSpeed>,
        red: impl PeripheralOutput<'d>,
        green: impl PeripheralOutput<'d>,
        blue: impl PeripheralOutput<'d>,
    ) -> Result<Self, channel::Error> {
        defmt::info!(
            "LED Timer: freq_hw = {:?}",
            tim.freq_hw().unwrap_or(Rate::from_hz(0))
        );
        defmt::info!("LED Timer: freq = {=u32} Hz", tim.frequency());

        let max_duty = if let Some(d) = tim.duty() {
            d as u32
        } else {
            0
        };
        defmt::info!("LED Timer: max_duty = {=u32}", (1 << max_duty) - 1);

        let mut ch_red = ledc.channel(channel::Number::Channel0, red);
        ch_red.configure(channel::config::Config {
            timer: tim,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })?;

        let mut ch_green = ledc.channel(channel::Number::Channel1, green);
        ch_green.configure(channel::config::Config {
            timer: tim,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })?;

        let mut ch_blue = ledc.channel(channel::Number::Channel2, blue);
        ch_blue.configure(channel::config::Config {
            timer: tim,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })?;

        Ok(Self {
            r: ch_red,
            g: ch_green,
            b: ch_blue,
        })
    }

    /// Set the LED color using the RGB8 format
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.r.set_duty_hw(r as u32);
        self.g.set_duty_hw(g as u32);
        self.b.set_duty_hw(b as u32);
    }
}
