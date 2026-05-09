use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    spi::master::Spi,
};
use mipidsi::interface::SpiInterface;

use crate::config::{AppPins, DisplayConfig};

pub struct SpiDisplayBuilder;

type SpiDisplay<'a, M> = mipidsi::Display<
    SpiInterface<'a, ExclusiveDevice<Spi<'a, esp_hal::Blocking>, Output<'a>, NoDelay>, Output<'a>>,
    M,
    Output<'a>,
>;

impl<'a> SpiDisplayBuilder {
    pub fn build<M>(
        app_pins: &mut AppPins,
        display_config: &mut DisplayConfig<M>,
        delay: &mut Delay,
        buffer: &'a mut [u8],
    ) -> SpiDisplay<'a, M>
    where
        M: mipidsi::models::Model,
        M::ColorFormat: mipidsi::interface::InterfacePixelFormat<u8>,
    {
        let rst = Output::new(
            display_config.pins.rst.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );

        let spi_device = Self::init_spi_device(app_pins, display_config, buffer);

        mipidsi::Builder::new(display_config.display_model.take().unwrap(), spi_device)
            .display_size(display_config.display_width, display_config.display_height)
            .reset_pin(rst)
            .init(delay)
            .unwrap()
    }

    fn init_spi_device<M>(
        app_pins: &mut AppPins,
        display_config: &mut DisplayConfig<M>,
        buffer: &'a mut [u8],
    ) -> SpiInterface<
        'a,
        ExclusiveDevice<Spi<'a, esp_hal::Blocking>, Output<'a>, NoDelay>,
        Output<'a>,
    >
    where
        M: mipidsi::models::Model,
        M::ColorFormat: mipidsi::interface::InterfacePixelFormat<u8>,
    {
        let sck = Output::new(
            display_config.pins.sck.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );
        let mosi = Output::new(
            display_config.pins.mosi.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );
        let cs = Output::new(
            display_config.pins.cs.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );
        let dc = Output::new(
            display_config.pins.dc.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );

        let spi_config = esp_hal::spi::master::Config::default()
            .with_mode(esp_hal::spi::Mode::_3)
            .with_frequency(esp_hal::time::Rate::from_mhz(80));

        let spi = Spi::new(app_pins.spi.take().unwrap(), spi_config)
            .unwrap()
            .with_sck(sck)
            .with_mosi(mosi);

        let spi_bus = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

        SpiInterface::new(spi_bus, dc, buffer)
    }
}
