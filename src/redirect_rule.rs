#[derive(Deserialize)]
pub struct RedirectRule {
    match_pattern: String,
    redirect_pattern: String,
}

impl RedirectRule {
    pub fn to_conf(&self) -> String {
        ""
    }
}
