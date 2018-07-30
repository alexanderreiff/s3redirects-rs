//! Contains struct to parse CSV stream into redirect rules
use csv;
use redirect_rule::RedirectRule;
use std::error::Error;
use std::io::Read;

/// Represents a container for a Read struct to be parsed into a set of redirect rules
pub struct Parser {
    reader: Box<Read>,
}

impl Parser {
    /// Creates a new Parser struct from a Boxed Read struct
    pub fn new(reader: Box<Read>) -> Self {
        Self { reader }
    }

    /// Consumes the Parser instance, parses the CSV reader and, on success, returns a Vec of
    /// RedirectRule structs corresponding to the rows in the CSV stream.
    pub fn into_rules(self) -> Result<Vec<RedirectRule>, Box<Error>> {
        let mut parser = csv::Reader::from_reader(self.reader);
        let mut rules = Vec::new();

        for row in parser.deserialize() {
            let rule: RedirectRule = row?;
            rules.push(rule);
        }

        Ok(rules)
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn it_returns_set_of_rules_for_valid_csv() {
        let csv = "match_pattern,redirect_pattern
^/resources/(.+)/subs(/.*)?$,/new-resources/$1/new-sub$2
^/simple$,/short"
            .as_bytes();

        let parser = Parser::new(Box::new(csv));

        assert_eq!(
            parser.get_rules().unwrap(),
            vec![
                RedirectRule::new(
                    "^/resources/(.+)/subs(/.*)?$",
                    "/new-resources/$1/new-sub$2",
                ),
                RedirectRule::new("^/simple$", "/short"),
            ]
        );
    }

    #[test]
    fn it_returns_error_for_invalid_csv() {
        let csv = "match_pattern,redirect_pattern
^/resources/(.+)/subs(/.*)?$;/new-resources/$1/new-sub$2
^/simple$,/short"
            .as_bytes();

        let parser = Parser::new(Box::new(csv));

        assert!(parser.get_rules().is_err());
    }
}
