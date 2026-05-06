//! Communication interface controller
use alloc::format;

use esp_hal::uart::{Config, DataBits, Parity, StopBits, Uart};

use juk_cmd::parse_cmd;
use juk_com::{Input, Interface, InterfaceMode, Terminal};

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
    twrite!(strings::PROMPT);

    loop {
        match interface
            .get_input_async_out(&mut uart, || global::TERMINAL.receive(), strings::PROMPT)
            .await
        {
            Ok(input) => {
                match input {
                    Input::Binary(mut bytes) => {
                        match postcard::from_bytes_cobs::<juk_cmd::cmd::Command>(&mut bytes) {
                            Ok(cmd) => {
                                defmt::info!("Executing: {:?}", cmd);
                                // TODO: execute
                            }
                            Err(e) => {
                                defmt::error!("Error deserializing binary data: {}", e);
                                global::LED.signal((0xff, 0x00, 0x00));
                            }
                        }
                    }
                    Input::Text(text) => {
                        // acquire the configuration and parse the command
                        let res = {
                            let cfg = global::SYSCFG.lock().await;
                            parse_cmd(&text, &cfg)
                        };
                        // the rationale is that the command executor will acquire the configuration
                        // only when it needs to

                        match res {
                            Ok(cmd) => {
                                defmt::info!("Executing: {:?}", cmd);
                                // TODO: execute
                            }
                            Err(e) => {
                                defmt::warn!(
                                    "Error parsing `{}`: {}",
                                    text.as_str(),
                                    defmt::Display2Format(&e)
                                );

                                // print just the command name, fallback to the full command
                                let cmd = if let Some(s) = text.split_ascii_whitespace().next() {
                                    s
                                } else {
                                    &text
                                };
                                // format the error message
                                let msg = format!("{}{}: {}\r\n", strings::ERROR, cmd, e);
                                twrite!(msg);
                            }
                        }
                        // redraw the prompt
                        twrite!(strings::PROMPT);
                        // redraw the linebuffer
                        if let Err(e) = interface.redraw_line(&mut uart).await {
                            defmt::error!("UART error: {}", e);
                        }
                    }
                    Input::Help => {
                        // get the stuff on the command line
                        let mut tokens = interface.linebuffer().trim().split_ascii_whitespace();
                        match tokens.next() {
                            // if there is a known command display help for it
                            Some("move") => twrite!(strings::help::MOVE),
                            Some("arc") => twrite!(strings::help::ARC),
                            Some("home") => twrite!(strings::help::HOME),
                            Some("set") => twrite!(strings::help::SET),
                            Some("get") => twrite!(strings::help::GET),
                            // if not, display a list of commands
                            _ => twrite!(strings::help::LIST),
                        }
                        // redraw the prompt
                        twrite!(strings::PROMPT);
                        // redraw the linebuffer
                        if let Err(e) = interface.redraw_line(&mut uart).await {
                            defmt::error!("UART error: {}", e);
                        }
                    }
                    Input::Bell => {
                        defmt::info!("Running an easter egg, enjoy :)");
                        global::EGG.signal(());
                        // redraw the prompt
                        twrite!(strings::PROMPT);
                        // redraw the linebuffer
                        if let Err(e) = interface.redraw_line(&mut uart).await {
                            defmt::error!("UART error: {}", e);
                        }
                    }
                    Input::Cancel => {
                        defmt::warn!("Sending a cancel signal");
                        // send the signal
                        global::CANCEL.signal(());
                        // flush the queue
                        global::MOVEMENT.clear();
                        // redraw the prompt
                        twrite!(strings::PROMPT);
                        // redraw the linebuffer
                        if let Err(e) = interface.redraw_line(&mut uart).await {
                            defmt::error!("UART error: {}", e);
                        }
                    }
                    Input::EndOfText => {
                        // redraw the prompt
                        twrite!(strings::PROMPT);
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
                    Input::ModeChange(mode) => {
                        match mode {
                            InterfaceMode::Binary => {
                                // change the mode global variable
                                {
                                    let mut cfg = global::SYSCFG.lock().await;
                                    cfg.mode = InterfaceMode::Binary;
                                    cfg.led = 0xff00ff;
                                }
                                // signal the change by setting the LED to magenta
                                global::LED.signal((0xff, 0x00, 0xff))
                            }
                            // if entering text mode redraw stuff
                            InterfaceMode::Text => {
                                twrite!(strings::PROMPT);
                                if let Err(e) = interface.redraw_line(&mut uart).await {
                                    defmt::error!("UART error: {}", e);
                                }
                                // change the mode global variable
                                {
                                    let mut cfg = global::SYSCFG.lock().await;
                                    cfg.mode = InterfaceMode::Text;
                                    cfg.led = 0x00ff00;
                                }
                                // signal the change by setting the LED to green
                                global::LED.signal((0x00, 0xff, 0x00));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                defmt::error!("UART error: {}", e);
            }
        }
    }
}
