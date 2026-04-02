alias b := build
alias r := run
alias mon := monitor
alias con := connect

jtag := "/dev/ttyACM0"
uart := "/dev/ttyACM1"

[private]
default:
  @just --list

# Build release configuration
build:
  @cargo build --release

# Run release configuration
run:
  @cargo run --release

# Monitor logs
monitor:
  @espflash flash --monitor --port {{jtag}} --chip esp32s3 --log-format defmt --output-format '[{L:severity:4}] {s}' target/xtensa-esp32s3-none-elf/release/juk-firmware

# Connect using UART
connect:
  @picocom --baud 115200 --flow n --parity n --databits 8 --stopbits 1 {{uart}}
