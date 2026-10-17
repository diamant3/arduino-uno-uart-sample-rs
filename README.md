# arduino-uno-uart-sample-rs
Bare-metal UART sample code for Arduino Uno R3 (Microchip ATmega328P) in Rust

# Build

```
cd arduino-uno-uart-sample-rs && cargo build --release
```

```
cd target/avr-none/release && avr-objcopy -j .text -j .data -O ihex arduino-uno-uart-sample-rs.elf arduino-uno-uart-sample-rs.hex
```

# Flash

```
avrdude -F -V -c arduino -p atmega328p -P /dev/ttyUSB0 -b 115200 -U flash:w:arduino-uno-uart-sample-rs.hex
```

# Run

```
tio /dev/ttyUSB0 -b 115200
```

# Credits

- https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
- https://os.phil-opp.com/freestanding-rust-binary/
- https://doc.rust-lang.org/rustc/platform-support/avr-none.html

