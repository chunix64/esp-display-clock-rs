use chrono::{DateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

pub trait ClockSource {
    fn current_time_us(&self) -> u64;
}

pub struct Clock<S: ClockSource> {
    time_zone: Tz,
    source: S,
}

impl<S: ClockSource> Clock<S> {
    pub fn new(time_zone: Tz, source: S) -> Self {
        Self { time_zone, source }
    }

    fn now(&self) -> DateTime<Tz> {
        let us = self.source.current_time_us() as i64;
        Utc.timestamp_micros(us)
            .single()
            .expect("RTC returned invalid timestamp")
            .with_timezone(&self.time_zone)
    }

    pub fn hour(&self) -> u32 {
        self.now().hour()
    }

    pub fn minute(&self) -> u32 {
        self.now().minute()
    }

    pub fn second(&self) -> u32 {
        self.now().second()
    }
}
