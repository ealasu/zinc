#!/bin/bash

cargo build --release
arm-none-eabi-objcopy -O binary -j .text -j .data target/thumbv7em-none-eabi/release/blink target/app.bin
arm-none-eabi-objcopy -R .stack -O ihex target/thumbv7em-none-eabi/release/blink target/app.hex
teensy_loader_cli -w -mmcu=mk20dx256 target/app.hex
