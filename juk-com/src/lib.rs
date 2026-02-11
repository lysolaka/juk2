#![no_std]

extern crate alloc;

mod eventparser;
mod interface;
mod terminal;

use alloc::{string::String, vec::Vec};

pub enum Input {
    Binary(Vec<u8>),
    Text(String),
    Bell,
    Cancel,
    EndOfText,
    EndOfTransmission,
}

pub use interface::Interface;
pub use terminal::Terminal;
