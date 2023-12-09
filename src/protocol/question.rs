#[derive(Debug)]
pub struct DNSQuestion {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl DNSQuestion {
    pub fn parse(bytes: &[u8], start: usize, qdcount: u16) -> (Self, usize) {
        let mut qname = String::new();
        let mut i = start;
        loop {
            let label_len = bytes[i] as usize;
            println!("label_len: {}", label_len);
            if label_len == 0 {
                break;
            }
            if !qname.is_empty() {
                qname.push('.');
            }
            qname.push_str(
                String::from_utf8_lossy(&bytes[i + 1..i + 1 + label_len])
                    .to_lowercase()
                    .as_ref(),
            );
            i += label_len + 1;
        }
        i += 1;
        let qtype = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
        i += 2;
        let qclass = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
        i += 2;
        let question = DNSQuestion {
            qname,
            qtype,
            qclass,
        };
        if qdcount == 1 {
            (question, i)
        } else {
            let (next_question, next_i) = DNSQuestion::parse(bytes, i, qdcount - 1);
            (next_question, next_i)
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in self.qname.split('.') {
            bytes.push(label.len() as u8);
            bytes.extend_from_slice(label.as_bytes());
        }
        bytes.push(0);
        bytes.push((self.qtype >> 8) as u8);
        bytes.push(self.qtype as u8);
        bytes.push((self.qclass >> 8) as u8);
        bytes.push(self.qclass as u8);
        bytes
    }
}
