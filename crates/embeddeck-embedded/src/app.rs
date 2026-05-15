use embassy_executor::Spawner;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;

use crate::actors::ui::ui_actor;
use crate::hardware::display::display_controller::DisplayController;
use crate::models::clock::EmbeddedClock;

#[embassy_executor::task]
pub async fn app_task(
    spawner: Spawner,
    display: &'static mut DisplayController,
    clock: &'static EmbeddedClock,
) {
    spawner.spawn(ui_actor(display, clock).unwrap());

    loop {
        Delay.delay_ms(1000).await;
    }
}
