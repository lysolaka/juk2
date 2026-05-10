#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

extern crate alloc;

use esp_hal::{
    interrupt::software::SoftwareInterruptControl,
    system::Stack,
    timer::systimer::SystemTimer,
};

use esp_rtos::embassy::Executor;

use esp_backtrace as _;
use esp_println as _;

use static_cell::{ConstStaticCell, StaticCell};

use juk_firmware::{app_core, main_core};

/// Main core executor
static MAIN_EXECUTOR: StaticCell<Executor> = StaticCell::new();
/// Application core stack (128K)
static APP_STACK: ConstStaticCell<Stack<131072>> = ConstStaticCell::new(Stack::new());
/// Application core executor
static APP_EXECUTOR: StaticCell<Executor> = StaticCell::new();

esp_bootloader_esp_idf::esp_app_desc!();

esp_hal::assign_resources! {
    Resources<'d> {
        com: ComResources<'d> {
            uart: UART0,
        },
        led: LedResources<'d> {
            r: GPIO17,
            g: GPIO18,
            b: GPIO8,
            ctrl: LEDC,
        },
        limits: LimitsResources<'d> {
            io: IO_MUX,
            x_p: GPIO4,
            x_m: GPIO5,
            y_p: GPIO6,
            y_m: GPIO7,
            z_p: GPIO15,
            z_m: GPIO16,
        },
        motor: MotorResources<'d> {
            en_drv: GPIO45,
            en_led: GPIO21,
            x_step: GPIO10,
            x_dir: GPIO9,
            y_step: GPIO12,
            y_dir: GPIO11,
            z_step: GPIO13,
            z_dir: GPIO14,
        },
    }
}

#[esp_hal::main]
fn main() -> ! {
    // we configure the clocks and etc. here to avoid doing it later
    // the default config is the best config
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    // see `lib.rs` for why this is such a pain
    let resources = split_resources!(peripherals);
    let resources = juk_firmware::Resources {
        com: juk_firmware::ComResources {
            uart: resources.com.uart,
        },
        led: juk_firmware::LedResources {
            r: resources.led.r,
            g: resources.led.g,
            b: resources.led.b,
            ctrl: resources.led.ctrl,
        },
        limits: juk_firmware::LimitsResources {
            io: resources.limits.io,
            x_p: resources.limits.x_p,
            x_m: resources.limits.x_m,
            y_p: resources.limits.y_p,
            y_m: resources.limits.y_m,
            z_p: resources.limits.z_p,
            z_m: resources.limits.z_m,
        },
        motor: juk_firmware::MotorResources {
            en_drv: resources.motor.en_drv,
            en_led: resources.motor.en_led,
            x_step: resources.motor.x_step,
            x_dir: resources.motor.x_dir,
            y_step: resources.motor.y_step,
            y_dir: resources.motor.y_dir,
            z_step: resources.motor.z_step,
            z_dir: resources.motor.z_dir,
        },
    };

    // turn on the heap allocator
    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    // start the RTOS timer
    let SystemTimer { alarm0, .. } = SystemTimer::new(peripherals.SYSTIMER);
    esp_rtos::start(alarm0);

    // start the application core (still don't know why we need software interrupts for this)
    let SoftwareInterruptControl {
        software_interrupt0,
        software_interrupt1,
        ..
    } = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start_second_core(
        peripherals.CPU_CTRL,
        software_interrupt0,
        software_interrupt1,
        APP_STACK.take(),
        || {
            let app_executor = APP_EXECUTOR.init(Executor::new());
            app_executor.run(|spawner| {
                // spawn the main task for the application core, which gets the spawner and can control its thread
                spawner.must_spawn(app_core::main(spawner, resources.limits, resources.motor));
            })
        },
    );

    let main_executor = MAIN_EXECUTOR.init(Executor::new());
    main_executor.run(|spawner| {
        // spawn the main task for the main thread
        spawner.must_spawn(main_core::main(spawner, resources.com, resources.led));
    })
}
