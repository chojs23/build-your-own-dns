pub mod answer;
pub mod header;
pub mod question;

#[derive(Debug)]
pub struct Dns {
    header: header::DnsHeader,
    questions: Vec<question::DnsQuestion>,
    answers: Vec<answer::DnsAnswer>,
}

impl Dns {
    pub fn parse(bytes: &[u8]) -> Self {
        let header = header::DnsHeader::parse(bytes);
        let mut dns = Dns {
            header,
            questions: Vec::new(),
            answers: Vec::new(),
        };

        let mut i = 12;
        i = dns.parse_questions(bytes, i);
        _ = dns.parse_answers(bytes, i);

        dns
    }

    pub fn response(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header = header::DnsHeader::new(
            self.header.id,
            true,
            self.header.qdcount,
            self.answers.len() as u16,
        )
        .to_bytes();
        bytes.extend_from_slice(&header);
        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }
        for answer in &self.answers {
            bytes.extend_from_slice(&answer.to_bytes());
        }
        bytes
    }

    fn parse_questions(&mut self, bytes: &[u8], start: usize) -> usize {
        let mut i = start;
        for _ in 0..self.header.qdcount {
            let (question, next) = question::DnsQuestion::parse(bytes, i, self.header.qdcount);
            self.questions.push(question);
            i = next;
        }
        i
    }

    fn parse_answers(&mut self, bytes: &[u8], start: usize) -> usize {
        let mut i = start;
        for _ in 0..self.header.ancount {
            let (answer, next) = answer::DnsAnswer::parse(bytes, i, self.header.ancount);
            self.answers.push(answer);
            i = next;
        }
        i
    }
}
