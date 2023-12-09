mod protocol;

use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);
                let header = protocol::header::DNSHeader::parse(&buf);
                println!("Header: {:?}", header);
                let question = protocol::question::DNSQuestion::parse(&buf, 12, header.qdcount);
                println!("Question: {:?}", question);
                let mut response: Vec<u8> = Vec::new();
                let response_header = protocol::header::DNSHeader::new(header.id, true, 1);
                response.extend_from_slice(&response_header.to_bytes());
                response.extend_from_slice(&question.0.to_bytes());
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
