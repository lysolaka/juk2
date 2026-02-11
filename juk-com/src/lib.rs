//! Communication interface for use with the `juk-firmware`.
//!
//! Implements the [`Interface`] struct meant to handle control input and the [`Terminal`] trait
//! used to perform actions required by [`Interface`].

#![no_std]

extern crate alloc;

mod eventparser;
mod interface;
mod terminal;

use alloc::{string::String, vec::Vec};

/// An enum representing input events fired by [`Interface`].
pub enum Input {
    /// Binary data was recieved.
    ///
    /// The sentinel NUL byte is also included in the payload.
    Binary(Vec<u8>),
    /// Text data was recieved.
    ///
    /// The payload is a stripped string.
    Text(String),
    /// CTRL + G was pressed.
    Bell,
    /// CTRL + X was pressed.
    Cancel,
    /// CTRL + C was pressed, the input buffer was cleared, redraw the prompt.
    EndOfText,
    /// CTRL + D was pressed.
    EndOfTransmission,
}

pub use interface::Interface;
pub use terminal::Terminal;
