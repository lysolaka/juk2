alias b := build
alias r := run
alias mon := monitor
alias con := connect

jtag := "/dev/ttyACM0"
uart := "/dev/ttyUSB0"

elf := "./target/xtensa-esp32s3-none-elf/release/juk-firmware"
log-output := "[{L:severity:4}] {s}"

[private]
default:
  @just --list

# Build release configuration
build:
  @cargo build --release

# Run release configuration
run:
  @cargo build --release
  espflash flash --monitor --port {{jtag}} --chip esp32s3 --log-format defmt --output-format '{{log-output}}' {{elf}}


# Monitor logs
monitor:
  @espflash monitor --port {{jtag}} --chip esp32s3 --log-format defmt --output-format '{{log-output}}' --elf {{elf}}

# Connect using UART
connect:
  @picocom --baud 115200 --flow n --parity n --databits 8 --stopbits 1 {{uart}}
