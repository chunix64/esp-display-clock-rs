use embassy_time::Ticker;

use crate::models::clock::Clock;

#[embassy_executor::task]
pub async fn timer_task(clock: &'static mut Clock) {
    let mut ticker = Ticker::every(embassy_time::Duration::from_secs(1));
    loop {
        ticker.next().await;
        clock.set_time(clock.now() + chrono::Duration::seconds(1));
    }
}
