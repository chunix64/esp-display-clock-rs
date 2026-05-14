use core::net::{IpAddr, SocketAddr};

use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use esp_hal::rtc_cntl::Rtc;
use log::warn;
use sntpc::{NtpTimestampGenerator, get_time};
use sntpc_net_embassy::UdpSocketWrapper;

const NTP_SERVER: &str = "time.google.com";

#[derive(Copy, Clone)]
struct NtpTimeStamp<'a> {
    rtc: &'a Rtc<'a>,
    current_time_us: u64,
}

impl<'a> NtpTimeStamp<'a> {
    fn new(rtc: &'a Rtc<'a>, current_time_us: u64) -> Self {
        Self {
            rtc,
            current_time_us,
        }
    }
}

impl NtpTimestampGenerator for NtpTimeStamp<'_> {
    fn init(&mut self) {
        self.current_time_us = self.rtc.current_time_us();
    }

    fn timestamp_sec(&self) -> u64 {
        self.current_time_us / 1_000_000
    }

    fn timestamp_subsec_micros(&self) -> u32 {
        (self.current_time_us % 1_000_000) as u32
    }
}

#[embassy_executor::task]
pub async fn ntp_task(network_stack: embassy_net::Stack<'static>, rtc: &'static Rtc<'static>) {
    network_stack.wait_config_up().await;

    let interval = 3_600_000; // one hour

    let ntp_addresses = network_stack
        .dns_query(NTP_SERVER, embassy_net::dns::DnsQueryType::A)
        .await
        .unwrap();

    if ntp_addresses.is_empty() {
        warn!("[NTP] Can not get the ntp address!");
        return;
    }

    let mut rx_meta = [PacketMetadata::EMPTY; 16];
    let mut rx_buffer = [0; 4096];
    let mut tx_meta = [PacketMetadata::EMPTY; 16];
    let mut tx_buffer = [0; 4096];

    let mut socket = UdpSocket::new(
        network_stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );

    socket.bind(123).unwrap();

    let socket = UdpSocketWrapper::new(socket);

    let ntp_address: SocketAddr = SocketAddr::new(
        match ntp_addresses[0] {
            embassy_net::IpAddress::Ipv4(address) => IpAddr::V4(address),
            embassy_net::IpAddress::Ipv6(address) => IpAddr::V6(address),
        },
        123,
    );

    loop {
        let mut timestamp = NtpTimeStamp::new(rtc, 0);
        timestamp.init();
        let ntp_context = sntpc::NtpContext::new(timestamp);
        let result = get_time(ntp_address, &socket, ntp_context).await;

        match result {
            Ok(time) => {
                rtc.set_current_time_us(
                    (time.sec() as u64 * 1_000_000)
                        + ((time.sec_fraction() as u64 * 1_000_000) >> 32),
                );
            }
            Err(error) => {
                warn!("Failed to get ntp time: {:?}", error);
            }
        }

        Delay.delay_ms(interval).await;
    }
}
