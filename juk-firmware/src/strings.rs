use juk_com::Terminal;
use const_format::formatc;

shadow_rs::shadow!(build);

const INFO: &str = "\x1b[1;32m*\x1b[0m";
const WARN: &str = "\x1b[1;33m*\x1b[0m";

const LICENSE_1: &str = "juk-firmware Copyright (C) 2026 lysolaka";
const LICENSE_2: &str = "License GNU GPL-3.0 <https://gnu.org/licenses/gpl.html>";
const LICENSE_3: &str = "This is free software: you are free to change and redistribute it.";
const LICENSE_4: &str = "There is ABSOLUTELY NO WARRANTY, to the extent permitted by law.";

const LICENSE_NOTE: &str = formatc!("{0} {LICENSE_1}\r\n{0} {LICENSE_2}\r\n{0} {LICENSE_3}\r\n{1} {LICENSE_4}\r\n", INFO, WARN);

const VERSION_1: &str = formatc!("{}: {} [{}]", build::PROJECT_NAME, build::PKG_VERSION, build::RUST_VERSION);
const VERSION_2: &str = formatc!("Date: {}", build::BUILD_TIME);
const VERSION_3: &str = formatc!("Built from {}, on branch: {}", build::COMMIT_HASH, build::BRANCH);

const VERSION_NOTE: &str = formatc!("{0} {VERSION_1}\r\n{0} {VERSION_2}\r\n{0} {VERSION_3}\r\n", INFO);

/// Welcome message to print when starting REPL.
pub const WELCOME_MOTD: &str = formatc!("{0} Welcome to JUK2\r\n{0} Type `?` anytime for help\r\n", INFO);

/// Prints license and version info to [`Terminal`].
pub async fn print_verinfo<T: Terminal>(term: &mut T) -> Result<(), T::Error> {
    term.write(b"\r\n\r\n").await?;
    term.write(LICENSE_NOTE.as_bytes()).await?;
    term.write(b"\r\n").await?;
    term.write(VERSION_NOTE.as_bytes()).await?;
    term.write(b"\r\n").await?;
    Ok(())
}
