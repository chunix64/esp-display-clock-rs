use embedded_hal::pwm::SetDutyCycle;
use esp_hal::{
    gpio::{AnyPin, DriveMode},
    ledc::{
        Ledc, LowSpeed,
        channel::{self, ChannelIFace},
        timer::{self, Timer, TimerIFace},
    },
    peripherals::LEDC,
    time::Rate,
};

use crate::config::BacklightConfig;

pub struct Backlight<'a> {
    backlight_pin: Option<AnyPin<'static>>,
    timer: Timer<'a, LowSpeed>,
    ledc: Ledc<'a>,
}

//  brightness: min_brightness -> 100
pub struct BacklightController<'a> {
    backlight: channel::Channel<'a, LowSpeed>,
    brightness: u8,
    min_brightness: u8,
}

impl<'a> Backlight<'a> {
    pub fn new(ledc_peripheral: LEDC<'a>, backlight_config: BacklightConfig) -> Self {
        let mut ledc = Ledc::new(ledc_peripheral);
        ledc.set_global_slow_clock(esp_hal::ledc::LSGlobalClkSource::APBClk);

        let mut timer = ledc.timer(timer::Number::Timer0);

        timer
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty8Bit,
                clock_source: timer::LSClockSource::APBClk,
                frequency: Rate::from_khz(24),
            })
            .unwrap();

        Self {
            backlight_pin: Some(backlight_config.pin),
            timer,
            ledc,
        }
    }

    pub fn get_controller(&mut self) -> BacklightController<'_> {
        let mut channel0 = self.ledc.channel(
            channel::Number::Channel0,
            self.backlight_pin.take().unwrap(),
        );
        channel0
            .configure(channel::config::Config {
                timer: &self.timer,
                duty_pct: 100,
                drive_mode: DriveMode::PushPull,
            })
            .unwrap();

        BacklightController {
            backlight: channel0,
            brightness: 100,
            min_brightness: 0,
        }
    }
}

impl<'a> BacklightController<'a> {
    pub fn set_brightness(&mut self, brightness: u8) -> bool {
        self.brightness = brightness.clamp(self.min_brightness, 100);
        self.backlight
            .set_duty_cycle_percent(self.brightness)
            .is_ok()
    }

    pub fn set_min_brightness(&mut self, min_brightness: u8) {
        self.min_brightness = min_brightness.clamp(0, 100);
    }

    // pub fn get_brightness(&self) -> u8 {
    //     self.brightness
    // }
}
