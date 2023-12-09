pub mod header;
pub mod question;

pub struct DNS {
    header: header::DNSHeader,
    questions: Vec<question::DNSQuestion>,
}

impl DNS {
    pub fn parse(bytes: &[u8]) -> Self {
        let header = header::DNSHeader::parse(bytes);
        let mut questions = Vec::new();
        let mut start = 12;
        for i in 0..header.qdcount {
            let (question, next) = question::DNSQuestion::parse(bytes, start, header.qdcount - i);
            questions.push(question);
            start = next;
        }
        DNS { header, questions }
    }

    pub fn response(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.to_bytes());
        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }
        bytes
    }
}
