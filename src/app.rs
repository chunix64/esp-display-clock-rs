use embedded_hal::digital::OutputPin;
use esp_hal::delay::Delay;
use mipidsi::{
    interface::{Interface, InterfacePixelFormat},
    models::Model,
};

use crate::backlight::ledc::BacklightController;

pub struct App<'a, DI, MODEL, RST>
where
    DI: Interface,
    MODEL: Model,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    RST: OutputPin,
{
    display: &'a mut mipidsi::Display<DI, MODEL, RST>,
    backlight_controller: Option<BacklightController<'a>>,
    delay: Delay,
}

impl<'a, DI, MODEL, RST> App<'a, DI, MODEL, RST>
where
    DI: Interface,
    MODEL: Model,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    RST: OutputPin,
{
    pub fn new(
        display: &'a mut mipidsi::Display<DI, MODEL, RST>,
        backlight_controller: Option<BacklightController<'a>>,
        delay: Delay,
    ) -> Self {
        Self {
            display,
            backlight_controller,
            delay,
        }
    }

    pub fn run(&mut self) -> ! {
        if let Some(backlight_controller) = &mut self.backlight_controller {
            backlight_controller.set_min_brightness(1);
            backlight_controller.set_brightness(100);
        }

        loop {}
    }
}
