use crate::wordle::types::GameState;

pub trait SolverStrategy {
    fn suggest_guess(&self, state: &GameState, all_words: &[String]) -> String;
}

pub struct SimpleSolver;
pub struct EntropyMaximizer;
pub struct FrequencyAnalyzer;

impl SolverStrategy for SimpleSolver {
    fn suggest_guess(&self, state: &GameState, _all_words: &[String]) -> String {
        // Simple strategy based on attempt number and candidates remaining
        if state.attempt_count == 0 {
            return "CRANE".to_string();
        }
        
        if state.candidates.len() == 1 {
            return state.candidates[0].clone();
        }
        
        if state.candidates.len() <= 20 {
            // Pick first candidate when few remain
            state.candidates.first().cloned().unwrap_or("SLATE".to_string())
        } else {
            // Use common elimination words for early guesses
            match state.attempt_count {
                1 => "SLATE".to_string(),
                2 => "MOIST".to_string(),
                _ => state.candidates.first().cloned().unwrap_or("PRINT".to_string()),
            }
        }
    }
}

impl SolverStrategy for EntropyMaximizer {
    fn suggest_guess(&self, state: &GameState, all_words: &[String]) -> String {
        use std::collections::HashMap;
        
        // For first guess, use a word with common letters
        // SALET, CRANE, SLATE are all good openers
        if state.attempt_count == 0 {
            return "SALET".to_string(); // Statistically best opener
        }
        
        // If only one candidate left, return it
        if state.candidates.len() == 1 {
            return state.candidates[0].clone();
        }
        
        // If very few candidates, just pick the first one
        if state.candidates.len() <= 2 {
            return state.candidates[0].clone();
        }
        
        // Calculate entropy for each possible guess
        let mut best_word = state.candidates[0].clone();
        let mut best_score = 0.0;
        
        // Decide which words to consider as guesses
        let (guess_pool, sample_size): (&[String], usize) = if state.candidates.len() > 100 {
            // Many candidates: sample from candidates only
            (&state.candidates, 200)
        } else if state.candidates.len() > 20 {
            // Medium pool: consider some candidates and some all_words
            (all_words, 300)
        } else {
            // Small pool: consider more words for optimal elimination
            (all_words, 500)
        };
        
        // Sample words evenly from the pool for better coverage
        let step = guess_pool.len().max(1) / sample_size.min(guess_pool.len()).max(1);
        
        for (i, guess_word) in guess_pool.iter().enumerate() {
            // Skip some words for performance (but always check candidates)
            if i % step != 0 && !state.candidates.contains(guess_word) {
                continue;
            }
            
            let score = calculate_entropy(guess_word, &state.candidates);
            
            if score > best_score {
                best_score = score;
                best_word = guess_word.clone();
            }
            
            // Early exit if we've checked enough words
            if i > sample_size {
                break;
            }
        }
        
        best_word
    }
}

// Helper function to calculate entropy for a guess
fn calculate_entropy(guess: &str, candidates: &[String]) -> f64 {
    use std::collections::HashMap;
    use crate::wordle::core::calculate_feedback;
    
    // Count how many candidates would give each feedback pattern
    let mut pattern_counts: HashMap<Vec<char>, usize> = HashMap::new();
    
    for candidate in candidates {
        let feedback = calculate_feedback(guess, candidate);
        *pattern_counts.entry(feedback).or_insert(0) += 1;
    }
    
    // Calculate entropy: sum of -p * log2(p) for each pattern
    let total = candidates.len() as f64;
    let mut entropy = 0.0;
    
    for count in pattern_counts.values() {
        if *count > 0 {
            let probability = *count as f64 / total;
            entropy -= probability * probability.log2();
        }
    }
    
    // Bonus for being in the candidate list (prefer valid solutions)
    if candidates.iter().any(|c| c == guess) {
        entropy += 0.1;
    }
    
    entropy
}

impl SolverStrategy for FrequencyAnalyzer {
    fn suggest_guess(&self, state: &GameState, _all_words: &[String]) -> String {
        // TODO: Implement frequency-based suggestion
        SimpleSolver.suggest_guess(state, _all_words)
    }
}

pub fn create_solver(strategy: &str) -> Box<dyn SolverStrategy> {
    match strategy {
        "simple" => Box::new(SimpleSolver),
        "entropy" => Box::new(EntropyMaximizer),
        "frequency" => Box::new(FrequencyAnalyzer),
        _ => Box::new(SimpleSolver), // Default to simple
    }
}
