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

mod actors;
mod app;
mod hardware;
mod models;
mod services;

use embassy_executor::Spawner;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use esp_hal::rtc_cntl::Rtc;
use esp_radio::wifi::WifiController;
use static_cell::StaticCell;

use crate::app::app_task;
use crate::hardware::backlight::ledc::Backlight;
use crate::hardware::board::Board;
use crate::hardware::display::display_controller::DisplayController;
use crate::hardware::display::spi_display::SpiDisplayBuilder;
use crate::hardware::radio::wifi::{init_network_stack, wifi_task};
use crate::models::clock::{EmbeddedClock, EmbeddedClockExt};
use crate::models::configs::WifiConfig;
use crate::services::embassy_net::{net_monitor_task, net_runner_task};
use crate::services::ntp::ntp_task;
use crate::services::webserver::webserver_task;

static WIFI_CONTROLLER: StaticCell<WifiController<'static>> = StaticCell::new();
static RTC: StaticCell<Rtc<'static>> = StaticCell::new();
static CLOCK: StaticCell<EmbeddedClock> = StaticCell::new();

const DISPLAY_BUFFER_SIZE: usize = 2048;
static DISPLAY_BUFFER: StaticCell<[u8; DISPLAY_BUFFER_SIZE]> = StaticCell::new();
static DISPLAY_CONTROLLER: StaticCell<DisplayController> = StaticCell::new();
static BACKLIGHT: StaticCell<Backlight> = StaticCell::new();

//
// Pin assignments and peripheral configuration can be changed in src/hardware/board.rs
//

#[allow(clippy::large_stack_frames)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // --- Board & RTC ---
    let board = Board::init();
    let rtc: &'static Rtc<'static> = RTC.init(Rtc::new(board.app_peripherals.lpwr));
    let clock: &'static EmbeddedClock = CLOCK.init(EmbeddedClock::default(rtc));

    // --- WiFi & Network ---
    let (wifi_controller_inner, wifi_interfaces) =
        esp_radio::wifi::new(board.app_peripherals.wifi, Default::default()).unwrap();
    let wifi_controller: &'static mut WifiController<'static> =
        WIFI_CONTROLLER.init(wifi_controller_inner);
    let (network_stack, network_runner) = init_network_stack(wifi_interfaces.station);

    // --- Display & Backlight ---
    let display_buffer: &'static mut [u8; DISPLAY_BUFFER_SIZE] =
        DISPLAY_BUFFER.init([0u8; DISPLAY_BUFFER_SIZE]);
    let backlight: &'static mut Backlight = BACKLIGHT.init(Backlight::new(
        board.app_peripherals.ledc,
        board.backlight_config,
    ));
    let backlight_controller = backlight.get_controller();
    let display = SpiDisplayBuilder::build(
        board.app_peripherals.spi,
        board.display_config,
        display_buffer,
    );
    let display_controller =
        DISPLAY_CONTROLLER.init(DisplayController::new(display, Some(backlight_controller)));

    // --- Config ---
    let wifi_config = WifiConfig {
        ssid: heapless::String::try_from("YOUR_SSID").unwrap(),
        password: heapless::String::try_from("YOUR_SSID_PASSWORD").unwrap(),
    };

    // --- Spawn Tasks ---
    spawner.spawn(wifi_task(wifi_controller, wifi_config).unwrap());
    spawner.spawn(net_runner_task(network_runner).unwrap());
    spawner.spawn(net_monitor_task(network_stack).unwrap());
    spawner.spawn(ntp_task(network_stack, rtc).unwrap());
    spawner.spawn(webserver_task(network_stack).unwrap());
    spawner.spawn(app_task(spawner, display_controller, clock).unwrap());

    loop {
        // Yield to other tasks, doing nothing
        Delay.delay_ms(3600).await;
    }
}
