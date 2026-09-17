pub struct Score {
    easy: u32,
    medium: u32,
    hard: u32,
}

impl Score {

    pub fn new() -> Self {
        Score {
            easy: 0,
            medium: 0,
            hard: 0,
        }
    }

    pub fn print_scores(&self) {
        println!("Your score is: ");
        println!("Easy: {}", self.easy);
        println!("Medium: {}", self.medium);
        println!("Hard: {}", self.hard);
    }

    pub fn update_score(&mut self, chances: u32, chances_used: u32) {
        match chances {
            10 => {
                if self.easy > chances_used || self.easy == 0 { self.easy = chances_used; }
            },
            5 => {
                if self.medium > chances_used || self.medium == 0 { self.medium = chances_used; }
            },
            3 => {
                if self.hard > chances_used || self.hard == 0 { self.hard = chances_used; }
            },
            _ => (),
        }
    }
}
