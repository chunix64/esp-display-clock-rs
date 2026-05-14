#[embassy_executor::task]
pub async fn net_runner_task(
    mut runner: embassy_net::Runner<'static, esp_radio::wifi::Interface<'static>>,
) {
    runner.run().await;
}

#[embassy_executor::task]
pub async fn net_monitor_task(network_stack: embassy_net::Stack<'static>) {
    network_stack.wait_config_up().await;
    if let Some(config) = network_stack.config_v4() {
        log::info!("[NET] IP: {}", config.address);
    }
}
