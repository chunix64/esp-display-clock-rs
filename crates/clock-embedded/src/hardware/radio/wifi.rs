use alloc::string::ToString;
use embassy_net::StackResources;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use esp_hal::rng::Rng;
use esp_radio::wifi::{Config, PowerSaveMode, WifiController, sta::StationConfig};
use log::{info, warn};
use static_cell::StaticCell;

use crate::models::configs::WifiConfig;

static NET_STACK_RESOURCES: StaticCell<StackResources<8>> = StaticCell::new();

#[embassy_executor::task]
pub async fn wifi_task(
    wifi_controller: &'static mut WifiController<'static>,
    wifi_config: WifiConfig,
) {
    wifi_handle_connection(wifi_controller, wifi_config).await;
}

async fn wifi_handle_connection(
    wifi_controller: &mut WifiController<'static>,
    wifi_config: WifiConfig,
) {
    let interval = 5000;

    let client_config = Config::Station(
        StationConfig::default()
            .with_ssid(wifi_config.ssid.as_str())
            .with_password(wifi_config.password.to_string()),
    );

    if wifi_controller.set_config(&client_config).is_err() {
        warn!("[WIFI] Failed to set WiFi config!");
        return;
    };

    if let Err(error) = wifi_controller.set_power_saving(PowerSaveMode::Minimum) {
        warn!("[WIFI] Can not set power saving mode: {:?}", error);
    }

    loop {
        if wifi_controller.is_connected() {
            let _ = wifi_controller.wait_for_disconnect_async().await;
            Delay.delay_ms(interval).await;
        } else {
            match wifi_controller.connect_async().await {
                Ok(_) => {
                    info!("[WIFI] WiFi connected to {}", wifi_config.ssid);
                }
                Err(error) => {
                    warn!(
                        "[WIFI] WiFi cannot connect to {}, retry! Error: {:?}",
                        wifi_config.ssid, error
                    );
                    Delay.delay_ms(interval).await;
                }
            }
        }
    }
}

pub fn init_network_stack<'a>(
    station: esp_radio::wifi::Interface<'a>,
) -> (
    embassy_net::Stack<'a>,
    embassy_net::Runner<'a, esp_radio::wifi::Interface<'a>>,
) {
    let rng = Rng::new();
    let stack_resources = NET_STACK_RESOURCES
        .uninit()
        .write(StackResources::<8>::new());

    let embassy_net_config = embassy_net::Config::dhcpv4(Default::default());
    let embassy_net_seed = (rng.random() as u64) << 32 | rng.random() as u64;
    embassy_net::new(
        station,
        embassy_net_config,
        stack_resources,
        embassy_net_seed,
    )
}
