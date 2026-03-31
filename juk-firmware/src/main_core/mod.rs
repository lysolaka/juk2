use embassy_executor::Spawner;

use crate::{ComResources, LedResources};

#[embassy_executor::task]
pub async fn main(spawner: Spawner, com: ComResources<'static>, led: LedResources<'static>) -> ! {
    // TODO: spawn some tasks
    let _ = spawner;

    // discard warnings
    let (_, _) = (com, led);

    loop {
        defmt::info!("Hello from the main core!");
        embassy_time::Timer::after_secs(5).await;
    }
}
