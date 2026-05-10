use esp_hal::{
    clock::CpuClock, interrupt::software::SoftwareInterruptControl, peripherals::SW_INTERRUPT,
    timer::timg::TimerGroup,
};
use mipidsi::models::ST7789;

use crate::config::{AppPeripherals, BacklightConfig, DisplayConfig, DisplayPins};

pub struct Board {
    pub app_peripherals: AppPeripherals,
    pub display_config: DisplayConfig<ST7789>,
    pub backlight_config: BacklightConfig,
}

impl Board {
    pub fn init() -> Self {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);

        esp_alloc::heap_allocator!(size: 128 * 1024);
        esp_println::logger::init_logger_from_env();
        Self::start_rtos(peripherals.TIMG0, peripherals.SW_INTERRUPT);

        // Don't use those pins
        // The following pins are used to bootstrap the chip. They are available
        // for use, but check the datasheet of the module for more information on them.
        // - GPIO0
        // - GPIO2
        // - GPIO5
        // - GPIO12
        // - GPIO15
        // These GPIO pins are in use by some feature of the module and should not be used.
        let _ = peripherals.GPIO6;
        let _ = peripherals.GPIO7;
        let _ = peripherals.GPIO8;
        let _ = peripherals.GPIO9;
        let _ = peripherals.GPIO10;
        let _ = peripherals.GPIO11;
        let _ = peripherals.GPIO16;
        let _ = peripherals.GPIO20;

        let app_peripherals = AppPeripherals {
            ledc: peripherals.LEDC,
            spi: peripherals.SPI2.into(),
        };

        let display_pins = DisplayPins {
            sck: peripherals.GPIO18.into(),
            mosi: peripherals.GPIO23.into(),
            dc: peripherals.GPIO2.into(),
            cs: peripherals.GPIO5.into(),
            rst: peripherals.GPIO4.into(),
        };

        let display_config = DisplayConfig {
            display_model: mipidsi::models::ST7789,
            display_width: 240,
            display_height: 320,
            pins: display_pins,
        };

        let backlight_config = BacklightConfig {
            pin: peripherals.GPIO14.into(),
        };

        Self {
            app_peripherals,
            display_config,
            backlight_config,
        }
    }

    pub fn start_rtos<T>(timer_group: T, sw_interrupt: SW_INTERRUPT<'static>)
    where
        T: esp_hal::timer::timg::TimerGroupInstance + 'static,
    {
        let timg0 = TimerGroup::new(timer_group);
        let sw_interrupt = SoftwareInterruptControl::new(sw_interrupt);
        esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
    }
}
