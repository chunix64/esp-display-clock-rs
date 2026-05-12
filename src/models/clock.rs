use chrono::{DateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

#[cfg(feature = "embedded")]
use esp_hal::rtc_cntl::Rtc;

pub trait ClockSource {
    fn current_time_us(&self) -> u64;
}

#[cfg(feature = "embedded")]
pub struct RtcSource {
    pub rtc: &'static Rtc<'static>,
}

#[cfg(feature = "embedded")]
impl ClockSource for RtcSource {
    fn current_time_us(&self) -> u64 {
        self.rtc.current_time_us()
    }
}

#[cfg(feature = "desktop")]
pub struct SystemClock;

#[cfg(feature = "desktop")]
impl ClockSource for SystemClock {
    fn current_time_us(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64
    }
}

pub struct Clock {
    time_zone: Tz,
    #[cfg(feature = "embedded")]
    source: RtcSource,
    #[cfg(feature = "desktop")]
    source: SystemClock,
}

impl Clock {
    #[cfg(feature = "embedded")]
    pub fn new(time_zone: Tz, source: RtcSource) -> Self {
        Self { time_zone, source }
    }

    #[cfg(feature = "embedded")]
    pub fn default(rtc: &'static Rtc<'static>) -> Self {
        // Suggest: use release date for each version as default_date
        let default_us = Tz::UTC
            .with_ymd_and_hms(2026, 5, 10, 12, 0, 0)
            .unwrap()
            .timestamp_micros() as u64;
        let time_zone = chrono_tz::UTC;

        rtc.set_current_time_us(default_us);

        Self::new(time_zone, RtcSource { rtc })
    }

    #[cfg(feature = "desktop")]
    pub fn new(time_zone: Tz, source: SystemClock) -> Self {
        Self { time_zone, source }
    }

    #[cfg(feature = "desktop")]
    pub fn default() -> Self {
        Self::new(chrono_tz::UTC, SystemClock)
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
