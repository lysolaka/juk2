//! Builtin strings
use const_format::{concatcp, formatcp};

shadow_rs::shadow!(build);

/// Marker for an info message.
pub const INFO: &str = "\x1b[1;32m*\x1b[0m ";
/// Marker for a warning message.
pub const WARN: &str = "\x1b[1;33m*\x1b[0m ";
/// Marker for an error message.
pub const ERROR: &str = "\x1b[1;31m*\x1b[0m ";

const WHITE: &str = "\x1b[1;37m";
const CLEAR: &str = "\x1b[0m";

/// Prompt for the CLI.
pub const PROMPT: &str = concatcp!("JUK2 ", WHITE, "$", CLEAR, " ");

const VERSION_VER: &str = formatcp!(
    "{}: {} ({})\r\n",
    build::PROJECT_NAME,
    build::PKG_VERSION,
    build::BUILD_TIME,
);
const VERSION_GIT: &str = formatcp!(
    "Built from {} ({}), on branch `{}`\r\n",
    build::SHORT_COMMIT,
    build::COMMIT_DATE,
    build::BRANCH
);

/// License note. States that the program is licensed under GNU GPL-3.
#[rustfmt::skip]
pub const LICENSE: &str = concatcp!(
    INFO, "juk-firmware Copyright (C) 2026 lysolaka\r\n",
    INFO, "License GNU GPL-3.0 <https://gnu.org/licenses/gpl.html>\r\n",
    INFO, "This is free software: you are free to change and redistribute it.\r\n",
    WARN, "There is ABSOLUTELY NO WARRANTY, to the extent permitted by law.\r\n"
);

/// Version note: package version and commit information.
#[rustfmt::skip]
pub const VERSION: &str = concatcp!(
    INFO, VERSION_VER,
    INFO, VERSION_GIT
);

/// Welcome message.
#[rustfmt::skip]
pub const WELCOME: &str = concatcp!(
    INFO, "Welcome to JUK2\r\n",
    INFO, "Type ", WHITE, "?", CLEAR, " anytime for help\r\n"
);

/// Help messages
pub mod help {
    use super::*;

    #[rustfmt::skip]
    pub const LIST: &str = concatcp!(
        INFO, "Commands list:\r\n",
        INFO, "\r\n",
        INFO, "  `move` - Move in a Line\r\n",
        INFO, "  `arc` - Move in an Arc\r\n",
        INFO, "  `home` - Perform Homing\r\n",
        INFO, "  `set` - Alter the configuration\r\n",
        INFO, "  `get` - View the configuration\r\n",
    );

    #[rustfmt::skip]
    pub const MOVE: &str = concatcp!(
        INFO, "`move` - Move in a Line\r\n",
        INFO, "\r\n",
        INFO, "Keys:\r\n",
        INFO, "  `x`: X axis displacement.\r\n",
        INFO, "  `y`: Y axis displacement.\r\n",
        INFO, "  `z`: Z axis displacement.\r\n",
        INFO, "  `a`: Movement acceleration.\r\n",
        INFO, "  `v`: Movement velocity.\r\n",
        INFO, "\r\n",
        INFO, "Relation: `|| ( x y z )`\r\n",
    );

    #[rustfmt::skip]
    pub const ARC: &str = concatcp!(
        INFO, "`arc` - Move in an Arc\r\n",
        INFO, "\r\n",
        INFO, "Keys:\r\n",
        INFO, "  `x`: X axis displacement.\r\n",
        INFO, "  `y`: Y axis displacement.\r\n",
        INFO, "  `z`: Z axis displacement. The arc is done in the XY plane. Use this to achieve helical motion.\r\n",
        INFO, "  `r`: Arc radius. Adheres to the same rules as displacement, but must not be negative.\r\n",
        INFO, "  `dir`: direction of the arc. `pos` means the direction where the angle of the radius vector increases, `neg` - where the angle decreases. Allowed values: `pos`, `neg`. Default is: `pos`\r\n",
        INFO, "  `a`: Movement acceleration.\r\n",
        INFO, "  `v`: Movement velocity.\r\n",
        INFO, " \r\n",
        INFO, "Relation: `* ( x y r )`\r\n",
    );

    #[rustfmt::skip]
    pub const HOME: &str = concatcp!(
        INFO, "`home` - Perform Homing\r\n",
        INFO, "\r\n",
        INFO, "Keys:\r\n",
        INFO, "  `axis`: the axis (axes) to perform homing on. The only axes are `x`, `y`, `z`, so any combination of these three may be used. Default is: `xyz`.\r\n",
    );

    #[rustfmt::skip]
    pub const SET: &str = concatcp!(
        INFO, "`set` - Alter the configuration\r\n",
        INFO, "\r\n",
        INFO, "Keys:\r\n",
        INFO, "  `accel`\r\n",
        INFO, "  `frame`\r\n",
        INFO, "  `led`\r\n",
        INFO, "  `mmpsX`\r\n",
        INFO, "  `mmpsY`\r\n",
        INFO, "  `mmpsZ`\r\n",
        INFO, "  `posX`\r\n",
        INFO, "  `posY`\r\n",
        INFO, "  `posZ`\r\n",
        INFO, "  `unit`\r\n",
        INFO, "  `vel`\r\n",
        INFO, "For the meaning of the keys, please read the reference found at `REFERENCE.md`\r\n",
        INFO, " \r\n",
        INFO, "Relation: `|| ( <keys> )`\r\n",
    );

    #[rustfmt::skip]
    pub const GET: &str = concatcp!(
        INFO, "`get` - View the configuration\r\n",
        INFO, "\r\n",
        INFO, "This commands functions differently than the others, as the keys do not accept a value.\r\n",
        INFO, "\r\n",
        INFO, "Keys:\r\n",
        INFO, "  `accel`\r\n",
        INFO, "  `frame`\r\n",
        INFO, "  `led`\r\n",
        INFO, "  `license`\r\n",
        INFO, "  `limits`\r\n",
        INFO, "  `mmps`\r\n",
        INFO, "  `mmpsX`\r\n",
        INFO, "  `mmpsY`\r\n",
        INFO, "  `mmpsZ`\r\n",
        INFO, "  `pos`\r\n",
        INFO, "  `posX`\r\n",
        INFO, "  `posY`\r\n",
        INFO, "  `posZ`\r\n",
        INFO, "  `status`\r\n",
        INFO, "  `unit`\r\n",
        INFO, "  `vel`\r\n",
        INFO, "  `version`\r\n",
        INFO, "For the meaning of the keys, please read the reference found at `REFERENCE.md`\r\n",
        INFO, " \r\n",
        INFO, "Relation: `^^ ( <keys> )`\r\n",
    );
}
