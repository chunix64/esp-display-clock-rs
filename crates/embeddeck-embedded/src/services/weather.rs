use anyhow::Result;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use log::{info, warn};
use nanofish::{HttpHeader, ResponseBody, SmallHttpClient, mime_types};
use static_cell::StaticCell;

const WEATHER_URL: &str =
    "http://api.open-meteo.com/v1/forecast?latitude=1.35&longitude=103.81&hourly=temperature_2m";

static RESPONSE_BUFFER: StaticCell<[u8; 512]> = StaticCell::new();

#[embassy_executor::task]
pub async fn weather_task(network_stack: embassy_net::Stack<'static>) {
    network_stack.wait_config_up().await;

    let interval = 60000; // 1 minute

    let client = SmallHttpClient::new(&network_stack);
    let response_buffer = RESPONSE_BUFFER.init([0u8; 512]);
    let mut headers = [
        HttpHeader::user_agent("Embeddeck/0.1.0"),
        HttpHeader::content_type(mime_types::JSON),
    ];

    loop {
        if let Err(error) = fetch_weather(&client, &mut headers, response_buffer).await {
            warn!("[WEATHER] Task failed: {:?}", error);
        };
        Delay.delay_ms(interval).await;
    }
}

async fn fetch_weather(
    client: &SmallHttpClient<'_>,
    headers: &mut [HttpHeader<'_>],
    response_buffer: &mut [u8],
) -> Result<()> {
    let (response, _bytes_read) = client
        .get(WEATHER_URL, headers, response_buffer)
        .await
        .map_err(|e| anyhow::anyhow!("[WEATHER] Http fetch failed: {:?}", e))?;

    match &response.body {
        ResponseBody::Text(json) => {
            info!("[WEATHER] JSON: {:?}", json);
        }
        ResponseBody::Binary(binary) => {
            warn!("[WEATHER] Unknown response binary: {:?}", binary);
        }
        ResponseBody::Empty => {
            warn!("[WEATHER] Fetch response is empty");
        }
    }

    Ok(())
}
