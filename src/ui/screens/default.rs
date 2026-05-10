use alloc::format;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::models::clock::Clock;

pub struct DefaultUI;

impl DefaultUI {
    pub fn draw(area: Rect, buf: &mut Buffer, clock: &Clock) {
        let title = Line::from("ESP Clock".bold());
        let block = Block::bordered()
            .title(title)
            .border_set(ratatui::symbols::border::THICK);

        let clock_text = format!(
            "{:02}:{:02}:{:02}",
            clock.hour(),
            clock.minute(),
            clock.second()
        );

        let clock = Paragraph::new(clock_text)
            .style(Style::default().bold())
            .centered()
            .block(block);

        clock.render(area, buf);
    }
}
