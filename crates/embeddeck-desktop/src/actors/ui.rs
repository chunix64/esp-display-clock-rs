use crossterm::event::{self, Event};
use embeddeck_ui::screens::default::DefaultScreen;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use std::time::Duration;

use crate::models::clock::DesktopClock;

pub fn ui_task(terminal: &mut DefaultTerminal, clock: &DesktopClock) -> std::io::Result<()> {
    let tick_rate = Duration::from_millis(500);

    loop {
        terminal.draw(|frame| render_ui(frame, clock))?;

        if event::poll(tick_rate)?
            && let Event::Key(_) = event::read()?
        {
            break Ok(());
        }
    }
}

// Add choosing layout/ui/theme logic later
fn render_ui(frame: &mut Frame, clock: &DesktopClock) {
    let area = frame.area();

    frame.render_widget(DefaultScreen::new(clock), area);
}
