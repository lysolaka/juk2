//! The [`Interface`] struct implementation.

use alloc::vec::Vec;
use core::mem;

use crate::{
    Input,
    Terminal,
    eventparser::{Event, EventParser, Key},
    linebuffer::LineBuffer,
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
    line: LineBuffer,
    binary_buf: Vec<u8>,
}

impl Interface {
    /// Construct a new interface parser.
    pub fn new() -> Self {
        Self {
            mode: InterfaceMode::Text,
            parser: EventParser::new(),
            line: LineBuffer::new(),
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
            Event::Print(c) => {
                self.line.insert_char(c);
                let mut b = [0; 4];
                let s = c.encode_utf8(&mut b);
                terminal.write(s.as_bytes()).await?;
                self.redraw_from_cursor(terminal).await?;
                Ok(None)
            }
            Event::Execute(b) => match b {
                // CTRL + SPACE (NUL)
                0x00 => {
                    // the actual stuff is handled by the eventparser
                    terminal.write(b"^@").await?;
                    Ok(None)
                }
                // CTRL + C (ETX)
                0x03 => {
                    terminal.write(b"^C\r\n").await?;
                    self.line.clear();
                    Ok(Some(Input::EndOfText))
                }
                // CTRL + D (EOT)
                0x04 => {
                    if self.line.is_empty() {
                        terminal.write(b"^D\r\n").await?;
                        Ok(Some(Input::EndOfTransmission))
                    } else {
                        Ok(None)
                    }
                }
                // CTRL + G (BEL)
                0x07 => {
                    terminal.write(b"^G\r\n").await?;
                    Ok(Some(Input::Bell))
                }
                // CTRL + M (CR) [ENTER]
                0x0d => {
                    terminal.write(b"\r\n").await?;
                    let text = self.line.take();
                    self.line.clear();
                    Ok(Some(Input::Text(text)))
                }
                // CTRL + X (CAN)
                0x18 => {
                    terminal.write(b"^X\r\n").await?;
                    Ok(Some(Input::Cancel))
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
            Key::ArrowUp => (),   // TODO
            Key::ArrowDown => (), // TODO
            Key::ArrowRight => {
                if self.line.move_cursor_right() {
                    terminal.cursor_right().await?;
                }
            }
            Key::ArrowLeft => {
                if self.line.move_cursor_left() {
                    terminal.cursor_left().await?;
                }
            }
            Key::Home => {
                let count = self.line.move_cursor_to_start();
                for _ in 0..count {
                    terminal.cursor_left().await?;
                }
            }
            Key::End => {
                let count = self.line.move_cursor_to_end();
                for _ in 0..count {
                    terminal.cursor_right().await?;
                }
            }
            Key::Backspace => {
                if self.line.delete_before_cursor() {
                    terminal.cursor_left().await?;
                    self.redraw_from_cursor(terminal).await?;
                }
            }
            Key::Delete => {
                if self.line.delete_at_cursor() {
                    self.redraw_from_cursor(terminal).await?;
                }
            }
            Key::CtrlBackspace => {
                let count = self.line.delete_word_left();
                for _ in 0..count {
                    terminal.cursor_left().await?;
                }
                self.redraw_from_cursor(terminal).await?;
            }
            Key::CtrlDelete => {
                self.line.delete_word_right();
                self.redraw_from_cursor(terminal).await?;
            }
            Key::CtrlRight => {
                let count = self.line.move_cursor_word_right();
                for _ in 0..count {
                    terminal.cursor_right().await?;
                }
            }
            Key::CtrlLeft => {
                let count = self.line.move_cursor_word_left();
                for _ in 0..count {
                    terminal.cursor_left().await?;
                }
            }
        }
        Ok(())
    }

    /// Redraw the line content from the cursor to the end of the line.
    async fn redraw_from_cursor<T: Terminal>(&self, terminal: &mut T) -> Result<(), T::Error> {
        todo!()
    }

    /// Redraw the entire line content.
    ///
    /// Assumes that the cursor is at an empty prompt.
    pub async fn redraw_line<T: Terminal>(&self, terminal: &mut T) -> Result<(), T::Error> {
        todo!()
    }
}
