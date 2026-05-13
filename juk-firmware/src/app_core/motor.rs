//! Motion executor
use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};

use critical_section::Mutex;
use embassy_futures::select::{Either, Either3, select, select3};
use esp_hal::{
    Blocking,
    gpio::{Level, Output, OutputConfig},
    time::Duration,
    timer::{OneShotTimer, timg::TimerGroup},
};
use heapless::spsc::{Consumer, Producer, Queue};

use juk_interp::Step;

use crate::{MotorResources, global, global::LimitStatus};

/// Pin for moving the X axis
static X_STEP: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));
/// Pin for moving the Y axis
static Y_STEP: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));
/// Pin for moving the Z axis
static Z_STEP: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));

/// Pin for setting the X axis direction
static X_DIR: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));
/// Pin for setting the Y axis direction
static Y_DIR: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));
/// Pin for setting the Z axis direction
static Z_DIR: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));

/// Step producer and consumer queue
static STEP_PC: (
    Mutex<RefCell<Option<Producer<'_, StepEvent>>>>,
    Mutex<RefCell<Option<Consumer<'_, StepEvent>>>>,
) = {
    static mut Q: Queue<StepEvent, 8> = Queue::new();
    // SAFETY: `Q` is only accessible in this scope
    #[allow(static_mut_refs)]
    let (p, c) = unsafe { Q.split_const() };

    (
        Mutex::new(RefCell::new(Some(p))),
        Mutex::new(RefCell::new(Some(c))),
    )
};

/// Abort condition variable, used to flush the step queue and abort movement
static ABORT_COND: AtomicBool = AtomicBool::new(false);

/// Pin set timer
static STEP_SET_TIM: Mutex<RefCell<Option<OneShotTimer<'_, Blocking>>>> =
    Mutex::new(RefCell::new(None));
/// Pin reset timer
static STEP_RESET_TIM: Mutex<RefCell<Option<OneShotTimer<'_, Blocking>>>> =
    Mutex::new(RefCell::new(None));

pub fn init(res: MotorResources<'static>) -> MotorControl<'static> {
    // configure timers
    let timg = TimerGroup::new(res.tim);

    let mut set_tim = OneShotTimer::new(timg.timer0);
    set_tim.set_interrupt_handler(step_set);
    set_tim.listen();

    let mut reset_tim = OneShotTimer::new(timg.timer1);
    reset_tim.set_interrupt_handler(step_reset);
    reset_tim.listen();

    // configure GPIO
    let x_step = Output::new(res.x_step, Level::Low, OutputConfig::default());
    let y_step = Output::new(res.y_step, Level::Low, OutputConfig::default());
    let z_step = Output::new(res.z_step, Level::Low, OutputConfig::default());

    let x_dir = Output::new(res.x_dir, Level::Low, OutputConfig::default());
    let y_dir = Output::new(res.y_dir, Level::Low, OutputConfig::default());
    let z_dir = Output::new(res.z_dir, Level::Low, OutputConfig::default());

    // install the statics
    critical_section::with(|cs| {
        X_STEP.borrow_ref_mut(cs).replace(x_step);
        Y_STEP.borrow_ref_mut(cs).replace(y_step);
        Z_STEP.borrow_ref_mut(cs).replace(z_step);

        X_DIR.borrow_ref_mut(cs).replace(x_dir);
        Y_DIR.borrow_ref_mut(cs).replace(y_dir);
        Z_DIR.borrow_ref_mut(cs).replace(z_dir);

        STEP_SET_TIM.borrow_ref_mut(cs).replace(set_tim);
        STEP_RESET_TIM.borrow_ref_mut(cs).replace(reset_tim);
    });

    // get the producer side of the queue
    let prod = critical_section::with(|cs| STEP_PC.0.borrow_ref_mut(cs).take().unwrap());

    MotorControl {
        drv: Output::new(res.en_drv, Level::High, OutputConfig::default()),
        led: Output::new(res.en_led, Level::Low, OutputConfig::default()),
        step_queue: prod,
    }
}

pub struct MotorControl<'d> {
    /// Motor driver IC enable line, active low
    drv: Output<'d>,
    /// LED indicator for the enable line
    led: Output<'d>,
    step_queue: Producer<'static, StepEvent>,
}

impl<'d> MotorControl<'d> {
    /// Execute the step and delay sequence, but return when a limit switch is hit or the user
    /// cancels the operation
    pub async fn execute<SI, DI>(&mut self, step_iter: SI, delay_iter: DI)
    where
        SI: Iterator<Item = (Step, Step, Step)>,
        DI: Iterator<Item = f32>,
    {
        // reset the abort state
        ABORT_COND.store(false, Ordering::Release);
        // reset any stale cancel signals
        global::CANCEL.reset();
        global::LIMIT_CANCEL.reset();
        // enable the driver
        self.drv.set_low();
        self.led.set_high();
        match select3(
            self.execute_impl(step_iter, delay_iter),
            global::CANCEL.wait(),
            global::LIMIT_CANCEL.wait(),
        )
        .await
        {
            Either3::First(_) => defmt::info!("Movement complete"),
            Either3::Second(_) => {
                ABORT_COND.store(true, Ordering::Release);
                defmt::warn!("Movement aborted: cancelled by the user");
            }
            Either3::Third(_) => {
                ABORT_COND.store(true, Ordering::Release);
                defmt::error!("Movement aborted: hit a limit switch");
            }
        }
        self.drv.set_high();
        self.led.set_low();
    }

    /// Execute the step and delay sequence, but return when a limit switch is hit. This version of
    /// `execute()` is only suitable for the homing sequence.
    ///
    /// # Warning
    ///
    /// This function does not check for the user cancel signal, the caller is responsible for doing
    /// that.
    pub async fn execute_homing<SI, DI>(&mut self, step_iter: SI, delay_iter: DI)
    where
        SI: Iterator<Item = (Step, Step, Step)>,
        DI: Iterator<Item = f32>,
    {
        // reset the abort state
        ABORT_COND.store(false, Ordering::Release);
        // reset any stale cancel signal
        global::LIMIT_CANCEL.reset();
        // enable the driver
        self.drv.set_low();
        self.led.set_high();
        match select(
            self.execute_impl(step_iter, delay_iter),
            global::LIMIT_CANCEL.wait(),
        )
        .await
        {
            Either::First(_) => (),
            Either::Second(_) => {
                ABORT_COND.store(true, Ordering::Release);
            }
        }
        self.drv.set_high();
        self.led.set_low();
    }

    async fn execute_impl<SI, DI>(&mut self, step_iter: SI, delay_iter: DI)
    where
        SI: Iterator<Item = (Step, Step, Step)>,
        DI: Iterator<Item = f32>,
    {
        // switch the unit of time to microseconds
        let mut delay_iter = delay_iter.map(|d| (d * 1_000_000.0) as u64 - 2);
        // get the first delay
        let d = defmt::expect!(delay_iter.next(), "Delay iter should not be empty");
        // schedule the first step
        critical_section::with(|cs| {
            let mut tim = STEP_SET_TIM.borrow_ref_mut(cs);
            let tim = defmt::expect!(tim.as_mut(), "The timer should be set up");
            defmt::unwrap!(tim.schedule(Duration::from_micros(d)));
        });

        // enqueue the steps
        for step in step_iter {
            // if no space in the queue, give some time to the executor
            while !self.step_queue.ready() {
                embassy_futures::yield_now().await;
            }
            // should always return `Ok` since it's ready
            let _ = self.step_queue.enqueue(StepEvent {
                step,
                delay: delay_iter.next(),
            });
        }

        // at the end wait for the queue to be empty
        while !self.step_queue.is_empty() {
            embassy_futures::yield_now().await;
        }
    }
}

#[derive(Clone, Copy)]
struct StepEvent {
    step: (Step, Step, Step),
    delay: Option<u64>,
}

#[esp_hal::handler]
fn step_set() {
    // obtain our side of the queue
    let step_queue = {
        static mut R: Option<Consumer<'_, StepEvent>> = None;
        // SAFETY: Mutable access to `R` is allowed exclusively in this scope
        // and the ISR cannot be called directly or preempt itself
        #[allow(static_mut_refs)]
        unsafe {
            &mut R
        }
    }
    .get_or_insert_with(|| {
        critical_section::with(|cs| defmt::unwrap!(STEP_PC.1.borrow_ref_mut(cs).take()))
    });

    critical_section::with(|cs| {
        let mut set_tim = STEP_SET_TIM.borrow_ref_mut(cs);
        let set_tim = defmt::unwrap!(set_tim.as_mut());

        let mut res_tim = STEP_RESET_TIM.borrow_ref_mut(cs);
        let res_tim = defmt::unwrap!(res_tim.as_mut());

        let mut x_step = X_STEP.borrow_ref_mut(cs);
        let x_step = defmt::unwrap!(x_step.as_mut());
        let mut x_dir = X_DIR.borrow_ref_mut(cs);
        let x_dir = defmt::unwrap!(x_dir.as_mut());

        let mut y_step = Y_STEP.borrow_ref_mut(cs);
        let y_step = defmt::unwrap!(y_step.as_mut());
        let mut y_dir = Y_DIR.borrow_ref_mut(cs);
        let y_dir = defmt::unwrap!(y_dir.as_mut());

        let mut z_step = Z_STEP.borrow_ref_mut(cs);
        let z_step = defmt::unwrap!(z_step.as_mut());
        let mut z_dir = Z_DIR.borrow_ref_mut(cs);
        let z_dir = defmt::unwrap!(z_dir.as_mut());

        let limits = global::LIMITS.borrow_ref(cs);

        let mut pos = global::POS.borrow_ref_mut(cs);

        // clear the interrupt
        set_tim.clear_interrupt();

        // get the step event
        let event = match step_queue.dequeue() {
            Some(e) => e,
            // either this IRQ is unwanted or the events are not coming in fast enough
            None => return,
        };

        // check if we've been cancelled
        if ABORT_COND.load(Ordering::Acquire) {
            // reset pins
            x_step.set_low();
            y_step.set_low();
            z_step.set_low();
            x_dir.set_low();
            y_dir.set_low();
            z_dir.set_low();
            // clear the queue
            while step_queue.dequeue().is_some() {}
            return;
        }

        // set the direction
        x_dir.set_level(event.step.0.dir());
        y_dir.set_level(event.step.1.dir());
        z_dir.set_level(event.step.2.dir());

        // let it set up for 2 us, we need a blocking delay so let's use the embassy time driver
        embassy_time::block_for(embassy_time::Duration::from_micros(2));

        // do the steps
        if event.step.0 == Step::Positive && !limits.contains(LimitStatus::PX) {
            x_step.set_level(event.step.0.step_level());
            pos.0 += event.step.0 as i32;
        } else if event.step.0 == Step::Negative && !limits.contains(LimitStatus::NX) {
            x_step.set_level(event.step.0.step_level());
            pos.0 += event.step.0 as i32;
        }

        if event.step.1 == Step::Positive && !limits.contains(LimitStatus::PY) {
            y_step.set_level(event.step.1.step_level());
            pos.1 += event.step.1 as i32;
        } else if event.step.1 == Step::Negative && !limits.contains(LimitStatus::NY) {
            y_step.set_level(event.step.1.step_level());
            pos.1 += event.step.1 as i32;
        }

        if event.step.2 == Step::Positive && !limits.contains(LimitStatus::PZ) {
            z_step.set_level(event.step.2.step_level());
            pos.2 += event.step.2 as i32;
        } else if event.step.2 == Step::Negative && !limits.contains(LimitStatus::NZ) {
            z_step.set_level(event.step.2.step_level());
            pos.2 += event.step.2 as i32;
        }

        // schedule the pin reset
        defmt::unwrap!(res_tim.schedule(Duration::from_micros(4)));

        // schedule the next step (if any)
        if let Some(delay) = event.delay {
            defmt::unwrap!(set_tim.schedule(Duration::from_micros(delay)));
        }
    });
}

#[esp_hal::handler]
fn step_reset() {
    critical_section::with(|cs| {
        if let Some(pin) = X_STEP.borrow_ref_mut(cs).as_mut() {
            pin.set_low();
        }
        if let Some(pin) = Y_STEP.borrow_ref_mut(cs).as_mut() {
            pin.set_low();
        }
        if let Some(pin) = Z_STEP.borrow_ref_mut(cs).as_mut() {
            pin.set_low();
        }
        if let Some(tim) = STEP_RESET_TIM.borrow_ref_mut(cs).as_mut() {
            tim.clear_interrupt();
        }
    });
}
