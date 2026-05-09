use esp_hal::{gpio::AnyPin, peripherals::LEDC, spi::master::AnySpi};

pub struct AppPins {
    pub ledc: Option<LEDC<'static>>,
    pub spi: Option<AnySpi<'static>>,
}

pub struct DisplayPins {
    pub sck: Option<AnyPin<'static>>,
    pub mosi: Option<AnyPin<'static>>,
    pub dc: Option<AnyPin<'static>>,
    pub cs: Option<AnyPin<'static>>,
    pub rst: Option<AnyPin<'static>>,
    pub backlight: Option<AnyPin<'static>>,
}

pub struct DisplayConfig<M: mipidsi::models::Model> {
    pub display_width: u16,
    pub display_height: u16,
    pub display_model: Option<M>,
    pub pins: DisplayPins,
}
