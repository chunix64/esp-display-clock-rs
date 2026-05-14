use embassy_time::Delay;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    spi::master::{AnySpi, Spi},
};
use mipidsi::{interface::SpiInterface, options::Rotation};

use crate::{
    hardware::display::types::{ConcreteDisplay, DisplayModel},
    models::configs::DisplayConfig,
};

pub struct SpiDisplayBuilder;

impl SpiDisplayBuilder {
    pub fn build(
        spi_peripheral: AnySpi<'static>,
        display_config: DisplayConfig<DisplayModel>,
        buffer: &'static mut [u8],
    ) -> ConcreteDisplay {
        let rst = Output::new(display_config.pins.rst, Level::Low, OutputConfig::default());

        let sck = Output::new(display_config.pins.sck, Level::Low, OutputConfig::default());
        let mosi = Output::new(
            display_config.pins.mosi,
            Level::Low,
            OutputConfig::default(),
        );
        let cs = Output::new(display_config.pins.cs, Level::Low, OutputConfig::default());
        let dc = Output::new(display_config.pins.dc, Level::Low, OutputConfig::default());

        let spi_config = esp_hal::spi::master::Config::default()
            .with_mode(esp_hal::spi::Mode::_3)
            .with_frequency(esp_hal::time::Rate::from_mhz(80));

        let spi = Spi::new(spi_peripheral, spi_config)
            .unwrap()
            .with_sck(sck)
            .with_mosi(mosi);

        let spi_bus = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

        let spi_device = SpiInterface::new(spi_bus, dc, buffer);

        mipidsi::Builder::new(display_config.display_model, spi_device)
            .display_size(display_config.display_width, display_config.display_height)
            .orientation(mipidsi::options::Orientation::default().rotate(Rotation::Deg0))
            .reset_pin(rst)
            .init(&mut Delay)
            .unwrap()
    }
}
