#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RedirectRule {
    match_pattern: String,
    redirect_pattern: String,
}

impl RedirectRule {
    #[cfg(test)]
    pub fn new(match_pattern: &str, redirect_pattern: &str) -> Self {
        Self {
            match_pattern: match_pattern.to_string(),
            redirect_pattern: redirect_pattern.to_string(),
        }
    }

    pub fn to_conf(&self) -> String {
        let mut conf = String::new();
        conf.push_str(&format!("location ~* {} {{\n", self.match_pattern));
        conf.push_str(&format!("    return 301 {};\n", self.redirect_pattern));
        conf.push_str("}\n");
        conf
    }
}

pub fn build_conf(rules: &[RedirectRule]) -> String {
    rules
        .iter()
        .map(|r| r.to_conf())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod redirect_rule_tests {
    use super::*;

    #[test]
    fn it_generates_conf_string_for_a_rule() {
        let rule = RedirectRule::new(
            "^/resources/(.+)/subs(/.*)?$",
            "/new-resources/$1/new-sub$2",
        );

        assert_eq!(
            rule.to_conf(),
            "location ~* ^/resources/(.+)/subs(/.*)?$ {
    return 301 /new-resources/$1/new-sub$2;
}
"
                .to_string()
        );
    }

    #[test]
    fn it_joins_conf_entries_from_multiple_rules() {
        let rules = vec![
            RedirectRule::new(
                "^/resources/(.+)/subs(/.*)?$",
                "/new-resources/$1/new-sub$2",
            ),
            RedirectRule::new("^/simple$", "/short"),
        ];

        assert_eq!(
            build_conf(&rules),
            "location ~* ^/resources/(.+)/subs(/.*)?$ {
    return 301 /new-resources/$1/new-sub$2;
}

location ~* ^/simple$ {
    return 301 /short;
}
"
                .to_string()
        );
    }
}
