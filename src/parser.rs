use csv;
use redirect_rule::RedirectRule;
use std::error::Error;
use std::io::Read;

pub struct Parser {
    reader: Box<Read>,
}

impl Parser {
    pub fn new(reader: Box<Read>) -> Self {
        Self { reader }
    }

    pub fn get_rules(self) -> Result<Vec<RedirectRule>, Box<Error>> {
        let mut parser = csv::Reader::from_reader(self.reader);
        let mut rules = Vec::new();

        for row in parser.deserialize() {
            let rule: RedirectRule = row?;
            rules.push(rule);
        }

        Ok(rules)
    }
}
