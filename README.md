# JUK2

JUK2 is a CNC plotter firmware for the ESP32-S3 and the non-existant (for now) custom PCB and plotter frame.

JUK2 is the successor of JUK - a private, messy and ugly project. It's second version is supposed to be everything the first one was not able to be.

# Hardware

For now the firmware is developed for [ESP32-S3-DEV-KIT-N8R8](https://www.waveshare.com/wiki/ESP32-S3-DEV-KIT-N8R8)

*TODO:* write some more about the hardware: motors used, kinematics.

# Software

Just like the first edition of JUK, the entire software stack is developed using Rust. This time the project is split into crates in a single workspace. The structure is as follows:
- `juk-firmware`: the main firmware executable
- `juk-led`: a simple library to use the onboard WS2812B RGB LED (one of many libraries of this type)

## Toolchain

Building this project requires the Xtensa Rust Toolchain, detailed installation instructions can be found [here](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html). Additionaly the [espflash](https://github.com/esp-rs/espflash/tree/main/espflash) utility is needed to flash the program onto the board.

# License

All code in this repository is licensed under the GPL-3 license.
