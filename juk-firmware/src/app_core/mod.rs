//! Code running on the second core (app core)

mod limits;
mod motor;

use embassy_executor::Spawner;
use embassy_time::Timer;

use juk_cmd::{ArcDir, Axis, Displacement, MotionError, cmd::Command, defaults};
use juk_motion::{
    interp::{ArcGenerator, LineGenerator},
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
        // FIXME: add proper error handling
        match global::MOVEMENT.receive().await {
            Command::Move { x, y, z, a, v } => {
                defmt::unwrap!(run_move(&mut motorctl, x, y, z, a, v).await)
            }
            Command::Arc {
                x,
                y,
                z,
                r,
                dir,
                a,
                v,
            } => defmt::unwrap!(run_arc(&mut motorctl, x, y, z, r, dir, a, v).await),
            Command::Home { x, y, z } => defmt::unwrap!(run_homing(&mut motorctl, x, y, z).await),
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

    defmt::debug!(
        "Begin line movement, dx: {=i32}, dy: {=i32}, dz: {=i32}",
        dx,
        dy,
        dz,
    );

    // no acceleration means the flat profile
    if a == 0.0 {
        let mut prof = Flat::new(v, line.len() as u32)?;
        ctl.execute(line.step_iter(), prof.delays()).await;
    } else {
        let mut prof = FastTrap::new(a, v, line.len() as u32)?;
        ctl.execute(line.step_iter(), prof.delays()).await;
    }

    defmt::debug!("Line movement done");

    Ok(())
}

async fn run_arc(
    ctl: &mut motor::MotorControl<'_>,
    x: Displacement,
    y: Displacement,
    z: Displacement,
    r: u32,
    dir: ArcDir,
    a: f32,
    v: f32,
) -> Result<(), MotionError> {
    // we can copy the position since we're not moving at this point in time
    let pos = critical_section::with(|cs| *global::POS.borrow_ref(cs));

    let dx = x.to_relative(Axis::X, pos);
    let dy = y.to_relative(Axis::Y, pos);
    let dz = z.to_relative(Axis::Z, pos);

    defmt::debug!(
        "Begin arc movement, dx: {=i32}, dy: {=i32}, dz: {=i32}, r: {=u32}, dir: {:?}",
        dx,
        dy,
        dz,
        r,
        dir
    );

    let arc = ArcGenerator::new(dx, dy, dz, r, dir)?;

    // no acceleration means the flat profile
    if a == 0.0 {
        let mut prof = Flat::new(v, arc.len() as u32)?;
        ctl.execute(arc.step_iter(), prof.delays()).await;
    } else {
        let mut prof = FastTrap::new(a, v, arc.len() as u32)?;
        ctl.execute(arc.step_iter(), prof.delays()).await;
    }

    defmt::debug!("Arc movement done");

    Ok(())
}

async fn run_homing(
    ctl: &mut motor::MotorControl<'_>,
    x: bool,
    y: bool,
    z: bool,
) -> Result<(), MotionError> {
    defmt::debug!(
        "Begin homing sequence, X: {=bool}, Y: {=bool}, Z: {=bool}",
        x,
        y,
        z
    );

    // home the x axis
    if x {
        // move back to the limit, 100% guarantee of hitting the limit switch
        let line = LineGenerator::new(-50000, 0, 0)?;
        let mut prof = FastTrap::new(defaults::ACCEL, defaults::VEL, 50000)?;
        ctl.execute(line.step_iter(), prof.delays()).await;
        // move away
        let line = LineGenerator::new(200, 0, 0)?;
        let mut prof = FastTrap::new(defaults::ACCEL, defaults::VEL, 200)?;
        ctl.execute(line.step_iter(), prof.delays()).await;

        // set the position to 0
        critical_section::with(|cs| global::POS.borrow_ref_mut(cs).0 = 0);
    }

    // home the y axis
    if y {
        // move back to the limit, 100% guarantee of hitting the limit switch
        let line = LineGenerator::new(0, -50000, 0)?;
        let mut prof = FastTrap::new(defaults::ACCEL, defaults::VEL, 50000)?;
        ctl.execute(line.step_iter(), prof.delays()).await;
        // move away
        let line = LineGenerator::new(0, 200, 0)?;
        let mut prof = FastTrap::new(defaults::ACCEL, defaults::VEL, 200)?;
        ctl.execute(line.step_iter(), prof.delays()).await;

        // set the position to 0
        critical_section::with(|cs| global::POS.borrow_ref_mut(cs).1 = 0);
    }

    // home the z axis
    if z {
        // move back to the limit, 100% guarantee of hitting the limit switch
        let line = LineGenerator::new(0, 0, -100000)?;
        let mut prof = FastTrap::new(defaults::ACCEL, defaults::VEL, 100000)?;
        ctl.execute(line.step_iter(), prof.delays()).await;
        // move away
        let line = LineGenerator::new(0, 0, 1500)?;
        let mut prof = FastTrap::new(defaults::ACCEL, defaults::VEL, 1500)?;
        ctl.execute(line.step_iter(), prof.delays()).await;

        // set the position to 0
        critical_section::with(|cs| global::POS.borrow_ref_mut(cs).2 = 0);
    }

    defmt::debug!("Homing complete");

    Ok(())
}
