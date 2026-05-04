//! Communication interface controller
use esp_hal::uart::{Config, DataBits, Parity, StopBits, Uart};
use juk_com::{Input, Interface, Terminal};

use crate::{ComResources, global, strings};

#[embassy_executor::task]
pub async fn com_control(com: ComResources<'static>) -> ! {
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

    // helper macro for printing
    macro_rules! twrite {
        ($text:expr) => {{
            defmt::expect!(
                <Uart<'_, esp_hal::Async> as Terminal>::write(&mut uart, $text.as_bytes()).await,
                "UART write failed"
            );
        }};
    }

    let mut interface = Interface::new();

    twrite!("\r\n");
    twrite!(strings::LICENSE);
    twrite!("\r\n");
    twrite!(strings::VERSION);
    twrite!("\r\n");
    twrite!(strings::WELCOME);
    twrite!("$ ");

    loop {
        match interface
            .get_input_async_out(&mut uart, || global::TERMINAL.receive(), "$ ")
            .await
        {
            Ok(input) => {
                match input {
                    Input::Binary(items) => defmt::todo!(),
                    Input::Text(text) => defmt::todo!(),
                    Input::Help => defmt::todo!(),
                    Input::Bell => {
                        defmt::info!("Running an easter egg, enjoy :)");
                        global::EGG.signal(());
                        // redraw the prompt
                        twrite!("$ ");
                        // redraw the linebuffer
                        if let Err(e) = interface.redraw_line(&mut uart).await {
                            defmt::error!("UART error: {}", e);
                        }
                    }
                    Input::Cancel => {
                        defmt::warn!("Sending a cancel signal");
                        // redraw the prompt
                        twrite!("$ ");
                        // redraw the linebuffer
                        if let Err(e) = interface.redraw_line(&mut uart).await {
                            defmt::error!("UART error: {}", e);
                        }
                        // TODO: send it
                    }
                    Input::EndOfText => {
                        // redraw the prompt
                        twrite!("$ ");
                        // redraw the linebuffer
                        if let Err(e) = interface.redraw_line(&mut uart).await {
                            defmt::error!("UART error: {}", e);
                        }
                    }
                    Input::EndOfTransmission => {
                        // do a software reset
                        defmt::warn!("CTRL + D was pressed, rebooting...");
                        esp_hal::system::software_reset();
                    }
                }
            }
            Err(e) => {
                defmt::error!("UART error: {}", e);
            }
        }
    }
}
