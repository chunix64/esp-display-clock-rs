#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// -----------

mod app;
mod backlight;
mod board;
mod config;
mod display;

use esp_hal::main;

use crate::app::App;
use crate::backlight::ledc::Backlight;
use crate::board::Board;
use crate::config::{AppPins, DisplayConfig, DisplayPins};
use crate::display::spi_display::SpiDisplayBuilder;

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    let board = Board::init();
    board.reserve_pins();

    // Config
    let mut app_pins = AppPins {
        ledc: Some(board.peripherals.LEDC),
        spi: Some(board.peripherals.SPI2.into()),
    };

    let display_pins = DisplayPins {
        sck: Some(board.peripherals.GPIO18.into()),
        mosi: Some(board.peripherals.GPIO23.into()),
        dc: Some(board.peripherals.GPIO2.into()),
        cs: Some(board.peripherals.GPIO5.into()),
        rst: Some(board.peripherals.GPIO4.into()),
        backlight: Some(board.peripherals.GPIO14.into()),
    };

    let mut display_config = DisplayConfig {
        display_model: Some(mipidsi::models::ST7789),
        display_width: 240,
        display_height: 320,
        pins: display_pins,
    };

    // Main logic
    let mut delay = board.delay;
    let mut buffer = [0u8; 2048];

    let mut display =
        SpiDisplayBuilder::build(&mut app_pins, &mut display_config, &mut delay, &mut buffer);
    let mut backlight = Backlight::new(&mut app_pins, &mut display_config.pins);
    let backlight_controller = backlight.get_backlight_controller();

    let mut app = App::new(&mut display, Some(backlight_controller), delay);
    app.run();
}
