pub enum ResponseFormat {
    Url,
    Json,
}

impl ResponseFormat {
    pub fn string(&self) -> String {
        match self {
            Self::Url => String::from("url"),
            Self::Json => String::from("b64_json"),
        }
    }
}