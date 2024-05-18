pub struct Character {
    name: String,
    score: u32,
    level: String,
}

impl Character {
    pub fn new(n: String, s: u32, l: String) -> Self {
        Character {
            name: n,
            score: s,
            level: l,
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn update_score(&mut self) -> u32 {
        self.score += 1;
        self.score
    }
}