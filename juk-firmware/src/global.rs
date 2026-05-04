//! Public signals, channels and synchronization
use alloc::string::String;

use juk_cmd::config::SystemConfig;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel,
    mutex::Mutex,
    signal::Signal,
};

/// System configuration
pub static SYSCFG: Mutex<CriticalSectionRawMutex, SystemConfig> =
    Mutex::new(SystemConfig::const_default());

/// RGB LED color signal
pub static LED: Signal<CriticalSectionRawMutex, (u8, u8, u8)> = Signal::new();

/// Easter egg run signal
pub static EGG: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Terminal async output stream
pub static TERMINAL: Channel<CriticalSectionRawMutex, String, 4> = Channel::new();
