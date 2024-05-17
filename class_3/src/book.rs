pub struct Book {
    pub name: String,
    pub status: bool,
    pub category: String,
}

impl Book {
    pub fn is_available(&self) -> bool {
        self.status
    }
}