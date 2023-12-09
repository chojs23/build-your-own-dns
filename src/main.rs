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

                let answer: DnsAnswer =
                    DnsAnswer::new("codecrafters.io".to_string(), 1, 1, 60, 4, vec![8, 8, 8, 8]);

                println!("Answer: {:?}", answer);

                let mut response = Dns::new(Dns::parse(&buf[0..size]).header.id, true, 1, 1);
                response.add_answer(answer);

                udp_socket
                    .send_to(&response.response(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
