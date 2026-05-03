//! Code running on the main core
use embassy_executor::Spawner;

use crate::{ComResources, LedResources};

#[embassy_executor::task]
pub async fn main(spawner: Spawner, com: ComResources<'static>, led: LedResources<'static>) -> ! {
    // TODO: spawn some tasks
    let _ = spawner;

    // discard warnings
    let (_, _) = (led, com);

    loop {}
}
