#![no_std]

extern crate alloc;

mod eventparser;
pub mod terminal;
mod interface;

use alloc::vec::Vec;
use alloc::string::String;

pub enum Input {
    Binary(Vec<u8>),
    Text(String),
    Bell,
    Cancel,
    EndOfText,
    EndOfTransmission,
}

pub use interface::Interface;
