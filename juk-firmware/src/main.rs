#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

extern crate alloc;

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{
    timer::timg::TimerGroup,
    uart::{Config, DataBits, Parity, StopBits, Uart},
};
use esp_println as _;
use juk_com::{Input, Interface, Terminal};
use juk_firmware::strings;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    // TODO: Spawn some tasks
    let _ = spawner;

    let uart_config = Config::default()
        .with_baudrate(115200)
        .with_data_bits(DataBits::_8)
        .with_stop_bits(StopBits::_1)
        .with_parity(Parity::None);

    let mut uart = defmt::expect!(
        Uart::new(peripherals.UART0, uart_config),
        "Failed to initialize the UART interface"
    )
    .into_async();

    let mut interface = Interface::new();

    defmt::expect!(strings::print_verinfo(&mut uart).await, "UART write failed");
    uwrite(&mut uart, strings::WELCOME_MOTD).await;
    uwrite(&mut uart, "$ ").await;

    loop {
        match interface.get_input(&mut uart).await {
            Ok(input) => match input {
                Input::Binary(items) => defmt::info!("Binary input: {=[u8]}", &items[..]),
                Input::Text(text) => {
                    defmt::info!("Text input: {}", text.as_str());
                    uwrite(&mut uart, "$ ").await;
                }
                Input::EndOfTransmission => {
                    defmt::info!("CTRL + D: resetting...");
                    esp_hal::system::software_reset();
                }
                _ => {
                    uwrite(&mut uart, "$ ").await;
                    defmt::expect!(interface.redraw_line(&mut uart).await, "UART write failed");
                }
            },
            Err(e) => {
                defmt::error!("UART Error: {}", e);
                defmt::panic!();
            }
        }
    }
}

/// Quick wrapper for UART writes using the [`Terminal`] trait.
///
/// NOTE: for testing purposes only.
#[inline]
async fn uwrite<T: Terminal>(term: &mut T, text: &str) {
    defmt::expect!(term.write(text.as_bytes()).await, "UART write failed");
}
