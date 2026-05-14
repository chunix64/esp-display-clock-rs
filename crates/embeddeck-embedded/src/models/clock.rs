use chrono::TimeZone;
use embeddeck_ui::models::clock::{Clock, ClockSource};
use esp_hal::rtc_cntl::Rtc;

pub struct RtcSource {
    pub rtc: &'static Rtc<'static>,
}

impl ClockSource for RtcSource {
    fn current_time_us(&self) -> u64 {
        self.rtc.current_time_us()
    }
}

pub type EmbeddedClock = Clock<RtcSource>;

pub trait EmbeddedClockExt {
    fn default(rtc: &'static Rtc<'static>) -> EmbeddedClock;
}

impl EmbeddedClockExt for EmbeddedClock {
    fn default(rtc: &'static Rtc<'static>) -> EmbeddedClock {
        let default_us = chrono_tz::UTC
            .with_ymd_and_hms(2026, 5, 10, 12, 0, 0)
            .unwrap()
            .timestamp_micros() as u64;
        rtc.set_current_time_us(default_us);

        EmbeddedClock::new(chrono_tz::UTC, RtcSource { rtc })
    }
}
