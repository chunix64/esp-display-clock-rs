use esp_hal::{clock::CpuClock, delay::Delay, peripherals::Peripherals};

pub struct Board {
    pub peripherals: Peripherals,
    pub delay: Delay,
}

impl Board {
    pub fn init() -> Self {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);

        esp_println::logger::init_logger_from_env();

        Self {
            peripherals,
            delay: Delay::new(),
        }
    }

    // Lock unsafe pins
    pub fn reserve_pins(&self) {
        // The following pins are used to bootstrap the chip. They are available
        // for use, but check the datasheet of the module for more information on them.
        // - GPIO0
        // - GPIO2
        // - GPIO5
        // - GPIO12
        // - GPIO15
        // These GPIO pins are in use by some feature of the module and should not be used.
        let _ = self.peripherals.GPIO6;
        let _ = self.peripherals.GPIO7;
        let _ = self.peripherals.GPIO8;
        let _ = self.peripherals.GPIO9;
        let _ = self.peripherals.GPIO10;
        let _ = self.peripherals.GPIO11;
        let _ = self.peripherals.GPIO16;
        let _ = self.peripherals.GPIO20;
    }
}
