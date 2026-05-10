//! Code running on the second core (app core)

mod limits;

use embassy_executor::Spawner;
use embassy_time::Timer;

use crate::{LimitsResources, MotorResources};

#[embassy_executor::task]
pub async fn main(
    spawner: Spawner,
    limits: LimitsResources<'static>,
    motor: MotorResources<'static>,
) -> ! {
    // TODO: spawn some tasks
    let _ = spawner;

    // discard warnings
    let _ = motor;

    // wait for the stuff to settle
    Timer::after_millis(20).await;
    limits::init(limits);

    defmt::info!("Motor control unit started");

    loop {}
}
