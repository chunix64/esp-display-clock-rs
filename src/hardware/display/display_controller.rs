use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{DrawTarget, RgbColor},
};
use embedded_hal::digital::OutputPin;
use mipidsi::{
    interface::{Interface, InterfacePixelFormat},
    models::Model,
    options::{Orientation, Rotation},
};

use crate::hardware::backlight::ledc::BacklightController;

pub struct DisplayController<'a, DI, MODEL, RST>
where
    DI: Interface + 'static,
    MODEL: Model + 'static,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    RST: OutputPin + 'static,
{
    display: mipidsi::Display<DI, MODEL, RST>,
    backlight: Option<BacklightController<'a>>,
}

impl<'a, DI, MODEL, RST> DisplayController<'a, DI, MODEL, RST>
where
    DI: Interface + 'static,
    MODEL: Model + 'static,
    MODEL::ColorFormat: InterfacePixelFormat<DI::Word>,
    MODEL: Model<ColorFormat = embedded_graphics::pixelcolor::Rgb565>,
    RST: OutputPin + 'static,
{
    pub fn new(
        display: mipidsi::Display<DI, MODEL, RST>,
        backlight: Option<BacklightController<'a>>,
    ) -> Self {
        Self { display, backlight }
    }

    pub fn init(&mut self) {
        self.set_min_brightness(1);
        self.set_brightness(100);
        self.display.clear(Rgb565::BLACK).unwrap();
    }

    pub fn raw_display(&mut self) -> &mut mipidsi::Display<DI, MODEL, RST> {
        &mut self.display
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        if let Some(backlight) = &mut self.backlight {
            backlight.set_brightness(brightness);
        }
    }

    pub fn set_min_brightness(&mut self, min_brightness: u8) {
        if let Some(backlight) = &mut self.backlight {
            backlight.set_min_brightness(min_brightness);
        }
    }

    pub fn rotate_landscape(&mut self) {
        let rotated = match self.display.orientation().rotation {
            Rotation::Deg90 => Rotation::Deg270,
            _ => Rotation::Deg90,
        };

        self.display
            .set_orientation(Orientation::new().rotate(rotated))
            .unwrap();
    }
}
