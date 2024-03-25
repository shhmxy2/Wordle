use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub total_rounds: i32,
    #[serde(default)]
    pub games: Vec<Games>,
}

#[derive(Serialize, Deserialize)]
pub struct Games {
    #[serde(default)]
    pub answer: String,
    #[serde(default)]
    pub guesses: Vec<String>,
}

impl Games {
    pub fn new() -> Self {
        let answer = String::new();
        let guesses: Vec<String> = Vec::new();
        Self { answer, guesses }
    }
}

impl State {
    pub fn new() -> Self {
        let total_rounds = 0;
        let mut games: Vec<Games> = Vec::new();
        let a = Games::new();
        games.push(a);
        Self {
            total_rounds,
            games,
        }
    }
}
