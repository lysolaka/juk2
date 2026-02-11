//! The [`Interface`] struct implementation.

use alloc::vec::Vec;
use core::mem;

use crate::{
    Input,
    Terminal,
    eventparser::{Event, EventParser, Key},
};

/// The operating mode of [`Interface`].
///
/// Used to track state of the [`Interface`] state machine.
#[derive(Clone, Copy, PartialEq, Eq)]
enum InterfaceMode {
    Binary,
    Text,
}

/// The main REPL + binary interface struct.
///
/// This structure behaves like a state machine with two states:
/// - Text
/// - Binary
///
/// In the text state, the behaviour is identical to a REPL interface, but note that you have to
/// print the prompt (usually a `$ ` or `> `) yourself.
///
/// In the binary state, the interface reads COBS encoded messages, with the sentinel byte `0x00`.
///
/// Switching between these mode is as follows:
/// - Text -> Binary: press CTRL + SPACE twice (send `0x00` twice)
/// - Binary -> Text: send `0x00` twice in a row (more accurately: send `0x00` when the binary
/// input buffer is empty)
///
/// The output of this state machine is a variant of the [`Input`] enum denoting what has been
/// recieved on the interface allowing the user to take appropriate action.
///
/// To use this struct's functionality, a type implementing the [`Terminal`] trait is required.
pub struct Interface {
    mode: InterfaceMode,
    parser: EventParser,
    binary_buf: Vec<u8>,
}

impl Interface {
    /// Construct a new interface parser.
    pub fn new() -> Self {
        Self {
            mode: InterfaceMode::Text,
            parser: EventParser::new(),
            binary_buf: Vec::with_capacity(128),
        }
    }

    /// Wait for an input event.
    ///
    /// The parser does not do any work, when this function is not running. The function will return
    /// an error, if the [`Terminal`] instance runs into an error. All other actions performed by
    /// this function and this parser are infallible.
    pub async fn get_input<T: Terminal>(&mut self, terminal: &mut T) -> Result<Input, T::Error> {
        loop {
            let byte = terminal.read_byte().await?;
            if let Some(input) = match self.mode {
                InterfaceMode::Binary => self.binary_dispatch(byte, terminal).await?,
                InterfaceMode::Text => self.text_dispatch(byte, terminal).await?,
            } {
                return Ok(input);
            }
        }
    }

    /// Dispatch a byte in the binary state.
    #[inline]
    async fn binary_dispatch<T: Terminal>(
        &mut self,
        byte: u8,
        terminal: &mut T,
    ) -> Result<Option<Input>, T::Error> {
        if byte == 0x00 {
            if self.binary_buf.is_empty() {
                defmt::debug!("Binary mode got an empty frame, switching input mode to text");
                // TODO: make this message nicer
                terminal.write(b"\r\nSwitching to text mode.\r\n").await?;
                self.mode = InterfaceMode::Text;
                Ok(None)
            } else {
                self.binary_buf.push(byte);
                let bytes = mem::replace(&mut self.binary_buf, Vec::with_capacity(128));
                Ok(Some(Input::Binary(bytes)))
            }
        } else {
            self.binary_buf.push(byte);
            Ok(None)
        }
    }

    /// Dispatch a byte in the text state.
    #[inline]
    async fn text_dispatch<T: Terminal>(
        &mut self,
        byte: u8,
        terminal: &mut T,
    ) -> Result<Option<Input>, T::Error> {
        if let Some(event) = self.parser.advance(byte) {
            defmt::trace!("Text mode event: {:?}", event);
            let input = self.run_event(event, terminal).await?;

            if self.parser.terminated() {
                defmt::debug!("Text mode parser terminated, switching input mode to binary");
                // TODO: make this message nicer
                terminal
                    .write(
                        b"\r\nSwitching to binary mode.\r\nPress CTRL + Space twice to leave.\r\n",
                    )
                    .await?;
                self.parser.unterminate();
                self.mode = InterfaceMode::Binary;
            }

            Ok(input)
        } else {
            Ok(None)
        }
    }

    /// Perform the action associated with `event`.
    #[inline]
    async fn run_event<T: Terminal>(
        &mut self,
        event: Event,
        terminal: &mut T,
    ) -> Result<Option<Input>, T::Error> {
        match event {
            Event::Print(c) => todo!(),
            Event::Execute(b) => match b {
                // CTRL + SPACE (NUL)
                0x00 => {
                    todo!()
                }
                // CTRL + C (ETX)
                0x03 => {
                    todo!()
                }
                // CTRL + D (EOT)
                0x04 => {
                    todo!()
                }
                // CTRL + G (BEL)
                0x07 => {
                    todo!()
                }
                // CTRL + M (CR) [ENTER]
                0x0d => {
                    todo!()
                }
                // CTRL + X (CAN)
                0x18 => {
                    todo!()
                }
                _ => Ok(None),
            },
            Event::KeyEvent(key) => {
                self.run_key_event(key, terminal).await?;
                Ok(None)
            }
        }
    }

    /// Helper for [`Self::run_event()`] to avoid excessive indentation.
    #[inline]
    async fn run_key_event<T: Terminal>(
        &mut self,
        key: Key,
        terminal: &mut T,
    ) -> Result<(), T::Error> {
        match key {
            Key::ArrowUp => todo!(),
            Key::ArrowDown => todo!(),
            Key::ArrowRight => todo!(),
            Key::ArrowLeft => todo!(),
            Key::Home => todo!(),
            Key::End => todo!(),
            Key::Backspace => todo!(),
            Key::Delete => todo!(),
            Key::CtrlBackspace => todo!(),
            Key::CtrlDelete => todo!(),
            Key::CtrlRight => todo!(),
            Key::CtrlLeft => todo!(),
        }
    }
}
