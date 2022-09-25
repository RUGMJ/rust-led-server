use sacn_unofficial::packet::ACN_SDT_MULTICAST_PORT;
use sacn_unofficial::receive::SacnReceiver;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

const UNIVERSE1: u16 = 1;
const TIMEOUT: Option<Duration> = Some(Duration::from_secs(30)); // A timeout of None means blocking behaviour, some indicates the actual timeout.

fn main() {
    let args: Vec<String> = env::args().collect();

    let leds = &args[0];

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);

    let mut dmx_rcv = SacnReceiver::with_ip(addr, None).unwrap();

    dmx_rcv.listen_universes(&[UNIVERSE1]).unwrap();

    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    loop {
        // .recv(TIMEOUT) handles processing synchronised as-well as normal data.
        match dmx_rcv.recv(TIMEOUT) {
            Err(e) => {
                println!("{:?}", e);
            }
            Ok(p) => {
                let mut spi_encoded_rgb_bits = vec![];
                for n in 0..leds.trim().parse().expect("# of Leds wasn't a number.") {
                    let i = n * 3;
                    let r = p[0].values[i + 1];
                    let g = p[0].values[i + 2];
                    let b = p[0].values[i];

                    spi_encoded_rgb_bits.extend_from_slice(&encode_rgb(r, g, b));
                }
                adapter.write_encoded_rgb(&spi_encoded_rgb_bits).unwrap();
            }
        }
    }
}
