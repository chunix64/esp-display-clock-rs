use ratatui::{Terminal, backend::Backend};

use crate::{models::clock::Clock, ui::screens::default::DefaultUI};

pub struct UIActor<'a, B: Backend> {
    terminal: Terminal<B>,
    clock: &'a Clock,
}

impl<'a, B> UIActor<'a, B>
where
    B: Backend,
{
    pub fn new(terminal: Terminal<B>, clock: &'a Clock) -> Self {
        Self { terminal, clock }
    }

    pub fn run(&mut self) {
        self.draw();
    }

    fn draw(&mut self) {
        self.terminal
            .draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();

                DefaultUI::draw(area, buf, self.clock);
            })
            .unwrap();
    }
}
