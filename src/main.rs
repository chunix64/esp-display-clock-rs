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
use crate::config::{AppPinConfig, DisplayConfig};
use crate::display::spi_display::SpiDisplayBuilder;

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    let board = Board::init();
    board.reserve_pins();

    let mut delay = board.delay;
    let mut buffer = [0u8; 2048];

    let mut app_pin_config = AppPinConfig {
        ledc: Some(board.peripherals.LEDC),
        spi: Some(board.peripherals.SPI2.into()),
        sck: Some(board.peripherals.GPIO18.into()),
        mosi: Some(board.peripherals.GPIO23.into()),
        dc: Some(board.peripherals.GPIO2.into()),
        cs: Some(board.peripherals.GPIO5.into()),
        rst: Some(board.peripherals.GPIO4.into()),
        backlight: Some(board.peripherals.GPIO14.into()),
    };

    let display_config = DisplayConfig {
        display_width: 240,
        display_height: 320,
        display_model: mipidsi::models::ST7789,
    };

    let display = SpiDisplayBuilder::build(&mut app_pin_config, display_config, &mut delay, &mut buffer);
    let mut backlight = Backlight::new(&mut app_pin_config);
    let backlight_controller = backlight.get_backlight_controller();

    let mut app = App::new(display, Some(backlight_controller), delay);
    delay.delay_millis(500);
    app.run();
}
