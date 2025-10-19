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

pub fn find_player<'a>(players: &'a [Player], name: &str) -> Option<&'a Player> {
    players.iter().find(|player| player.name == name)
}

pub fn award_bonus(players: &mut [Player], min_score: u32, bonus: u32) -> usize {
    let mut rewarded = 0;
    for player in players.iter_mut() {
        if player.score >= min_score {
            player.add_score(bonus);
            rewarded += 1;
        }
    }
    rewarded
}

pub fn average_score(players: &[Player]) -> Option<f64> {
    if players.is_empty() {
        return None;
    }
    let total: u64 = players.iter().map(|player| player.score as u64).sum();
    Some(total as f64 / players.len() as f64)
}

pub fn top_n_players<'a>(players: &'a [Player], n: usize) -> Vec<&'a Player> {
    let mut refs: Vec<&Player> = players.iter().collect();
    refs.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.name.cmp(&b.name)));
    if n < refs.len() {
        refs.truncate(n);
    }
    refs
}
