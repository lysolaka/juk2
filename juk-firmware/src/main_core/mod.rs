//! Code running on the main core

mod com;
mod led;

use embassy_executor::Spawner;
use embassy_time::Timer;

use crate::{ComResources, LedResources, global};

#[embassy_executor::task]
pub async fn main(spawner: Spawner, com: ComResources<'static>, led: LedResources<'static>) -> ! {
    // spawn the LED controller
    defmt::expect!(
        spawner.spawn(led::led_control(led)),
        "Failed to spawn the LED controller task"
    );

    // wait for stuff before enabling the interface
    Timer::after_micros(20).await;

    // spawn the COM controller
    defmt::expect!(
        spawner.spawn(com::com_control(com)),
        "Failed to spawn the communication interface task"
    );

    // signal the default LED color
    let rgb = {
        let cfg = global::SYSCFG.lock().await;
        cfg.led
    };
    global::LED.signal(rgb_to_tuple(rgb));

    defmt::info!("Communication interface started");

    // run the easter egg in the main loop
    loop {
        let _ = global::EGG.wait().await;
        // save the current color
        let rgb = {
            let cfg = global::SYSCFG.lock().await;
            cfg.led
        };

        // 10 iterations
        for hue in (0..=255).cycle().take(256 * 10) {
            global::LED.signal(wheel(hue));
            Timer::after_millis(20).await;
        }

        // restore the saved color
        global::LED.signal(rgb_to_tuple(rgb));
    }
}

/// Convert a u32 to an RGB8 tuple
#[inline]
fn rgb_to_tuple(rgb: u32) -> (u8, u8, u8) {
    let r = ((rgb >> 16) & 0xff) as u8;
    let g = ((rgb >> 8) & 0xff) as u8;
    let b = (rgb & 0xff) as u8;

    (r, g, b)
}

/// Color wheel function for the easter egg rainbow
fn wheel(h: u8) -> (u8, u8, u8) {
    match h {
        0..=84 => {
            let p = h * 3;
            (255 - p, p, 0)
        }
        85..=169 => {
            let p = (h - 85) * 3;
            (0, 255 - p, p)
        }
        _ => {
            let p = (h - 170) * 3;
            (p, 0, 255 - p)
        }
    }
}
