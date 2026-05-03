//! Public signals, channels and synchronization
use juk_cmd::config::SystemConfig;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, signal::Signal};

pub static SYSCFG: Mutex<CriticalSectionRawMutex, SystemConfig> =
    Mutex::new(SystemConfig::const_default());

pub static LED: Signal<CriticalSectionRawMutex, (u8, u8, u8)> = Signal::new();
