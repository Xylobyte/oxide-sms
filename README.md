# oxide-sms

> ⚠️ **Project Status:** Under active development. Features, APIs, and architecture are subject to change.

A modular, `no_std` Sega Master System emulator written from scratch in Rust, targeted for native execution on the **Raspberry Pi Pico 2 (RP2350)** with DVI/HDMI output.

## Philosophy & Architecture

To achieve clean, professional, and portfolio-grade code quality, this project follows strict design principles:

* **Strictly Specification-Driven:** Developed from scratch relying **exclusively on official hardware documentation** and technical manuals located in the `docs/` folder. No third-party emulation code or libraries are used.
* **Modular `no_std` Core:** Complete decoupling of the emulation engine from microcontroller hardware peripherals.
* **Test-Driven Validation:** Validated through CPU and hardware test ROMs located in `docs/test-roms/`.

## Features

* **Pure `no_std` Core (`sms-core`):** Hardware-agnostic emulation engine (Z80 CPU, VDP, PSG) with zero external dependencies.
* **Hardware-Accelerated Video:** Uses the RP2350's **HSTX** peripheral in DDR mode (126 MHz) for native 640×480 DVI/HDMI output with minimal CPU overhead.
* **USB HID Support:** Native USB Host controller interface for standard USB gamepads.
* **Desktop Harness:** Run, test, and debug the core emulator on host platforms (Linux/macOS/Windows) before hardware deployment.

## Repository Structure

```text
oxide-sms/
├── crates/
│   ├── sms-core/       # Hardware-agnostic SMS emulation logic (Z80, VDP, PSG)
│   └── rp2350-dvi/     # RP2350 HSTX / DVI driver implementation
├── docs/               # Official datasheets, chip manuals, and technical docs
│   └── test-roms/      # Z80 and VDP validation ROMs (e.g. ZEXALL, hardware tests)
├── firmware/           # RP2350 release binary target (#![no_main])
└── runner-desktop/     # PC runner (minifb) for rapid debugging and test execution

```

## Getting Started

### 1. Running on PC (Desktop Harness)

You can run test ROMs (from `docs/test-roms/`) or SMS software locally using the desktop harness:

```bash
cargo run -p runner-desktop

```

### 2. Building for Raspberry Pi Pico 2 (RP2350)

Add the ARM Cortex-M33 target:

```bash
rustup target add thumbv8m.main-none-eabi

```

Build the release firmware binary:

```bash
cargo build --release -p firmware --target thumbv8m.main-none-eabi

```

## Hardware Setup

* **Microcontroller:** Raspberry Pi Pico 2 (RP2350)
* **Display Output:** HDMI/DVI breakout board (e.g., Pico DV Sock) wired to HSTX GPIO pins
* **Input:** USB-A female socket / OTG breakout connected to USB pins

## License

Dual-licensed under [MIT](https://www.google.com/search?q=LICENSE-MIT) or [Apache-2.0](https://www.google.com/search?q=LICENSE-APACHE).