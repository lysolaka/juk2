//! Limit switch handling
use core::cell::RefCell;

use esp_hal::gpio::{Event, Input, InputConfig, Io, Level, Pull};

use critical_section::Mutex;

use crate::{LimitsResources, global, global::LimitStatus};

static X_P: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static X_M: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static Y_P: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static Y_M: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static Z_P: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static Z_M: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

pub fn init(limits: LimitsResources<'static>) {
    let config = InputConfig::default().with_pull(Pull::Up);

    // init the input drivers
    let mut x_p = Input::new(limits.x_p, config);
    let mut x_m = Input::new(limits.x_m, config);
    let mut y_p = Input::new(limits.y_p, config);
    let mut y_m = Input::new(limits.y_m, config);
    let mut z_p = Input::new(limits.z_p, config);
    let mut z_m = Input::new(limits.z_m, config);

    // check the current limit switch state
    critical_section::with(|cs| {
        let mut status = global::LIMITS.borrow_ref_mut(cs);

        if x_p.is_low() {
            status.insert(LimitStatus::PX);
        }
        if x_m.is_low() {
            status.insert(LimitStatus::NX);
        }
        if y_p.is_low() {
            status.insert(LimitStatus::PY);
        }
        if y_m.is_low() {
            status.insert(LimitStatus::NY);
        }
        if z_p.is_low() {
            status.insert(LimitStatus::PZ);
        }
        if z_m.is_low() {
            status.insert(LimitStatus::NZ);
        }

        defmt::info!(
            "Initial limits state: X+: {0=0..1}, X-: {0=1..2}; Y+: {0=2..3}, Y-: {0=3..4}; Z+: {0=4..5}, Z-: {0=5..6}",
            status.bits()
        );
    });

    let mut io_mux = Io::new(limits.io);
    io_mux.set_interrupt_handler(limits_isr);

    // install the drivers into the statics
    critical_section::with(|cs| {
        x_p.listen(Event::AnyEdge);
        X_P.borrow_ref_mut(cs).replace(x_p);
        x_m.listen(Event::AnyEdge);
        X_M.borrow_ref_mut(cs).replace(x_m);
        y_p.listen(Event::AnyEdge);
        Y_P.borrow_ref_mut(cs).replace(y_p);
        y_m.listen(Event::AnyEdge);
        Y_M.borrow_ref_mut(cs).replace(y_m);
        z_p.listen(Event::AnyEdge);
        Z_P.borrow_ref_mut(cs).replace(z_p);
        z_m.listen(Event::AnyEdge);
        Z_M.borrow_ref_mut(cs).replace(z_m);
    });
}

#[esp_hal::handler]
fn limits_isr() {
    critical_section::with(|cs| {
        let mut status = global::LIMITS.borrow_ref_mut(cs);

        handle_limit(&mut X_P.borrow_ref_mut(cs), LimitStatus::PX, &mut status);
        handle_limit(&mut X_M.borrow_ref_mut(cs), LimitStatus::NX, &mut status);
        handle_limit(&mut Y_P.borrow_ref_mut(cs), LimitStatus::PY, &mut status);
        handle_limit(&mut Y_M.borrow_ref_mut(cs), LimitStatus::NY, &mut status);
        handle_limit(&mut Z_P.borrow_ref_mut(cs), LimitStatus::PZ, &mut status);
        handle_limit(&mut Z_M.borrow_ref_mut(cs), LimitStatus::NZ, &mut status);

        defmt::trace!(
            "Limits IRQ fired, status: X+: {0=0..1}, X-: {0=1..2}; Y+: {0=2..3}, Y-: {0=3..4}; Z+: {0=4..5}, Z-: {0=5..6}",
            status.bits()
        );
    });
}

fn handle_limit(pin: &mut Option<Input>, flag: LimitStatus, status: &mut LimitStatus) {
    // get the pin
    let Some(pin) = pin.as_mut() else {
        return;
    };
    // check if it's the pin we need to handle
    if !pin.is_interrupt_set() {
        return;
    }

    // the interrupt triggers on any edge, we need to check which one is it
    match pin.level() {
        // if pressed set the endstop flag
        Level::Low => {
            status.insert(flag);
            global::CANCEL.signal(());
        }
        // if not remove it
        Level::High => status.remove(flag),
    }

    pin.clear_interrupt();
}
