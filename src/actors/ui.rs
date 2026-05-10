use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use ratatui::{Terminal, backend::Backend};

pub async fn ui_task(terminal: &mut Terminal<impl Backend>, delay: &mut Delay) {
    loop {
        delay.delay_ms(500).await;
    }
}
