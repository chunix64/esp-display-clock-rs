#![cfg(feature = "desktop")]

use ratatui::DefaultTerminal;
use ratatui::Frame;
use std::{thread, time::Duration};

use crate::{models::clock::Clock, ui::screens::default::DefaultUI};

pub fn ui_task(terminal: &mut DefaultTerminal, clock: &Clock) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render_ui(frame, clock))?;
        thread::sleep(Duration::from_millis(1000));
    }
}

// Add choosing layout/ui/theme logic later
fn render_ui(frame: &mut Frame, clock: &Clock) {
    let area = frame.area();

    frame.render_widget(DefaultUI::new(clock), area);
}
