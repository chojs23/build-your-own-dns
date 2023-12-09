#[derive(Debug)]
pub struct DnsHeader {
    /// Packet Identifier (ID) - 16 bits
    /// A random ID assigned to query packets. Response packets must reply with the same ID.
    pub id: u16,

    /// Query/Response Indicator (QR) - 1 bit
    /// 1 for a response packet, 0 for a query packet.
    pub qr: u8,

    /// Operation Code (OPCODE) - 4 bits
    /// 0 for a standard query, 1 for an inverse query, 2 for a server status request, 3-15 reserved for future use.
    pub opcode: u8,

    /// Authoritative Answer (AA) - 1 bit
    /// 1 if the responding server is an authority for the domain name in question, 0 otherwise.
    pub aa: u8,

    /// Truncation (TC) - 1 bit
    /// 1 if the response was truncated due to the packet being too large for the transport protocol, 0 otherwise.
    /// For DNS over UDP, this is always 0.
    pub tc: u8,

    /// Recursion Desired (RD) - 1 bit
    /// 1 if the client wants the server to recursively resolve the domain name in question, 0 otherwise.
    pub rd: u8,

    /// Recursion Available (RA) - 1 bit
    /// 1 if the server supports recursive resolution, 0 otherwise.
    pub ra: u8,

    /// Reserved (Z) - 3 bits
    /// Used by DNSSEC, always 0 otherwise.
    pub z: u8,

    /// Response Code (RCODE) - 4 bits
    /// - 0 for no error
    /// - 1 for a format error
    /// - 2 for a server failure
    /// - 3 for a name error
    /// - 4 for a not implemented error
    /// - 5 for a refused error
    /// - 6-15 reserved for future use.
    pub rcode: u8,

    /// Question Count (QDCOUNT) - 16 bits
    /// The number of questions in the question section of the packet.
    pub qdcount: u16,

    /// Answer Record Count (ANCOUNT) - 16 bits
    /// The number of resource records in the answer section of the packet.
    pub ancount: u16,

    /// Authority Record Count (NSCOUNT) - 16 bits
    /// The number of resource records in the authority section of the packet.
    pub nscount: u16,

    /// Additional Record Count (ARCOUNT) - 16 bits
    /// The number of resource records in the additional section of the packet.
    pub arcount: u16,
}

impl DnsHeader {
    pub fn new(id: u16, response: bool, qdcount: u16, ancount: u16) -> Self {
        DnsHeader {
            id,
            qr: if response { 1 } else { 0 },
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 1,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount,
            ancount,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn parse(bytes: &[u8]) -> Self {
        let id = u16::from_be_bytes([bytes[0], bytes[1]]);
        let qr = bytes[2] >> 7;
        let opcode = (bytes[2] >> 3) & 0b0000_1111;
        let aa = (bytes[2] >> 2) & 0b0000_0001;
        let tc = (bytes[2] >> 1) & 0b0000_0001;
        let rd = bytes[2] & 0b0000_0001;
        let ra = bytes[3] >> 7;
        let z = (bytes[3] >> 4) & 0b0000_0111;
        let rcode = bytes[3] & 0b0000_1111;
        let qdcount = u16::from_be_bytes([bytes[4], bytes[5]]);
        let ancount = u16::from_be_bytes([bytes[6], bytes[7]]);
        let nscount = u16::from_be_bytes([bytes[8], bytes[9]]);
        let arcount = u16::from_be_bytes([bytes[10], bytes[11]]);
        DnsHeader {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bytes = [0; 12];
        bytes[0..2].copy_from_slice(&self.id.to_be_bytes());
        bytes[2] = (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd;
        bytes[3] = (self.ra << 7) | (self.z << 4) | self.rcode;
        bytes[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        bytes[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        bytes
    }
}
