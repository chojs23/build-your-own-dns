#[derive(Debug)]
pub struct DnsQuestion {
    /// Name (QNAME) - variable length
    /// The domain name that this question pertains to.
    ///
    /// Domain name encoding:
    ///
    /// Domain names in DNS packets are encoded as a sequence of labels.
    ///
    /// Labels are encoded as <length><content>, where <length> is a single byte that specifies the length of the label, and <content> is the actual content of the label. The sequence of labels is terminated by a null byte (\x00).
    pub qname: String,

    /// Type (QTYPE) - 16 bits
    /// The type of the question. For example, A, AAAA, NS, MX, etc.
    pub qtype: u16,

    /// Class (QCLASS) - 16 bits
    /// The class of the question.
    pub qclass: u16,
}

impl DnsQuestion {
    pub fn parse(bytes: &[u8], start: usize, qdcount: u16) -> (Self, usize) {
        let mut qname = String::new();
        let mut i = start;
        loop {
            let label_len = bytes[i] as usize;
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
        let question = DnsQuestion {
            qname,
            qtype,
            qclass,
        };
        if qdcount == 1 {
            (question, i)
        } else {
            let (next_question, next_i) = DnsQuestion::parse(bytes, i, qdcount - 1);
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
