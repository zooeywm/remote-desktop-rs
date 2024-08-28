pub struct AppConfig {
    pub num: u64,
}

impl AppConfig {
    pub fn new(num: u64) -> Self {
        Self { num }
    }
}
