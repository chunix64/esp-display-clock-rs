#![cfg(feature = "embedded")]

use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::{Frame, Terminal};

use crate::{
    hardware::display::display_controller::DisplayController, models::clock::Clock,
    ui::screens::default::DefaultUI,
};

#[embassy_executor::task]
pub async fn ui_task(display: &'static mut DisplayController, clock: &'static Clock) {
    display.init();
    display.rotate_landscape();

    let backend = EmbeddedBackend::new(display.raw_display(), EmbeddedBackendConfig::default());
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        let _ = terminal.draw(|frame| render_ui(frame, clock));
        Delay.delay_ms(1000).await;
    }
}

// Add choosing layout/ui/theme logic later
fn render_ui(frame: &mut Frame, clock: &'static Clock) {
    let area = frame.area();

    frame.render_widget(DefaultUI::new(clock), area);
}
