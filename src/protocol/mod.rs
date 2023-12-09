pub mod header;
pub mod question;

pub struct Dns {
    header: header::DNSHeader,
    questions: Vec<question::DNSQuestion>,
}

impl Dns {
    pub fn parse(bytes: &[u8]) -> Self {
        let header = header::DNSHeader::parse(bytes);
        let mut questions = Vec::new();
        let mut start = 12;
        for i in 0..header.qdcount {
            let (question, next) = question::DNSQuestion::parse(bytes, start, header.qdcount - i);
            questions.push(question);
            start = next;
        }
        Self { header, questions }
    }

    pub fn response(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header = header::DNSHeader::new(self.header.id, true, self.header.qdcount).to_bytes();
        bytes.extend_from_slice(&header);
        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }
        bytes
    }
}
