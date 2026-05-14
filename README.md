# embeddeck-rs

A modular, general-purpose embedded display runtime for ESP32, written in Rust. Built on the ESP-HAL ecosystem and Embassy, `embeddeck` turns a microcontroller + screen into a multi-app smart panel — think weather, AI, media info, reading, and more — all running on hardware that fits in your hand.

<img width="1280" height="960" alt="photo_2026-05-14_09-29-52" src="https://github.com/user-attachments/assets/f49d334e-44d5-4c9e-965e-0d795910060a" />

## Features

- **Multi-app, multi-screen** — switch between apps/screens at runtime
- **Weather** — live conditions and forecast display (planned)
- **AI integration** — on-device or networked inference display (planned)
- **Reading** — render text content, feeds, or documents (planned)
- **Sound & music** — playback info, visualizers (planned)
- **PC sync** — mirror or relay data from a connected desktop (planned)
- **NTP time sync** over WiFi
- **Adjustable backlight** with smooth PWM control
- **Multiple display driver support** — not locked to ST7789, built on `mipidsi`
- **TUI rendering** via `embedded-graphics` + `ratatui`
- **Desktop simulator** — iterate on UI without hardware
- **Multi-crate workspace** with clean separation of concerns
- Built with async/await using Embassy

## Hardware

Tested with:
- ESP32 (various modules)
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

Pins can be changed by editing the board configuration. Other display drivers supported by `mipidsi` (ILI9341, ILI9486, SSD1351, etc.) can be wired in with minimal changes.

## Prerequisites

- `rustc`: 1.91+
- Xtensa toolchain for embedded targets: install via [`espup`](https://github.com/esp-rs/espup)

## Project Structure

Cargo workspace split into three crates:

```
crates/
├── embeddeck-ui/          # Shared UI library (target-agnostic)
│   └── src/
│       ├── models/        # Shared data models
│       ├── screens/       # Screen definitions
│       └── widgets/       # Ratatui widgets
│
├── embeddeck-embedded/    # ESP32 embedded target
│   └── src/
│       ├── hardware/      # Board, display (SPI), backlight (LEDC), WiFi
│       ├── models/        # Embedded-specific models and configs
│       ├── services/      # NTP, web server, embassy-net
│       ├── actors/        # Async actor tasks
│       └── main.rs
│
└── embeddeck-desktop/     # Desktop simulator target
    └── src/
        ├── actors/        # Desktop actor tasks
        ├── models/        # Desktop-specific models
        └── main.rs
```

`embeddeck-ui` is shared between both targets, keeping all UI logic hardware-agnostic.

## Quick Start

### 1. Clone

```bash
git clone https://github.com/chunix64/embeddeck-rs.git
cd embeddeck-rs
```

### 2. Configure WiFi

Edit `crates/embeddeck-embedded/src/models/configs.rs`:

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
- [ ] AI display (local or networked)
- [ ] Reading / RSS / document rendering
- [ ] Sound & music info display
- [ ] PC sync (desktop ↔ device)
- [ ] Multiple UI layouts and themes
- [ ] Configuration via web interface or BLE
- [ ] Deep sleep / low power modes
- [ ] Touch support
- [ ] Broader display driver support (ILI9341, SSD1351, etc.)
- [ ] Sync between desktop simulator and embedded target

## Contributing

Contributions are welcome — open an issue or PR.

## License

MIT License.

## Acknowledgments

- [esp-hal](https://github.com/esp-rs/esp-hal)
- [Embassy](https://github.com/embassy-rs/embassy)
- [mipidsi](https://github.com/almindor/mipidsi)
- [ratatui](https://github.com/ratatui-org/ratatui)
