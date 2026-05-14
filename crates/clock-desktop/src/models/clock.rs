use clock_ui::models::clock::{Clock, ClockSource};

pub struct SystemClock;

impl ClockSource for SystemClock {
    fn current_time_us(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64
    }
}

pub type DesktopClock = Clock<SystemClock>;

pub trait DesktopClockExt {
    fn default() -> DesktopClock;
}

impl DesktopClockExt for DesktopClock {
    fn default() -> DesktopClock {
        DesktopClock::new(chrono_tz::UTC, SystemClock)
    }
}
