//! Code running on the second core (app core)

mod limits;

use embassy_executor::Spawner;

use crate::{LimitsResources, MotorResources};

#[embassy_executor::task]
pub async fn main(
    spawner: Spawner,
    limits: LimitsResources<'static>,
    motor: MotorResources<'static>,
) -> ! {
    // TODO: spawn some tasks
    let _ = spawner;


    limits::init(limits);
    // discard warnings
    let _ = motor;

    loop {}
}
