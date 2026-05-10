#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
// #![deny(clippy::large_stack_frames)]

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    log::info!("[PANIC] {}", panic_info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

extern crate alloc;

// -----------

mod app;
mod config;
mod hardware;
mod models;
mod ui;

use embassy_executor::Spawner;
use static_cell::StaticCell;

use crate::app::App;
use crate::config::{AppPeripherals, BacklightConfig, DisplayConfig, DisplayPins};
use crate::hardware::backlight::ledc::Backlight;
use crate::hardware::board::Board;
use crate::hardware::display::display_controller::DisplayController;
use crate::hardware::display::spi_display::SpiDisplayBuilder;
use crate::hardware::timer::system_timer::timer_task;
use crate::models::clock::Clock;

static CLOCK: StaticCell<Clock> = StaticCell::new();

#[allow(clippy::large_stack_frames)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let board = Board::init();
    board.reserve_pins();
    Board::start_rtos(board.peripherals.TIMG0, board.peripherals.SW_INTERRUPT);
    let clock = CLOCK.init(Clock::default());
    let clock_ptr = clock as *mut Clock;

    spawner.spawn(timer_task(clock).unwrap());

    let clock_read: &'static Clock = unsafe { &*clock_ptr };

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
        display_model: mipidsi::models::ST7789,
        display_width: 240,
        display_height: 320,
        pins: display_pins,
    };

    let backlight_config = BacklightConfig {
        pin: board.peripherals.GPIO14.into(),
    };

    let display_buffer: &'static mut [u8; 2048] = {
        static mut DISPLAY_BUFFER: [u8; 2048] = [0u8; 2048];
        unsafe { &mut *core::ptr::addr_of_mut!(DISPLAY_BUFFER) }
    };

    // Main logic
    let mut backlight = Backlight::new(app_peripherals.ledc, backlight_config);
    let backlight_controller = backlight.get_controller();

    let display = SpiDisplayBuilder::build(app_peripherals.spi, display_config, display_buffer);
    let display_controller = DisplayController::new(display, Some(backlight_controller));

    let mut app = App::new(display_controller, clock_read);
    app.run(spawner).await;
}
