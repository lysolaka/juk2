//! RGB LED controller
use esp_hal::{
    ledc::{LSGlobalClkSource, Ledc, LowSpeed, timer, timer::TimerIFace},
    time::Rate,
};
use juk_led::LEDAdapter;

use crate::{LedResources, global};

#[embassy_executor::task]
pub async fn led_control(led: LedResources<'static>) -> ! {
    let mut ledc = Ledc::new(led.ctrl);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut tim0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    defmt::expect!(
        tim0.configure(timer::config::Config {
            duty: timer::config::Duty::Duty8Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(25),
        }),
        "LEDC timer configuration failed"
    );

    let mut led = defmt::expect!(
        LEDAdapter::new(&ledc, &tim0, led.r, led.g, led.b),
        "LEDC channel configuration failed"
    );

    loop {
        let (r, g, b) = global::LED.wait().await;
        led.set_color(r, g, b);
    }
}
