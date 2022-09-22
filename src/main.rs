use sacn_unofficial::packet::ACN_SDT_MULTICAST_PORT;
use sacn_unofficial::receive::SacnReceiver;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use colored::*;

const UNIVERSE1: u16 = 1;
const TIMEOUT: Option<Duration> = Some(Duration::from_secs(30)); // A timeout of None means blocking behaviour, some indicates the actual timeout.

fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);

    let mut dmx_rcv = SacnReceiver::with_ip(addr, None).unwrap();

    dmx_rcv.listen_universes(&[UNIVERSE1]).unwrap();

    // b, r, g

    loop {
        // .recv(TIMEOUT) handles processing synchronised as-well as normal data.
        match dmx_rcv.recv(TIMEOUT) {
            Err(e) => {
                // Print out the error.
                println!("{:?}", e);
            }
            Ok(p) => {
                print!("{}", "\n");
                // Print out the data.
                // println!("{:?}", p);
                // TODO: Add logic to convert [R1, G1, B1, R2, G2, B2 ...] into [(R, G, B), (R, G, B)]
                for n in 0..100 {
                    let i = n * 3;
                    // println!("{:?}", p[0].values[i]);
                    let r = p[0].values[i + 1];
                    let g = p[0].values[i + 2];
                    let b = p[0].values[i];
                    print!("{}", "■".truecolor(r, g, b));
                }
                // TODO: Install https://crates.io/crates/ws2818-rgb-led-spi-driver and get it working so that we can actually display the dmx data on the led strip
                // TODO: Minimise computations to as little as possible to enchance performence
            }
        }
    }
}
