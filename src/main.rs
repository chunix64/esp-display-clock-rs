#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    log::info!("[PANIC] {}", panic_info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// -----------

mod app;
mod backlight;
mod board;
mod config;
mod display;

use embassy_executor::Spawner;

use crate::app::App;
use crate::backlight::ledc::Backlight;
use crate::board::Board;
use crate::config::{AppPeripherals, BacklightConfig, DisplayConfig, DisplayPins};
use crate::display::spi_display::SpiDisplayBuilder;

#[allow(clippy::large_stack_frames)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let board = Board::init();
    board.reserve_pins();
    board::Board::start_rtos(board.peripherals.TIMG0, board.peripherals.SW_INTERRUPT);

    // Config
    let app_peripherals = AppPeripherals {
        ledc: board.peripherals.LEDC,
        spi: board.peripherals.SPI2.into(),
    };

    let display_pins = DisplayPins {
        sck: board.peripherals.GPIO18.into(),
        mosi: board.peripherals.GPIO23.into(),
        dc: board.peripherals.GPIO2.into(),
        cs: board.peripherals.GPIO5.into(),
        rst: board.peripherals.GPIO4.into(),
    };

    let display_config = DisplayConfig {
        display_width: 240,
        display_height: 320,
        pins: display_pins,
    };

    let backlight_config = BacklightConfig {
        pin: board.peripherals.GPIO14.into(),
    };

    let display_model = mipidsi::models::ST7789;

    let mut display_buffer = [0u8; 2048];

    // Main logic
    let _ = spawner;
    let mut delay = board.delay;

    let mut backlight = Backlight::new(app_peripherals.ledc, backlight_config);
    let backlight_controller = backlight.get_controller();

    let display = SpiDisplayBuilder::build(
        app_peripherals.spi,
        display_config,
        display_model,
        &mut delay,
        &mut display_buffer,
    );

    let mut app = App::new(display, Some(backlight_controller), delay);
    app.run().await;
}
