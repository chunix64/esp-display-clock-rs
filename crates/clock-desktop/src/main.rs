extern crate std;

mod actors;
mod models;

use crate::{
    actors::ui::ui_task,
    models::clock::{DesktopClock, DesktopClockExt},
};

fn main() -> std::io::Result<()> {
    let clock = DesktopClock::default();
    ratatui::run(|terminal| ui_task(terminal, &clock))
}
