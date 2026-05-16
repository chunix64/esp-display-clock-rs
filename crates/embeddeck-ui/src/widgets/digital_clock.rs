use alloc::{format, vec};
use embeddeck_core::models::clock::{Clock, ClockSource};
use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::Line, widgets::Widget};
use tui_big_text::BigText;

pub struct DigitalClockWidget<'a, S: ClockSource> {
    clock: &'a Clock<S>,
}

impl<'a, S: ClockSource> DigitalClockWidget<'a, S> {
    pub fn new(clock: &'a Clock<S>) -> Self {
        Self { clock }
    }
}

impl<S: ClockSource> Widget for DigitalClockWidget<'_, S> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let clock_text = Line::from(format!(
            "{:02}:{:02}:{:02}",
            self.clock.hour(),
            self.clock.minute(),
            self.clock.second()
        ));

        let clock = BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Quadrant)
            .style(Style::new().white())
            .lines(vec![clock_text])
            .centered()
            .build();

        clock.render(area, buf);
    }
}
