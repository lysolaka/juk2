alias b := build
alias r := run
alias mon := monitor
alias con := connect

jtag := "/dev/serial/by-id/usb-Espressif_USB_JTAG_serial_debug_unit_D8:3B:DA:4A:FC:30-if00"
uart := "/dev/serial/by-id/usb-1a86_USB_Single_Serial_5A67168064-if00"

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
