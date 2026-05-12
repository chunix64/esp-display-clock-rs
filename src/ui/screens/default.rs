use alloc::format;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::models::clock::Clock;

pub struct DefaultUI<'a> {
    clock: &'a Clock,
}

impl<'a> DefaultUI<'a> {
    pub fn new(clock: &'a Clock) -> Self {
        Self { clock }
    }
}

impl Widget for DefaultUI<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("ESP Clock".bold());
        let block = Block::bordered()
            .title(title)
            .border_set(ratatui::symbols::border::THICK);

        let clock_text = format!(
            "{:02}:{:02}:{:02}",
            self.clock.hour(),
            self.clock.minute(),
            self.clock.second()
        );

        let clock = Paragraph::new(clock_text)
            .style(Style::default().bold())
            .centered()
            .block(block);

        clock.render(area, buf);
    }
}
