//! Public signals, channels and synchronization
use alloc::vec::Vec;

use juk_cmd::{cmd::Command, config::SystemConfig};

use critical_section as cs;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel,
    mutex::Mutex,
    signal::Signal,
};

bitflags::bitflags! {
    /// Limit switch status type
    pub struct LimitStatus: u8 {
        /// +X axis
        const PX = 0b000001;
        /// -X axis
        const NX = 0b000010;
        /// +Y axis
        const PY = 0b000100;
        /// -Y axis
        const NY = 0b001000;
        /// +Z axis
        const PZ = 0b010000;
        /// -Z axis
        const NZ = 0b100000;
    }
}

/// System configuration
pub static SYSCFG: Mutex<CriticalSectionRawMutex, SystemConfig> =
    Mutex::new(SystemConfig::const_default());

/// RGB LED color signal
pub static LED: Signal<CriticalSectionRawMutex, (u8, u8, u8)> = Signal::new();

/// Easter egg run signal
pub static EGG: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Terminal async output stream
pub static TERMINAL: Channel<CriticalSectionRawMutex, Vec<u8>, 4> = Channel::new();

/// Movement cancel signal
pub static CANCEL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Movement execution queue
pub static MOVEMENT: Channel<CriticalSectionRawMutex, Command, 8> = Channel::new();

/// Limit switch status
pub static LIMITS: cs::Mutex<LimitStatus> = cs::Mutex::new(LimitStatus::empty());
