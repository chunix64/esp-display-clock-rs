use esp_hal::{
    clock::CpuClock,
    interrupt::software::SoftwareInterruptControl,
    peripherals::{Peripherals, SW_INTERRUPT},
    timer::timg::TimerGroup,
};

pub struct Board {
    pub peripherals: Peripherals,
}

impl Board {
    pub fn init() -> Self {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);
        esp_alloc::heap_allocator!(size: 128 * 1024);

        esp_println::logger::init_logger_from_env();

        Self { peripherals }
    }

    pub fn start_rtos<T>(timer_group: T, sw_interrupt: SW_INTERRUPT<'static>)
    where
        T: esp_hal::timer::timg::TimerGroupInstance + 'static,
    {
        let timg0 = TimerGroup::new(timer_group);
        let sw_interrupt = SoftwareInterruptControl::new(sw_interrupt);
        esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
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
