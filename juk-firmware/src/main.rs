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
    let mut buf = [0; 128];

    loop {
        match uart.read_async(&mut buf).await {
            Ok(0) => defmt::panic!("UART read returned 0"),
            Ok(read) => {
                defmt::info!("UART in: {=[u8]:02x}", &buf[..read]);
                let mut count = 0;
                while count != read {
                    match uart.write_async(&buf[count..read]).await {
                        Ok(written) => count += written,
                        Err(e) => {
                            defmt::error!("UART error: {}", e);
                            defmt::panic!("Bye, bye!");
                        }
                    }
                }
            }
            Err(e) => {
                defmt::error!("UART error: {}", e);
                defmt::panic!("Bye, bye!");
            }
        }
    }
}
