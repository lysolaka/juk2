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

    // discard warnings
    let (_, _) = (limits, motor);

    loop {
        embassy_time::Timer::after_secs(15).await;
        let s = alloc::format!(
            "{}Some random info message! Hope it didn't interrupt you\r\n",
            crate::strings::INFO
        );
        crate::TEST_CH.send(s).await;
        embassy_time::Timer::after_secs(20).await;
        let s = alloc::format!(
            "{}Some random warn message! This one surely didn't interrupt you\r\n{}It even has a second line\r\n",
            crate::strings::WARN,
            crate::strings::INFO
        );
        crate::TEST_CH.send(s).await;
    }
}
