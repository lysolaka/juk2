# JUK2

JUK2 is a CNC plotter firmware for the ESP32-S3 on a custom PCB and plotter frame.

JUK2 is the successor of JUK - a private, messy and ugly project. It's second version is supposed to be everything the first one was not able to be.

# Hardware

The custom design can be found under the `juk-pcb` directory. It also has a schematic and a STEP model available for those, who don't have KiCad installed.

*TODO:* add a nice picture of the board, once it arrives

*TODO:* write some more about the hardware: motors used, kinematics.

# Software

Just like the first edition of JUK, the entire software stack is developed using Rust. This time the project is split into crates in a single workspace. The structure is as follows:
- `juk-com`: a communication library implementing text and binary interfaces
- `juk-firmware`: the main firmware executable
- `juk-led`: a simple library to use the onboard WS2812B RGB LED (one of many libraries of this type)

## Toolchain

Building this project requires the Xtensa Rust Toolchain, detailed installation instructions can be found [here](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html). Additionaly the [espflash](https://github.com/esp-rs/espflash/tree/main/espflash) utility is needed to flash the program onto the board.

# License

All code in this repository is licensed under the GPL-3 license.
