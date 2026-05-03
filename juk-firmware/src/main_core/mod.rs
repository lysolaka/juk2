//! Code running on the main core

mod led;

use embassy_executor::Spawner;

use crate::{ComResources, LedResources};

#[embassy_executor::task]
pub async fn main(spawner: Spawner, com: ComResources<'static>, led: LedResources<'static>) -> ! {
    // spawn the LED controller
    defmt::expect!(
        spawner.spawn(led::led_control(led)),
        "Failed to spawn the LED controller task"
    );

    // discard warnings
    let _ = com;

    loop {}
}
