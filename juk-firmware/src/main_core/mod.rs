use embassy_executor::Spawner;

use esp_hal::{
    ledc::{
        LSGlobalClkSource,
        Ledc,
        LowSpeed,
        timer,
        timer::TimerIFace,
    },
    time::Rate,
};

use juk_led::LEDAdapter;

use crate::{ComResources, LedResources};

#[embassy_executor::task]
pub async fn main(spawner: Spawner, com: ComResources<'static>, led: LedResources<'static>) -> ! {
    // TODO: spawn some tasks
    let _ = spawner;

    // discard warnings
    let _ = com;

    let mut ledc = Ledc::new(led.ctrl);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut tim0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    defmt::unwrap!(tim0.configure(timer::config::Config {
        duty: timer::config::Duty::Duty8Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: Rate::from_khz(25),
    }));

    let mut rgb_led = defmt::unwrap!(LEDAdapter::new(&ledc, &tim0, led.r, led.g, led.b));

    loop {
        for i in u8::MIN..u8::MAX {
            let pat = wheel(i);
            rgb_led.set_color(pat.0, pat.1, pat.2);
            embassy_time::Timer::after_millis(20).await;
        }
    }
}

// chatgpt special
fn wheel(pos: u8) -> (u8, u8, u8) {
    if pos < 85 {
        let x = pos * 3;
        (255 - x, x, 0)
    } else if pos < 170 {
        let x = (pos - 85) * 3;
        (0, 255 - x, x)
    } else {
        let x = (pos - 170) * 3;
        (x, 0, 255 - x)
    }
}
