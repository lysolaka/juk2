//! Code running on the second core (app core)

mod limits;
mod motor;

use embassy_executor::Spawner;
use embassy_time::Timer;

use juk_cmd::{Axis, Displacement, MotionError, cmd::Command};
use juk_motion::{
    interp::LineGenerator,
    prof::{FastTrap, Flat, Profile},
};

use crate::{LimitsResources, MotorResources, global};

#[embassy_executor::task]
pub async fn main(
    spawner: Spawner,
    limits: LimitsResources<'static>,
    motor: MotorResources<'static>,
) -> ! {
    // TODO: spawn some tasks
    let _ = spawner;

    let mut motorctl = motor::init(motor);

    // wait for the stuff to settle
    Timer::after_millis(20).await;
    limits::init(limits);

    defmt::info!("Motor control unit started");

    loop {
        match global::MOVEMENT.receive().await {
            Command::Move { x, y, z, a, v } => {
                // FIXME: add proper error handling
                defmt::unwrap!(run_move(&mut motorctl, x, y, z, a, v).await);
            }
            #[allow(unused_variables)]
            Command::Arc {
                x,
                y,
                z,
                r,
                dir,
                a,
                v,
            } => defmt::todo!(),
            #[allow(unused_variables)]
            Command::Home { x, y, z } => defmt::todo!(),
            _ => defmt::unreachable!(),
        }
    }
}

async fn run_move(
    ctl: &mut motor::MotorControl<'_>,
    x: Displacement,
    y: Displacement,
    z: Displacement,
    a: f32,
    v: f32,
) -> Result<(), MotionError> {
    // we can copy the position since we're not moving at this point in time
    let pos = critical_section::with(|cs| *global::POS.borrow_ref(cs));

    let dx = x.to_relative(Axis::X, pos);
    let dy = y.to_relative(Axis::Y, pos);
    let dz = z.to_relative(Axis::Z, pos);

    let line = LineGenerator::new(dx, dy, dz)?;

    // no acceleration means the flat profile
    if a == 0.0 {
        let mut prof = Flat::new(v, line.len() as u32)?;
        ctl.execute(line.step_iter(), prof.delays()).await;
        Ok(())
    } else {
        let mut prof = FastTrap::new(a, v, line.len() as u32)?;
        ctl.execute(line.step_iter(), prof.delays()).await;
        Ok(())
    }
}
