use crate::wordle::types::GameState;
use std::collections::HashSet;

pub trait SolverStrategy {
    fn suggest_guess(&self, state: &GameState, all_words: &[String]) -> String;
}

pub struct SimpleSolver;
pub struct EntropyMaximizer;
pub struct FrequencyAnalyzer;

impl SolverStrategy for SimpleSolver {
    fn suggest_guess(&self, state: &GameState, _all_words: &[String]) -> String {
        if state.attempt_count == 0 {
            return "CRANE".to_string();
        }
        if state.candidates.len() == 1 {
            return state.candidates[0].clone();
        }
        if state.candidates.len() <= 20 {
            return state
                .candidates
                .first()
                .cloned()
                .unwrap_or_else(|| "SLATE".to_string());
        } else {
            match state.attempt_count {
                1 => "SLATE".to_string(),
                2 => "MOIST".to_string(),
                _ => state
                    .candidates
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "PRINT".to_string()),
            }
        }
    }
}

impl SolverStrategy for EntropyMaximizer {
    fn suggest_guess(&self, state: &GameState, all_words: &[String]) -> String {
        // Base cases
        if state.attempt_count == 0 {
            return "SALET".to_string(); // strong opener; adjust to your list
        }
        if state.candidates.len() == 1 {
            return state.candidates[0].clone();
        }
        if state.candidates.len() <= 2 {
            return state.candidates[0].clone();
        }

        // Always consider every candidate (ensures solvability-now choices are seen).
        let cand_set: HashSet<&str> = state.candidates.iter().map(|s| s.as_str()).collect();

        // Sampling budget for *non-candidate* "probe" guesses (improves splits early).
        // We'll examine all candidates + ~N sampled non-candidates.
        let non_cand_sample_budget = match state.candidates.len() {
            n if n > 500 => 200,
            n if n > 100 => 300,
            n if n > 20 => 400,
            _ => 600,
        };

        // --- Pass 1: score all candidates
        let mut best_word = state.candidates[0].clone();
        let mut best_h = -1.0f64;
        let mut best_is_cand = true;
        let mut best_exp = f64::INFINITY;
        let mut best_pwin = -1.0f64;

        for g in &state.candidates {
            let (h, exp, pwin) = score_guess(g, &state.candidates);
            if better_guess(
                h,
                true,
                exp,
                pwin,
                best_h,
                best_is_cand,
                best_exp,
                best_pwin,
            ) {
                best_h = h;
                best_is_cand = true;
                best_exp = exp;
                best_pwin = pwin;
                best_word = g.clone();
            }
        }

        // --- Pass 2: sample extra non-candidates from the full guess list
        // Use a stride so we cover the list evenly without scanning it all.
        if non_cand_sample_budget > 0 && !all_words.is_empty() {
            // How many to *try* sampling (cap by how many non-cands exist).
            let non_cand_total = all_words.len().saturating_sub(state.candidates.len());
            let target = non_cand_sample_budget.min(non_cand_total).max(0);
            if target > 0 {
                // Ceil division for stride
                let step = ((all_words.len() + target - 1) / target).max(1);
                let mut examined = 0usize;

                for i in (0..all_words.len()).step_by(step) {
                    let g = &all_words[i];
                    if cand_set.contains(g.as_str()) {
                        continue; // already evaluated in pass 1
                    }
                    let (h, exp, pwin) = score_guess(g, &state.candidates);
                    if better_guess(
                        h,
                        false,
                        exp,
                        pwin,
                        best_h,
                        best_is_cand,
                        best_exp,
                        best_pwin,
                    ) {
                        best_h = h;
                        best_is_cand = false;
                        best_exp = exp;
                        best_pwin = pwin;
                        best_word = g.clone();
                    }
                    examined += 1;
                    if examined >= target {
                        break;
                    }
                }
            }
        }

        best_word
    }
}

impl SolverStrategy for FrequencyAnalyzer {
    fn suggest_guess(&self, state: &GameState, all_words: &[String]) -> String {
        // TODO: Implement frequency-based suggestion
        SimpleSolver.suggest_guess(state, all_words)
    }
}

pub fn create_solver(strategy: &str) -> Box<dyn SolverStrategy> {
    match strategy {
        "simple" => Box::new(SimpleSolver),
        "entropy" => Box::new(EntropyMaximizer),
        "frequency" => Box::new(FrequencyAnalyzer),
        _ => Box::new(SimpleSolver),
    }
}

// ======== Scoring helpers ========

const WORD_LEN: usize = 5;
const NUM_PATTERNS: usize = 243; // 3^5
const PID_ALL_GREEN: usize = 242; // [2,2,2,2,2] in base-3

/// Return Wordle feedback encoded in base-3 (trits):
/// 0 = gray, 1 = yellow, 2 = green  => integer in 0..=242.
/// Two-pass algorithm: mark greens, count remaining letters, then mark yellows.
#[inline]
fn feedback_id(guess: &str, solution: &str) -> usize {
    let g = guess.as_bytes();
    let s = solution.as_bytes();
    debug_assert_eq!(g.len(), WORD_LEN);
    debug_assert_eq!(s.len(), WORD_LEN);
    if g.len() != WORD_LEN || s.len() != WORD_LEN {
        return 0; // defensive; your wordlists should always be length 5
    }

    let mut trits = [0u8; WORD_LEN]; // 0=B, 1=Y, 2=G
    let mut counts = [0i8; 26]; // remaining unmatched letters in solution

    // Pass 1: greens + count non-green solution letters
    for i in 0..WORD_LEN {
        if g[i] == s[i] {
            trits[i] = 2; // green
        } else {
            let idx = (s[i].to_ascii_uppercase() - b'A') as usize;
            counts[idx] += 1;
        }
    }
    // Pass 2: yellows where counts remain
    for i in 0..WORD_LEN {
        if trits[i] == 0 {
            let idx = (g[i].to_ascii_uppercase() - b'A') as usize;
            if counts[idx] > 0 {
                trits[i] = 1; // yellow
                counts[idx] -= 1;
            }
        }
    }

    // Encode five trits in base-3
    let mut id = 0usize;
    for &t in &trits {
        id = id * 3 + t as usize;
    }
    id
}

/// Score a guess against the current candidate set.
/// Returns (entropy in bits, expected remaining |C'|, win probability).
#[inline]
fn score_guess(guess: &str, candidates: &[String]) -> (f64, f64, f64) {
    let mut hist = [0usize; NUM_PATTERNS];

    for s in candidates {
        let pid = feedback_id(guess, s);
        // SAFETY: pid is 0..=242
        hist[pid] += 1;
    }

    let n = candidates.len() as f64;
    let mut h = 0.0;
    let mut sum_sq: usize = 0;

    for &c in &hist {
        if c > 0 {
            let p = c as f64 / n;
            h -= p * p.log2(); // Shannon entropy of feedback
            sum_sq += c * c;
        }
    }
    let expect_remaining = sum_sq as f64 / n;
    let p_win = hist[PID_ALL_GREEN] as f64 / n;
    (h, expect_remaining, p_win)
}

/// Tie-break order:
/// 1) Higher entropy (more information expected)
/// 2) If equal entropy: prefer guesses that are in the candidate set
/// 3) If still equal: smaller expected remaining candidates
/// 4) If still equal: higher probability to win immediately
#[inline]
fn better_guess(
    h: f64,
    is_cand: bool,
    exp: f64,
    pwin: f64,
    best_h: f64,
    best_is_cand: bool,
    best_exp: f64,
    best_pwin: f64,
) -> bool {
    h > best_h
        || (h == best_h && is_cand && !best_is_cand)
        || (h == best_h && is_cand == best_is_cand && exp < best_exp)
        || (h == best_h && is_cand == best_is_cand && exp == best_exp && pwin > best_pwin)
}
