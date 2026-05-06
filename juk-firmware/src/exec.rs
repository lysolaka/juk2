//! Command execution logic
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use juk_cmd::{
    cmd::{Command, Response},
    config::{Frame, Unit},
    defaults,
};

use crate::{global, global::LimitStatus, strings};

/// Run the given [`Command`] from text mode.
pub async fn run_text(cmd: Command) -> Option<String> {
    defmt::info!("Running text command: {:?}", cmd);
    match cmd {
        // text mode has the CTRL + X hook to send a cancel signal
        Command::Cancel => defmt::unreachable!(),
        Command::ConfigSet { kv } => set_config(kv).await,
        Command::ConfigGet { key } => Some(get_config(key).await),
        // movements go here
        cmd => defmt::todo!(),
    }
}

#[inline]
async fn set_config(kv: Vec<(String, String)>) -> Option<String> {
    for (k, v) in kv {
        match k.as_str() {
            "accel" => {
                let a = match v.parse() {
                    Ok(a) if defaults::MOTION_ARG_RANGE.contains(&a) => a,
                    Ok(a) => {
                        defmt::error!("`set accel={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set accel={}: value out of range\r\n",
                            strings::ERROR,
                            a
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set accel={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set accel={}: expected a floating-point number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };
                defmt::info!("Setting `accel` to {}", a);
                let mut cfg = global::SYSCFG.lock().await;
                cfg.accel = a;
            }
            "frame" => {
                let frame = match v.as_str() {
                    "rel" => Frame::Relative,
                    "abs" => Frame::Absolute,
                    _ => {
                        defmt::error!(
                            "`set frame={}` failed: expected \"rel\" or \"abs\"",
                            v.as_str()
                        );
                        return Some(format!(
                            "{}set frame={}: expected \"rel\" or \"abs\"\r\n",
                            strings::ERROR,
                            v.as_str()
                        ));
                    }
                };
                defmt::info!("Setting `frame` to {}", frame);
                let mut cfg = global::SYSCFG.lock().await;
                cfg.frame = frame;
            }
            "led" => {
                let rgb = match u32::from_str_radix(v.as_str(), 16) {
                    Ok(rgb) if defaults::LED_RANGE.contains(&rgb) => rgb,
                    Ok(rgb) => {
                        defmt::error!("`set led={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set led={:06x}: value out of range\r\n",
                            strings::ERROR,
                            rgb
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set led={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set led={}: expected an integer number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };

                let r = ((rgb >> 16) & 0xff) as u8;
                let g = ((rgb >> 8) & 0xff) as u8;
                let b = (rgb & 0xff) as u8;

                defmt::info!("Setting `led` to {:08x}", rgb);
                // set the color
                global::LED.signal((r, g, b));
                let mut cfg = global::SYSCFG.lock().await;
                cfg.led = rgb;
            }
            "mmpsX" => {
                let val = match v.parse() {
                    Ok(val) if defaults::MMPS_RANGE.contains(&val) => val,
                    Ok(val) => {
                        defmt::error!("`set mmpsX={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set mmpsX={}: value out of range\r\n",
                            strings::ERROR,
                            val
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set mmpsX={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set mmpsX={}: expected a floating-point number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };
                defmt::info!("Setting `mmpsX` to {}", val);
                let mut cfg = global::SYSCFG.lock().await;
                cfg.mmps.0 = val;
            }
            "mmpsY" => {
                let val = match v.parse() {
                    Ok(val) if defaults::MMPS_RANGE.contains(&val) => val,
                    Ok(val) => {
                        defmt::error!("`set mmpsY={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set mmpsY={}: value out of range\r\n",
                            strings::ERROR,
                            val
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set mmpsY={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set mmpsY={}: expected a floating-point number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };
                defmt::info!("Setting `mmpsY` to {}", val);
                let mut cfg = global::SYSCFG.lock().await;
                cfg.mmps.1 = val;
            }
            "mmpsZ" => {
                let val = match v.parse() {
                    Ok(val) if defaults::MMPS_RANGE.contains(&val) => val,
                    Ok(val) => {
                        defmt::error!("`set mmpsZ={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set mmpsZ={}: value out of range\r\n",
                            strings::ERROR,
                            val
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set mmpsZ={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set mmpsZ={}: expected a floating-point number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };
                defmt::info!("Setting `mmpsZ` to {}", val);
                let mut cfg = global::SYSCFG.lock().await;
                cfg.mmps.2 = val;
            }
            "posX" => {
                let val = match i32::from_str_radix(v.as_str(), 10) {
                    Ok(val) if defaults::POS_RANGE.contains(&val) => val,
                    Ok(val) => {
                        defmt::error!("`set posX={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set posX={}: value out of range\r\n",
                            strings::ERROR,
                            val
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set posX={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set posX={}: expected an integer number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };
                defmt::info!("Setting `posX` to {}", val);
                critical_section::with(|cs| {
                    let mut pos = global::POS.borrow_ref_mut(cs);
                    pos.0 = val;
                });
            }
            "posY" => {
                let val = match i32::from_str_radix(v.as_str(), 10) {
                    Ok(val) if defaults::POS_RANGE.contains(&val) => val,
                    Ok(val) => {
                        defmt::error!("`set posY={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set posY={}: value out of range\r\n",
                            strings::ERROR,
                            val
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set posY={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set posY={}: expected an integer number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };
                defmt::info!("Setting `posY` to {}", val);
                critical_section::with(|cs| {
                    let mut pos = global::POS.borrow_ref_mut(cs);
                    pos.1 = val;
                });
            }
            "posZ" => {
                let val = match i32::from_str_radix(v.as_str(), 10) {
                    Ok(val) if defaults::POS_RANGE.contains(&val) => val,
                    Ok(val) => {
                        defmt::error!("`set posZ={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set posZ={}: value out of range\r\n",
                            strings::ERROR,
                            val
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set posZ={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set posZ={}: expected an integer number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };
                defmt::info!("Setting `posZ` to {}", val);
                critical_section::with(|cs| {
                    let mut pos = global::POS.borrow_ref_mut(cs);
                    pos.2 = val;
                });
            }
            "unit" => {
                let unit = match v.as_str() {
                    "steps" => Unit::Steps,
                    "mm" => Unit::Millimeters,
                    _ => {
                        defmt::error!(
                            "`set unit={}` failed: expected \"steps\" or \"mm\"",
                            v.as_str()
                        );
                        return Some(format!(
                            "{}set unit={}: expected \"steps\" or \"mm\"\r\n",
                            strings::ERROR,
                            v.as_str()
                        ));
                    }
                };
                defmt::info!("Setting `unit` to {}", unit);
                let mut cfg = global::SYSCFG.lock().await;
                cfg.unit = unit;
            }
            "vel" => {
                let vel = match v.parse() {
                    Ok(vel) if defaults::MOTION_ARG_RANGE.contains(&vel) => vel,
                    Ok(vel) => {
                        defmt::error!("`set vel={}` failed: value out of range", v.as_str());
                        return Some(format!(
                            "{}set vel={}: value out of range\r\n",
                            strings::ERROR,
                            vel
                        ));
                    }
                    Err(e) => {
                        defmt::error!("`set vel={}` failed: string parsing error", v.as_str());
                        return Some(format!(
                            "{}set vel={}: expected a floating-point number, {}\r\n",
                            strings::ERROR,
                            v.as_str(),
                            e
                        ));
                    }
                };
                defmt::info!("Setting `vel` to {}", vel);
                let mut cfg = global::SYSCFG.lock().await;
                cfg.vel = vel;
            }
            // NOTE: right now it is unreachable
            // TODO: edit juk-cmd to allow for any key, since this error message is better
            _ => {
                return Some(format!(
                    "{}set: unknown key `{}`\r\n",
                    strings::ERROR,
                    k.as_str()
                ));
            }
        }
    }

    None
}

#[inline]
async fn get_config(key: String) -> String {
    match key.as_str() {
        "accel" => {
            let accel = {
                let c = global::SYSCFG.lock().await;
                c.accel
            };
            format!("{}  `accel` = {}\r\n", strings::INFO, accel)
        }
        "frame" => {
            let frame = {
                let c = global::SYSCFG.lock().await;
                c.frame
            };
            match frame {
                Frame::Absolute => format!("{}  `frame` = abs\r\n", strings::INFO),
                Frame::Relative => format!("{}  `frame` = rel\r\n", strings::INFO),
            }
        }
        "led" => {
            let led = {
                let c = global::SYSCFG.lock().await;
                c.led
            };
            format!("{}  `led` = {:06x}\r\n", strings::INFO, led)
        }
        "license" => strings::LICENSE.to_string(),
        "limits" => {
            let lim = critical_section::with(|cs| *global::LIMITS.borrow_ref(cs));
            format!(
                "{0}  `limits`\r\n{0}   X+: {1}, X-: {2}\r\n{0}   Y+: {3}, Y-: {4}\r\n{0}   Z+: {5}, Z-: {6}\r\n",
                strings::INFO,
                lim.contains(LimitStatus::PX),
                lim.contains(LimitStatus::NX),
                lim.contains(LimitStatus::PY),
                lim.contains(LimitStatus::NY),
                lim.contains(LimitStatus::PZ),
                lim.contains(LimitStatus::NZ),
            )
        }
        "mmps" => {
            let mmps = {
                let c = global::SYSCFG.lock().await;
                c.mmps
            };
            format!(
                "{0}  `mmps`\r\n{0}   X = {1}\r\n{0}   Y = {2}\r\n{0}   Z = {3}\r\n",
                strings::INFO,
                mmps.0,
                mmps.1,
                mmps.2
            )
        }
        "mmpsX" => {
            let val = {
                let c = global::SYSCFG.lock().await;
                c.mmps.0
            };
            format!("{}  `mmpsX` = {}\r\n", strings::INFO, val)
        }
        "mmpsY" => {
            let val = {
                let c = global::SYSCFG.lock().await;
                c.mmps.0
            };
            format!("{}  `mmpsY` = {}\r\n", strings::INFO, val)
        }
        "mmpsZ" => {
            let val = {
                let c = global::SYSCFG.lock().await;
                c.mmps.0
            };
            format!("{}  `mmpsZ` = {}\r\n", strings::INFO, val)
        }
        "pos" => {
            let pos = critical_section::with(|cs| *global::POS.borrow_ref(cs));
            format!(
                "{0}  `pos`\r\n{0}   X = {1}\r\n{0}   Y = {2}\r\n{0}   Z = {3}\r\n",
                strings::INFO,
                pos.0,
                pos.1,
                pos.2
            )
        }
        "posX" => {
            let pos = critical_section::with(|cs| *global::POS.borrow_ref(cs));
            format!("{}  `posX` = {}\r\n", strings::INFO, pos.0)
        }
        "posY" => {
            let pos = critical_section::with(|cs| *global::POS.borrow_ref(cs));
            format!("{}  `posY` = {}\r\n", strings::INFO, pos.1)
        }
        "posZ" => {
            let pos = critical_section::with(|cs| *global::POS.borrow_ref(cs));
            format!("{}  `posZ` = {}\r\n", strings::INFO, pos.2)
        }
        "status" => {
            let mmps = {
                let c = global::SYSCFG.lock().await;
                c.mmps
            };
            let mmps = format!(
                "{0}  `mmps`\r\n{0}   X = {1}\r\n{0}   Y = {2}\r\n{0}   Z = {3}\r\n",
                strings::INFO,
                mmps.0,
                mmps.1,
                mmps.2
            );

            let pos = critical_section::with(|cs| *global::POS.borrow_ref(cs));
            let pos = format!(
                "{0}  `pos`\r\n{0}   X = {1}\r\n{0}   Y = {2}\r\n{0}   Z = {3}\r\n",
                strings::INFO,
                pos.0,
                pos.1,
                pos.2
            );

            let lim = critical_section::with(|cs| *global::LIMITS.borrow_ref(cs));
            let lim = format!(
                "{0}  `limits`\r\n{0}   X+: {1}, X-: {2}\r\n{0}   Y+: {3}, Y-: {4}\r\n{0}   Z+: {5}, Z-: {6}\r\n",
                strings::INFO,
                lim.contains(LimitStatus::PX),
                lim.contains(LimitStatus::NX),
                lim.contains(LimitStatus::PY),
                lim.contains(LimitStatus::NY),
                lim.contains(LimitStatus::PZ),
                lim.contains(LimitStatus::NZ),
            );

            format!("{}{}{}", mmps, pos, lim)
        }
        "unit" => {
            let unit = {
                let c = global::SYSCFG.lock().await;
                c.unit
            };
            match unit {
                Unit::Steps => format!("{}  `unit` = steps\r\n", strings::INFO),
                Unit::Millimeters => format!("{}  `unit` = mm\r\n", strings::INFO),
            }
        }
        "vel" => {
            let vel = {
                let c = global::SYSCFG.lock().await;
                c.vel
            };
            format!("{}  `vel` = {}\r\n", strings::INFO, vel)
        }
        "version" => strings::VERSION.to_string(),
        // NOTE: right now it is unreachable
        // TODO: edit juk-cmd to allow for any key, since this error message is better
        _ => {
            format!("{}get: unknown key `{}`\r\n", strings::ERROR, key.as_str())
        }
    }
}

/// Run the given [`Command`] from binary mode.
pub async fn run_binary(cmd: Command) -> Option<Response> {
    defmt::info!("Running binary command: {:?}", cmd);
    match cmd {
        Command::Cancel => {
            defmt::warn!("Sending a cancel signal and flushing the queue");
            global::CANCEL.signal(());
            global::MOVEMENT.clear();
            Some(Response::Ok)
        }
        Command::ConfigSet { kv: _ } => {
            defmt::error!("Configuration is not supported in binary mode");
            None
        }
        Command::ConfigGet { key: _ } => {
            let cfg = global::SYSCFG.lock().await;
            defmt::info!("Configuration acquired, sending back");
            Some(Response::Config {
                accel: cfg.accel,
                vel: cfg.vel,
                frame: cfg.frame,
                unit: cfg.unit,
                mmps: cfg.mmps,
            })
        }
        // movements go here
        cmd => defmt::todo!(),
    }
}
