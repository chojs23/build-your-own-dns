use self::answer::DnsAnswer;

pub mod answer;
pub mod header;
pub mod question;

#[derive(Debug)]
pub struct Dns {
    pub header: header::DnsHeader,
    pub questions: Vec<question::DnsQuestion>,
    pub answers: Vec<answer::DnsAnswer>,
}

impl Dns {
    pub fn new(id: u16, response: bool, qdcount: u16, ancount: u16) -> Self {
        Dns {
            header: header::DnsHeader::new(id, response, qdcount, ancount),
            questions: Vec::with_capacity(qdcount as usize),
            answers: Vec::with_capacity(ancount as usize),
        }
    }

    #[allow(dead_code)]
    pub fn add_question(&mut self, question: question::DnsQuestion) {
        self.questions.push(question);
    }

    pub fn add_answer(&mut self, answer: answer::DnsAnswer) {
        self.answers.push(answer);
    }

    pub fn parse(bytes: &[u8]) -> Self {
        let header = header::DnsHeader::parse(bytes);

        let mut dns = Dns {
            header,
            questions: Vec::new(),
            answers: Vec::new(),
        };

        let mut i = 12;

        if dns.header.qdcount > 0 {
            i = dns.parse_questions(bytes, i);
        }
        if dns.header.ancount > 0 {
            _ = dns.parse_answers(bytes, i);
        }

        dns
    }

    pub fn response(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header = self.header.to_bytes();
        bytes.extend_from_slice(&header);

        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }

        for _question in &self.questions {
            let answer: DnsAnswer =
                DnsAnswer::new("codecrafters.io".to_string(), 1, 1, 60, 4, vec![8, 8, 8, 8]);
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
        for _ in 0..1 {
            let (answer, next) = answer::DnsAnswer::parse(bytes, i, self.header.ancount);
            self.answers.push(answer);
            i = next;
        }
        i
    }
}
