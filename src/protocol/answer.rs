#[derive(Debug)]
pub struct DnsAnswer {
    /// Name (NAME) - variable length
    /// The domain name that this resource record pertains to.
    pub name: String,
    /// Type (TYPE) - 16 bits
    /// The type of resource record. For example, A, AAAA, NS, MX, etc.
    pub rtype: u16,
    /// Class (CLASS) - 16 bits
    /// The class of resource record.
    /// IN 1 the Internet
    /// CS 2 the CSNET class (Obsolete)
    /// CH 3 the CHAOS class
    /// HS 4 Hesiod [Dyer 87]
    pub class: u16,
    /// Time to Live (TTL) - 32 bits
    /// The number of seconds that this resource record may be cached before it should be discarded.
    pub ttl: u32,
    /// Resource Data Length (RDLENGTH) - 16 bits
    /// The length of the RDATA field in bytes.
    pub rdlength: u16,
    /// Resource Data (RDATA) - variable length
    /// The data of the resource record. The format of this data varies according to the TYPE and CLASS of the resource record.
    /// For example, the if the TYPE is A and the CLASS is IN, the RDATA field is a 4 octet ARPA Internet address.
    pub rdata: Vec<u8>,
}

#[allow(dead_code)]
impl DnsAnswer {
    pub fn new(
        name: String,
        rtype: u16,
        class: u16,
        ttl: u32,
        rdlength: u16,
        rdata: Vec<u8>,
    ) -> Self {
        DnsAnswer {
            name,
            rtype,
            class,
            ttl,
            rdlength,
            rdata,
        }
    }
    pub fn parse(bytes: &[u8], start: usize, ancount: u16) -> (Self, usize) {
        let mut name = String::new();
        let mut i = start;
        loop {
            let label_len = bytes[i] as usize;
            if label_len == 0 {
                break;
            }
            if !name.is_empty() {
                name.push('.');
            }
            name.push_str(
                String::from_utf8_lossy(&bytes[i + 1..i + 1 + label_len])
                    .to_lowercase()
                    .as_ref(),
            );
            i += label_len + 1;
        }
        i += 1;
        let rtype = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
        i += 2;
        let class = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
        i += 2;
        let ttl = u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
        i += 4;
        let rdlength = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
        i += 2;
        let rdata = bytes[i..i + rdlength as usize].to_vec();
        i += rdlength as usize;
        let answer = DnsAnswer::new(name, rtype, class, ttl, rdlength, rdata);
        if ancount == 1 {
            (answer, i)
        } else {
            let (next_answer, next_i) = DnsAnswer::parse(bytes, i, ancount - 1);
            (next_answer, next_i)
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in self.name.split('.') {
            bytes.push(label.len() as u8);
            bytes.extend_from_slice(label.as_bytes());
        }
        bytes.push(0);
        bytes.push((self.rtype >> 8) as u8);
        bytes.push(self.rtype as u8);
        bytes.push((self.class >> 8) as u8);
        bytes.push(self.class as u8);
        bytes.extend_from_slice(&self.ttl.to_be_bytes());
        bytes.extend_from_slice(&self.rdlength.to_be_bytes());
        bytes.extend_from_slice(&self.rdata);
        bytes
    }
}
