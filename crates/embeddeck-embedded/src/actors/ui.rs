use embassy_time::Delay;
use embeddeck_ui::screens::default::DefaultScreen;
use embedded_hal_async::delay::DelayNs;
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::{Frame, Terminal};

use crate::{
    hardware::display::display_controller::DisplayController, models::clock::EmbeddedClock,
};

#[embassy_executor::task]
pub async fn ui_task(display: &'static mut DisplayController, clock: &'static EmbeddedClock) {
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
fn render_ui(frame: &mut Frame, clock: &'static EmbeddedClock) {
    let area = frame.area();

    frame.render_widget(DefaultScreen::new(clock), area);
}
