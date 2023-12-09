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
                println!("Dns: {:?}", Dns::parse(&buf[0..size]));

                let id = u16::from_be_bytes([buf[0], buf[1]]);

                let answer: DnsAnswer =
                    DnsAnswer::new("codecrafters.io".to_string(), 1, 1, 60, 4, vec![8, 8, 8, 8]);

                let mut response = Dns::new(id, true, 1, 1);
                response.add_answer(answer);

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
