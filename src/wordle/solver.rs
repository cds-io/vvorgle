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

// Placeholder implementations for other strategies
impl SolverStrategy for EntropyMaximizer {
    fn suggest_guess(&self, state: &GameState, _all_words: &[String]) -> String {
        // TODO: Implement entropy-based suggestion
        SimpleSolver.suggest_guess(state, _all_words)
    }
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
