use embeddeck_core::models::clock::{Clock, ClockSource};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

use crate::widgets::digital_clock::DigitalClockWidget;

pub struct DefaultScreen<'a, S: ClockSource> {
    clock: &'a Clock<S>,
}

impl<'a, S: ClockSource> DefaultScreen<'a, S> {
    pub fn new(clock: &'a Clock<S>) -> Self {
        Self { clock }
    }
}

impl<S: ClockSource> Widget for DefaultScreen<'_, S> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Embeddeck-rs".bold());

        let block = Block::bordered()
            .title(title)
            .border_set(ratatui::symbols::border::THICK);
        let inner = block.inner(area);
        let center = inner.centered(Constraint::Fill(1), Constraint::Length(4));
        block.render(area, buf);

        let digital_clock = DigitalClockWidget::new(self.clock);
        digital_clock.render(center, buf);
    }
}
