use crate::wordle::types::Guess;

fn get_positions(vec: &Vec<char>, c: char, word: &String) -> Vec<(usize, char)> {
    vec.iter()
        .enumerate()
        .filter_map(|(i, &x)| {
            if x == c {
                Some((i, word.chars().nth(i).unwrap()))
            } else {
                None
            }
        })
        .collect()
}

pub fn filter_by_green(words: &mut Vec<String>, guess: &Guess) {
    let greens = get_positions(&guess.feedback, 'G', &guess.word);
    words.retain(|word| {
        greens
            .iter()
            .all(|&(i, c)| word.chars().nth(i).unwrap() == c)
    });
}

pub fn filter_by_yellow(words: &mut Vec<String>, guess: &Guess) {
    let yellows = get_positions(&guess.feedback, 'Y', &guess.word);
    words.retain(|word| {
        yellows.iter().all(|&(i, c)| {
            // Word must contain the letter
            word.contains(c) &&
            // But NOT at the guessed position
            word.chars().nth(i).unwrap() != c
        })
    });
}

pub fn filter_by_black(words: &mut Vec<String>, guess: &Guess) {
    let blacks = get_positions(&guess.feedback, 'B', &guess.word);

    // Get unique black letters that are NOT also green or yellow
    let green_yellow_letters: Vec<char> = guess
        .feedback
        .iter()
        .enumerate()
        .filter_map(|(i, &fb)| {
            if fb == 'G' || fb == 'Y' {
                Some(guess.word.chars().nth(i).unwrap())
            } else {
                None
            }
        })
        .collect();

    words.retain(|word| {
        blacks.iter().all(|&(_, c)| {
            // If this letter also appears as green/yellow, don't filter based on it
            if green_yellow_letters.contains(&c) {
                true
            } else {
                // Black letter should NOT exist in the word at all
                !word.contains(c)
            }
        })
    });
}

pub fn filter_words(words: &mut Vec<String>, guess: &Guess) {
    filter_by_green(words, guess);
    filter_by_yellow(words, guess);
    filter_by_black(words, guess);
}

pub fn calculate_feedback(guess: &str, solution: &str) -> Vec<char> {
    let guess_chars: Vec<char> = guess.chars().collect();
    let solution_chars: Vec<char> = solution.chars().collect();
    let mut feedback = vec!['B'; 5];
    let mut solution_used = vec![false; 5];

    // First pass: mark greens
    for i in 0..5 {
        if guess_chars[i] == solution_chars[i] {
            feedback[i] = 'G';
            solution_used[i] = true;
        }
    }

    // Second pass: mark yellows
    for i in 0..5 {
        if feedback[i] == 'B' {
            for j in 0..5 {
                if !solution_used[j] && guess_chars[i] == solution_chars[j] {
                    feedback[i] = 'Y';
                    solution_used[j] = true;
                    break;
                }
            }
        }
    }

    feedback
}

pub fn parse_input(input: &str) -> Result<Guess, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Input must be WORD FEEDBACK".to_string());
    }

    let word = parts[0].to_string().to_uppercase();
    let feedback: Vec<char> = parts[1].to_uppercase().chars().collect();

    if word.len() != 5 {
        return Err("Word must be 5 letters".to_string());
    }
    if feedback.len() != 5 || !feedback.iter().all(|&c| c == 'G' || c == 'Y' || c == 'B') {
        return Err("Feedback must be 5 characters of G, Y, or B".to_string());
    }
    Ok(Guess::new(word, feedback))
}

// Embed the word list at compile time
const WORD_LIST: &str = include_str!("../../words.txt");

pub fn load_words() -> Result<Vec<String>, std::io::Error> {
    // Parse the embedded word list
    let words: Vec<String> = WORD_LIST
        .lines()
        .map(|word| word.trim().to_uppercase())
        .filter(|word| word.len() == 5)
        .collect();

    if words.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No valid words found in embedded list",
        ));
    }

    Ok(words)
}

// Optional: Keep ability to load from file for development
#[allow(dead_code)]
pub fn load_words_from_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let words: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|word| word.to_uppercase())
        .filter(|word| word.len() == 5)
        .collect();

    Ok(words)
}
