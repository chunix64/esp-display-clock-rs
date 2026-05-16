# embeddeck-rs

A modular, general-purpose embedded display runtime for ESP32, written in Rust. Built on the `esp-hal` ecosystem and `embassy` in `no_std` rust. `embeddeck` turns a microcontroller + screen into a multi-app smart panel. It have weather, AI, media info, reading, and more - all running on hardware that fits in your hand.

<img width="1280" height="960" alt="photo_2026-05-14_09-29-52" src="https://github.com/user-attachments/assets/f49d334e-44d5-4c9e-965e-0d795910060a" />

## Features

### Current
- NTP time sync over WiFi
- Adjustable backlight with smooth PWM control
- Multiple display driver support, built on `mipidsi` (default: ST7789)
- Shared TUI rendering via `ratatui` (Desktop + embedded)
- Built with async/await using `embassy`

### Coming soon
- Webui to login, configure and control the app and hardware.
- Weather with live conditions and forecast display.
- Reading - render text content, feeds, or documents.
- AI integration - chatbot, assistant.
- Sound & music - play music with music player.
- PC sync - mirror or relay data from a connected desktop
- Behave as a external display

## Hardware

Tested with:
- Classic ESP32 (ESP32-D0WD-V3 - revision v3.1)
- 240×320 ST7789 SPI IPS display
- Backlight connected to a PWM-capable GPIO

Default pinout (defined in `crates/embeddeck-embedded/src/hardware/board.rs`):

| Function    | GPIO |
|-------------|------|
| SPI SCK     | 18   |
| SPI MOSI    | 23   |
| Display DC  | 2    |
| Display CS  | 5    |
| Display RST | 4    |
| Backlight   | 14   |

Pins can be changed by editing the board configuration.

Other display drivers supported by `mipidsi` (ILI9341, ILI9486, SSD1351, etc.) can be wired in with minimal changes at `crates/embeddeck-embedded/src/hardware/display/types.rs`.

## Prerequisites

- `rustc`: 1.91+
- Xtensa toolchain for embedded targets: install via [`espup`](https://github.com/esp-rs/espup)

## Project Structure

Cargo workspace split into three crates:

```
crates/
├── embeddeck-core/        # Shared models, functions, etc
│   └── src/
│       └── models/        # Shared models
│
├── embeddeck-ui/          # Shared UI library
│   └── src/
│       ├── models/        # UI-specific models
│       ├── screens/       # Screen definitions
│       └── widgets/       # Ratatui widgets
│
├── embeddeck-embedded/    # ESP32 embedded target
│   └── src/
│       ├── hardware/      # Board, display (SPI), backlight (LEDC), WiFi
│       ├── models/        # Embedded-specific models and configs
│       ├── services/      # NTP, web server, network services
│       ├── actors/        # Async actor tasks
│       └── main.rs
│
└── embeddeck-desktop/     # Desktop simulator target
    └── src/
        ├── actors/        # Desktop actor tasks
        ├── models/        # Desktop-specific models
        └── main.rs
```

`embeddeck-ui` and `embeddeck-core` are shared between both targets.

## Quick Start

### 1. Clone

```bash
git clone https://github.com/chunix64/embeddeck-rs.git
cd embeddeck-rs
```

### 2. Configure WiFi

- Optional, but required for network-related tasks such as NTP.

Edit `crates/embeddeck-embedded/src/main.rs`:

```rust
let wifi_config = WifiConfig {
    ssid: heapless::String::try_from("YOUR_SSID").unwrap(),
    password: heapless::String::try_from("YOUR_PASSWORD").unwrap(),
};
```

### 3. Build & Flash

```bash
cd crates/embeddeck-embedded
cargo run --release
```

Output binary:
```
crates/embeddeck-embedded/target/xtensa-esp32-none-elf/release/embeddeck-embedded
```

### Desktop Simulator

Develop and iterate on UI without any hardware:

```bash
cd crates/embeddeck-desktop
cargo run --release
```

## Roadmap

- [ ] Weather integration
- [ ] Configuration via web interface or BLE
- [ ] Multiple UI layouts and themes
- [ ] Reading / RSS / document rendering
- [ ] AI display 
- [ ] Sound & music info display and music player
- [ ] PC sync (desktop <-> device)
- [ ] Deep sleep / low power modes
- [ ] Touch support
- [ ] Broader display driver support (ILI9341, SSD1351, etc.)

## Contributing

Contributions are welcome - open an issue or pull request.

## License

MIT License.

## Acknowledgments

- [esp-hal](https://github.com/esp-rs/esp-hal)
- [embassy](https://github.com/embassy-rs/embassy)
- [mipidsi](https://github.com/almindor/mipidsi)
- [ratatui](https://github.com/ratatui-org/ratatui)
