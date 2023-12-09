mod protocol;

use std::net::UdpSocket;

use crate::protocol::{answer::DnsAnswer, Dns};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);
                let dns = Dns::parse(&buf[0..size]);

                println!("Received request: {:?}", dns);

                let response = Dns::new(dns.header.id, true, dns.header.qdcount, 1);

                println!("Sending response: {:?}", response);

                let response_bytes = response.response();

                udp_socket
                    .send_to(&response_bytes, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
