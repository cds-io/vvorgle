use std::collections::HashSet;

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
    pub available_letters: HashSet<char>,
}

impl GameState {
    pub fn new(candidates: Vec<String>) -> Self {
        let mut available_letters = HashSet::new();
        for c in 'A'..='Z' {
            available_letters.insert(c);
        }

        GameState {
            candidates,
            attempts: Vec::new(),
            attempt_count: 0,
            available_letters,
        }
    }

    pub fn add_attempt(&mut self, guess: Guess) {
        // Update available letters based on feedback
        for (i, &feedback_char) in guess.feedback.iter().enumerate() {
            if feedback_char != 'G' && feedback_char != 'Y' {
                // Gray letter - not in the word at all
                if let Some(letter) = guess.word.chars().nth(i) {
                    self.available_letters.remove(&letter);
                }
            }
        }

        self.attempts.push(guess);
        self.attempt_count += 1;
    }

    pub fn get_available_letters_sorted(&self) -> Vec<char> {
        let mut letters: Vec<char> = self.available_letters.iter().cloned().collect();
        letters.sort();
        letters
    }
}

pub enum FeedbackType {
    Green,
    Yellow,
    Black,
}
