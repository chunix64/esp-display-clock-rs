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

// -------------

mod app;
mod hardware;
mod models;
mod types;
mod ui;

use embassy_executor::Spawner;

use crate::app::App;
use crate::hardware::backlight::ledc::Backlight;
use crate::hardware::board::Board;
use crate::hardware::display::display_controller::DisplayController;
use crate::hardware::display::spi_display::SpiDisplayBuilder;
use crate::hardware::radio::wifi::wifi_task;
use crate::models::clock::Clock;
use crate::types::WifiConfig;

const DISPLAY_BUFFER_SIZE: usize = 2048;
static mut DISPLAY_BUFFER: [u8; DISPLAY_BUFFER_SIZE] = [0u8; DISPLAY_BUFFER_SIZE];

#[allow(clippy::large_stack_frames)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // Pin assignments and peripheral configuration can be changed in src/hardware/board.rs
    let board = Board::init();
    let clock = Clock::default();

    let wifi_config = WifiConfig {
        ssid: heapless::String::try_from("YOUR_SSID").unwrap(),
        password: heapless::String::try_from("YOUR_SSID_PASSWORD").unwrap(),
    };

    spawner.spawn(wifi_task(board.app_peripherals.wifi, wifi_config).unwrap());

    let display_buffer: &'static mut [u8; DISPLAY_BUFFER_SIZE] =
        unsafe { &mut *core::ptr::addr_of_mut!(DISPLAY_BUFFER) };

    // -------------
    let mut backlight = Backlight::new(board.app_peripherals.ledc, board.backlight_config);
    let backlight_controller = backlight.get_controller();

    let display = SpiDisplayBuilder::build(
        board.app_peripherals.spi,
        board.display_config,
        display_buffer,
    );
    let display_controller = DisplayController::new(display, Some(backlight_controller));

    let mut app = App::new(display_controller, &clock);
    app.run(spawner).await;
}
