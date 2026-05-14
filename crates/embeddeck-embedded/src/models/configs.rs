use esp_hal::{
    gpio::AnyPin,
    peripherals::{LEDC, LPWR, WIFI},
    spi::master::AnySpi,
};
use mipidsi::models::Model;

// Configs
pub struct AppPeripherals {
    pub ledc: LEDC<'static>,
    pub spi: AnySpi<'static>,
    pub wifi: WIFI<'static>,
    pub lpwr: LPWR<'static>,
}

pub struct DisplayPins {
    pub sck: AnyPin<'static>,
    pub mosi: AnyPin<'static>,
    pub dc: AnyPin<'static>,
    pub cs: AnyPin<'static>,
    pub rst: AnyPin<'static>,
}

pub struct BacklightConfig {
    pub pin: AnyPin<'static>,
}

pub struct DisplayConfig<M: Model> {
    pub display_model: M,
    pub display_width: u16,
    pub display_height: u16,
    pub pins: DisplayPins,
}

pub struct WifiConfig {
    pub ssid: heapless::String<32>,
    pub password: heapless::String<32>,
}
