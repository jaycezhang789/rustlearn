#[allow(dead_code)]
pub struct Player {
    pub name: String,
    pub score: u32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
        }
    }

    pub fn add_score(&mut self, delta: u32) -> u32 {
        self.score = self.score.saturating_add(delta);
        self.score
    }

    pub fn apply_penalty(&mut self, penalty: u32) -> u32 {
        self.score = self.score.saturating_sub(penalty);
        self.score
    }
}

pub fn highest_scorer<'a>(players: &'a [Player]) -> Option<&'a Player> {
    let mut best: Option<&Player> = None;
    for player in players {
        match best {
            Some(current_best) if current_best.score >= player.score => {}
            _ => best = Some(player),
        }
    }
    best
}
