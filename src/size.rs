pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    pub fn string(&self) -> String {
        match self {
            Self::Small => String::from("256x256"),
            Self::Medium => String::from("512x512"),
            Self::Large => String::from("1024x1024"),
        }
    }
}