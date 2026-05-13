//! Text input to commands parser
use alloc::{string::ToString, vec::Vec};

use crate::{
    ParseError,
    cmd::{ArcDir, Axis, Command, Displacement},
    config::{SystemConfig, Unit},
    defaults,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arg<'a> {
    KeyValue(&'a str, &'a str),
    // used for the get command only
    Key(&'a str),
}

/// Attempt to parse a command from the input.
pub fn parse_cmd(input: &str, cfg: &SystemConfig) -> Result<Command, ParseError> {
    let (cmd, args) = split_args(input)?;

    match cmd {
        "move" => parse_move(args, cfg),
        "arc" => parse_arc(args, cfg),
        "home" => parse_home(args),
        "set" => parse_set(args),
        "get" => parse_get(args),
        _ => Err(ParseError::UnknownCommand),
    }
}

/// Splits the command from its arguments. Returns an error if the input is empty.
fn split_args(input: &str) -> Result<(&str, impl Iterator<Item = Arg<'_>>), ParseError> {
    let mut parts = input.split_ascii_whitespace();

    let cmd = parts.next().ok_or(ParseError::EmptyInput)?;

    let iter = parts.map(|token| {
        if let Some((k, v)) = token.split_once('=') {
            Arg::KeyValue(k, v)
        } else {
            Arg::Key(token)
        }
    });

    Ok((cmd, iter))
}

/// Parse the move command.
fn parse_move<'a>(
    args: impl Iterator<Item = Arg<'a>>,
    cfg: &SystemConfig,
) -> Result<Command, ParseError> {
    let mut out_x = None;
    let mut out_y = None;
    let mut out_z = None;
    let mut out_a = cfg.accel;
    let mut out_v = cfg.vel;

    for arg in args {
        match arg {
            Arg::KeyValue("x", val) => match cfg.unit {
                Unit::Steps => {
                    let x: i32 = val.parse()?;
                    out_x = Some(Displacement::from_steps(x, cfg)?)
                }
                Unit::Millimeters => {
                    let x: f32 = val.parse()?;
                    out_x = Some(Displacement::from_mm(x, Axis::X, cfg)?)
                }
            },
            Arg::KeyValue("y", val) => match cfg.unit {
                Unit::Steps => {
                    let y: i32 = val.parse()?;
                    out_y = Some(Displacement::from_steps(y, cfg)?)
                }
                Unit::Millimeters => {
                    let y: f32 = val.parse()?;
                    out_y = Some(Displacement::from_mm(y, Axis::Y, cfg)?)
                }
            },
            Arg::KeyValue("z", val) => match cfg.unit {
                Unit::Steps => {
                    let z: i32 = val.parse()?;
                    out_z = Some(Displacement::from_steps(z, cfg)?)
                }
                Unit::Millimeters => {
                    let z: f32 = val.parse()?;
                    out_z = Some(Displacement::from_mm(z, Axis::Z, cfg)?)
                }
            },
            Arg::KeyValue("a", val) => {
                let a: f32 = val.parse()?;
                // convert to steps, use just the X axis, I can't be bothered
                let a = if cfg.unit == Unit::Millimeters {
                    a / cfg.mmps.0
                } else {
                    a
                };

                if !defaults::ACCEL_ARG_RANGE.contains(&a) {
                    return Err(ParseError::MotionRange);
                } else {
                    out_a = a;
                }
            }
            Arg::KeyValue("v", val) => {
                let v: f32 = val.parse()?;
                // convert to steps, use just the X axis, I can't be bothered
                let v = if cfg.unit == Unit::Millimeters {
                    v / cfg.mmps.0
                } else {
                    v
                };

                if !defaults::VEL_ARG_RANGE.contains(&v) {
                    return Err(ParseError::MotionRange);
                } else {
                    out_v = v;
                }
            }
            _ => return Err(ParseError::InvalidArgument),
        }
    }

    if out_x.is_none() && out_y.is_none() && out_z.is_none() {
        return Err(ParseError::ArgumentRelation);
    }

    Ok(Command::Move {
        x: out_x.unwrap_or(Displacement::Relative(0)),
        y: out_y.unwrap_or(Displacement::Relative(0)),
        z: out_z.unwrap_or(Displacement::Relative(0)),
        a: out_a,
        v: out_v,
    })
}

/// Parse the arc command.
fn parse_arc<'a>(
    args: impl Iterator<Item = Arg<'a>>,
    cfg: &SystemConfig,
) -> Result<Command, ParseError> {
    let mut out_x = None;
    let mut out_y = None;
    let mut out_z = Displacement::Relative(0);
    let mut out_r = None;
    let mut out_dir = ArcDir::Pos;
    let mut out_a = cfg.accel;
    let mut out_v = cfg.vel;

    for arg in args {
        match arg {
            Arg::KeyValue("x", val) => match cfg.unit {
                Unit::Steps => {
                    let x: i32 = val.parse()?;
                    out_x = Some(Displacement::from_steps(x, cfg)?)
                }
                Unit::Millimeters => {
                    let x: f32 = val.parse()?;
                    out_x = Some(Displacement::from_mm(x, Axis::X, cfg)?)
                }
            },
            Arg::KeyValue("y", val) => match cfg.unit {
                Unit::Steps => {
                    let y: i32 = val.parse()?;
                    out_y = Some(Displacement::from_steps(y, cfg)?)
                }
                Unit::Millimeters => {
                    let y: f32 = val.parse()?;
                    out_y = Some(Displacement::from_mm(y, Axis::Y, cfg)?)
                }
            },
            Arg::KeyValue("z", val) => match cfg.unit {
                Unit::Steps => {
                    let z: i32 = val.parse()?;
                    out_z = Displacement::from_steps(z, cfg)?
                }
                Unit::Millimeters => {
                    let z: f32 = val.parse()?;
                    out_z = Displacement::from_mm(z, Axis::Z, cfg)?
                }
            },
            Arg::KeyValue("r", val) => match cfg.unit {
                Unit::Steps => {
                    let r: u32 = val.parse()?;
                    // there's the defaults module for checks, but fuck me
                    if r < 50_000 {
                        out_r = Some(r);
                    } else {
                        return Err(ParseError::DisplacementRange);
                    }
                }
                Unit::Millimeters => {
                    let r: f32 = val.parse()?;
                    // check if it's positive
                    if r < 0.0 {
                        return Err(ParseError::DisplacementRange);
                    } else {
                        // convert to steps using the mmpsX variable
                        // due to this the arcs can be eggs, but whatever
                        let r = libm::roundf(r / cfg.mmps.0) as u32;
                        out_r = Some(r);
                    }
                }
            },
            Arg::KeyValue("dir", val) => match val {
                "pos" => out_dir = ArcDir::Pos,
                "neg" => out_dir = ArcDir::Neg,
                _ => return Err(ParseError::InvalidArgument),
            },
            Arg::KeyValue("a", val) => {
                let a: f32 = val.parse()?;
                // convert to steps, use just the X axis, I can't be bothered
                let a = if cfg.unit == Unit::Millimeters {
                    a / cfg.mmps.0
                } else {
                    a
                };

                if !defaults::ACCEL_ARG_RANGE.contains(&a) {
                    return Err(ParseError::MotionRange);
                } else {
                    out_a = a;
                }
            }
            Arg::KeyValue("v", val) => {
                let v: f32 = val.parse()?;
                // convert to steps, use just the X axis, I can't be bothered
                let v = if cfg.unit == Unit::Millimeters {
                    v / cfg.mmps.0
                } else {
                    v
                };

                if !defaults::VEL_ARG_RANGE.contains(&v) {
                    return Err(ParseError::MotionRange);
                } else {
                    out_v = v;
                }
            }
            _ => return Err(ParseError::InvalidArgument),
        }
    }

    if out_x.is_none() || out_y.is_none() || out_r.is_none() {
        return Err(ParseError::ArgumentRelation);
    }

    // unwrap is ok here since we've just checked
    Ok(Command::Arc {
        x: out_x.unwrap(),
        y: out_y.unwrap(),
        z: out_z,
        r: out_r.unwrap(),
        dir: out_dir,
        a: out_a,
        v: out_v,
    })
}

/// Parse the home command.
fn parse_home<'a>(args: impl Iterator<Item = Arg<'a>>) -> Result<Command, ParseError> {
    let mut out = (true, true, true);

    for arg in args {
        match arg {
            Arg::KeyValue("axis", val) if val.chars().all(|c| c == 'x' || c == 'y' || c == 'z') => {
                out = (false, false, false);
                // let's cut corners and check using `contains()`
                if val.contains('x') {
                    out.0 = true;
                }
                if val.contains('y') {
                    out.1 = true;
                }
                if val.contains('z') {
                    out.2 = true;
                }
            }
            _ => return Err(ParseError::InvalidArgument),
        }
    }

    Ok(Command::Home {
        x: out.0,
        y: out.1,
        z: out.2,
    })
}

/// Parse the set command.
fn parse_set<'a>(args: impl Iterator<Item = Arg<'a>>) -> Result<Command, ParseError> {
    let mut out = Vec::with_capacity(3);

    for arg in args {
        match arg {
            Arg::KeyValue(k, val) => out.push((k.to_string(), val.to_string())),
            _ => return Err(ParseError::InvalidArgument),
        }
    }

    if out.is_empty() {
        return Err(ParseError::ArgumentRelation);
    }

    Ok(Command::ConfigSet { kv: out })
}

/// Parse the get command.
fn parse_get<'a>(args: impl Iterator<Item = Arg<'a>>) -> Result<Command, ParseError> {
    let mut out = None;

    for arg in args {
        match arg {
            Arg::Key(k) => {
                if out.is_some() {
                    return Err(ParseError::ArgumentRelation);
                }

                out = Some(k.to_string());
            }
            _ => return Err(ParseError::InvalidArgument),
        }
    }

    if out.is_none() {
        return Err(ParseError::ArgumentRelation);
    }

    // we've just checked that it's `Some`
    Ok(Command::ConfigGet { key: out.unwrap() })
}
