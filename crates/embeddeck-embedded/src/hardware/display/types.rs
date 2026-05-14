use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use esp_hal::{gpio::Output, spi::master::Spi};
use mipidsi::{Display, interface::SpiInterface, models::ST7789};

pub type DisplayModel = ST7789;

pub fn display_model() -> DisplayModel {
    ST7789
}

pub type DisplayRst = Output<'static>;
pub type DisplayInterface = SpiInterface<
    'static,
    ExclusiveDevice<Spi<'static, esp_hal::Blocking>, Output<'static>, NoDelay>,
    Output<'static>,
>;

pub type ConcreteDisplay = Display<DisplayInterface, DisplayModel, DisplayRst>;
