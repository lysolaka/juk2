use embassy_executor::Spawner;

use esp_hal::uart::{Config, DataBits, Parity, StopBits, Uart};
use juk_cmd::config::SystemConfig;
use juk_com::{Input, Interface, Terminal};

use crate::{ComResources, LedResources, strings};

#[embassy_executor::task]
pub async fn main(spawner: Spawner, com: ComResources<'static>, led: LedResources<'static>) -> ! {
    // TODO: spawn some tasks
    let _ = spawner;

    // discard warnings
    let _ = led;

    let uart_config = Config::default()
        .with_baudrate(115200)
        .with_data_bits(DataBits::_8)
        .with_stop_bits(StopBits::_1)
        .with_parity(Parity::None);

    let mut uart = defmt::expect!(
        Uart::new(com.uart, uart_config),
        "Failed to initialize the UART interface"
    )
    .into_async();

    let mut interface = Interface::new();

    twrite(&mut uart, "\r\n").await;
    twrite(&mut uart, strings::LICENSE).await;
    twrite(&mut uart, "\r\n").await;
    twrite(&mut uart, strings::VERSION).await;
    twrite(&mut uart, "\r\n").await;
    twrite(&mut uart, strings::WELCOME).await;
    twrite(&mut uart, "$ ").await;

    let cfg = SystemConfig::default();

    loop {
        match interface
            .get_input_async_out(&mut uart, async || crate::TEST_CH.receive().await, "$ ")
            .await
        {
            Ok(input) => match input {
                Input::Binary(items) => defmt::info!("Binary input: {=[u8]}", &items[..]),
                Input::Text(text) => {
                    defmt::info!("Text input: {}", text.as_str());
                    match juk_cmd::parse_cmd(&text, &cfg) {
                        Ok(c) => defmt::info!("Got command: {:?}", c),
                        Err(e) => {
                            defmt::error!("Error: {}", defmt::Display2Format(&e));
                        }
                    }
                    twrite(&mut uart, "$ ").await;
                }
                Input::EndOfTransmission => {
                    defmt::info!("CTRL + D: resetting...");
                    esp_hal::system::software_reset();
                }
                _ => {
                    twrite(&mut uart, "$ ").await;
                    defmt::unwrap!(interface.redraw_line(&mut uart).await);
                }
            },
            Err(e) => {
                defmt::error!("UART error: {}", e);
                defmt::panic!();
            }
        }
    }
}

async fn twrite<T: Terminal>(term: &mut T, text: &str) {
    defmt::expect!(term.write(text.as_bytes()).await, "UART write failed");
}
