use csv;
use std::error::Error;
use std::io::Read;
use redirect_rule::RedirectRule;

pub struct Parser {
    reader: Box<Read>,
}

impl Parser {
    pub fn new(reader: Box<Read>) -> Self {
        Self { reader }
    }

    pub fn get_rules(&self) -> Result<Vec<RedirectRule>, Box<Error>> {
        let mut parser = csv::Reader::from_reader(self.reader);

        let rules = parser
            .deserialize()
            .map(|row| {
                let rule: RedirectRule = row?;
                rule
            })
            .collect::<Vec<RedirectRule>>();

        Ok(rules)
    }
}
