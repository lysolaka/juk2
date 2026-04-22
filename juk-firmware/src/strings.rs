use const_format::{concatcp, formatcp};

shadow_rs::shadow!(build);

pub const INFO: &str = "\x1b[1;32m*\x1b[0m ";
pub const WARN: &str = "\x1b[1;33m*\x1b[0m ";

const VERSION_VER: &str = formatcp!(
    "{}: {} ({})\r\n",
    build::PROJECT_NAME,
    build::PKG_VERSION,
    build::BUILD_TIME,
);
const VERSION_GIT: &str = formatcp!(
    "Built from {} ({}), on branch `{}`\r\n",
    build::COMMIT_HASH,
    build::COMMIT_DATE,
    build::BRANCH
);

#[rustfmt::skip]
pub const LICENSE: &str = concatcp!(
    INFO, "juk-firmware Copyright (C) 2026 lysolaka\r\n",
    INFO, "License GNU GPL-3.0 <https://gnu.org/licenses/gpl.html>\r\n",
    INFO, "This is free software: you are free to change and redistribute it.\r\n",
    WARN, "There is ABSOLUTELY NO WARRANTY, to the extent permitted by law.\r\n"
);

#[rustfmt::skip]
pub const VERSION: &str = concatcp!(
    INFO, VERSION_VER,
    INFO, VERSION_GIT
);

#[rustfmt::skip]
pub const WELCOME: &str = concatcp!(
    INFO, "Welcome to JUK2\r\n",
    INFO, "Type `?` anytime for help\r\n"
);
