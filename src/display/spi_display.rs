use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    spi::master::Spi,
};
use mipidsi::interface::SpiInterface;

use crate::config::{AppPinConfig, DisplayConfig};

pub struct SpiDisplayBuilder;

type SpiDisplay<'a, M> = mipidsi::Display<
    SpiInterface<'a, ExclusiveDevice<Spi<'a, esp_hal::Blocking>, Output<'a>, NoDelay>, Output<'a>>,
    M,
    Output<'a>,
>;

impl<'a> SpiDisplayBuilder {
    pub fn build<M>(
        app_pin_config: &mut AppPinConfig,
        display_config: DisplayConfig<M>,
        delay: &mut Delay,
        buffer: &'a mut [u8],
    ) -> SpiDisplay<'a, M>
    where
        M: mipidsi::models::Model,
        M::ColorFormat: mipidsi::interface::InterfacePixelFormat<u8>,
    {
        let rst = Output::new(
            app_pin_config.rst.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );

        let spi_device = Self::init_spi_device(app_pin_config, buffer);

        mipidsi::Builder::new(display_config.display_model, spi_device)
            .display_size(display_config.display_width, display_config.display_height)
            .reset_pin(rst)
            .init(delay)
            .unwrap()
    }

    fn init_spi_device(
        app_pin_config: &mut AppPinConfig,
        buffer: &'a mut [u8],
    ) -> SpiInterface<
        'a,
        ExclusiveDevice<Spi<'a, esp_hal::Blocking>, Output<'a>, NoDelay>,
        Output<'a>,
    > {
        let sck = Output::new(
            app_pin_config.sck.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );
        let mosi = Output::new(
            app_pin_config.mosi.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );
        let cs = Output::new(
            app_pin_config.cs.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );
        let dc = Output::new(
            app_pin_config.dc.take().unwrap(),
            Level::Low,
            OutputConfig::default(),
        );

        let spi_config = esp_hal::spi::master::Config::default()
            .with_mode(esp_hal::spi::Mode::_3)
            .with_frequency(esp_hal::time::Rate::from_mhz(80));

        let spi = Spi::new(app_pin_config.spi.take().unwrap(), spi_config)
            .unwrap()
            .with_sck(sck)
            .with_mosi(mosi);

        let spi_bus = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

        SpiInterface::new(spi_bus, dc, buffer)
    }
}
