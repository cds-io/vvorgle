#[derive(Debug, PartialEq, Clone)]
pub struct Guess {
    pub word: String,
    pub feedback: Vec<char>,
}

impl Guess {
    pub fn new(word: String, feedback: Vec<char>) -> Self {
        Guess { word, feedback }
    }
}

pub struct GameState {
    pub candidates: Vec<String>,
    pub attempts: Vec<Guess>,
    pub attempt_count: usize,
}

impl GameState {
    pub fn new(candidates: Vec<String>) -> Self {
        GameState {
            candidates,
            attempts: Vec::new(),
            attempt_count: 0,
        }
    }
    
    pub fn add_attempt(&mut self, guess: Guess) {
        self.attempts.push(guess);
        self.attempt_count += 1;
    }
}

pub enum FeedbackType {
    Green,
    Yellow,
    Black,
}
