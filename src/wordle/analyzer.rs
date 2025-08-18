use crate::wordle::core::load_words;
use std::collections::HashSet;

const WORD_LEN: usize = 5;
const NUM_PATTERNS: usize = 243; // 3^5
const PID_ALL_GREEN: usize = 242; // [2,2,2,2,2] in base-3

/// Analyzes the word list to find the best starting words
pub struct StartingWordAnalyzer {
    words: Vec<String>,
}

impl StartingWordAnalyzer {
    pub fn new() -> Result<Self, std::io::Error> {
        let words = load_words()?;
        Ok(Self { words })
    }

    pub fn from_words(words: Vec<String>) -> Self {
        Self { words }
    }

    /// Analyze all words and return the top N best starting words
    pub fn find_best_starters(&self, top_n: usize) -> Vec<(String, f64, f64, f64)> {
        let mut scores: Vec<(String, f64, f64, f64)> = Vec::new();
        let total = self.words.len();
        let mut processed = 0;

        // Consider all words as potential first guesses
        for (i, guess) in self.words.iter().enumerate() {
            let (entropy, exp_remaining, p_win) = self.score_as_opener(guess);
            scores.push((guess.clone(), entropy, exp_remaining, p_win));

            processed += 1;
            if processed % 100 == 0 {
                eprint!(
                    "\rAnalyzing... {}/{} ({:.1}%)",
                    processed,
                    total,
                    (processed as f64 / total as f64) * 100.0
                );
            }
        }
        eprintln!("\rAnalyzing... {}/{} (100.0%)   ", total, total);

        // Sort by entropy (descending), then by expected remaining (ascending)
        scores.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap()
                .then(a.2.partial_cmp(&b.2).unwrap())
                .then(b.3.partial_cmp(&a.3).unwrap())
        });

        scores.into_iter().take(top_n).collect()
    }

    /// Calculate the entropy of a word as an opening guess
    fn score_as_opener(&self, guess: &str) -> (f64, f64, f64) {
        let mut hist = [0usize; NUM_PATTERNS];

        // Calculate feedback distribution
        for solution in &self.words {
            let pid = feedback_id(guess, solution);
            hist[pid] += 1;
        }

        // Calculate entropy and expected remaining
        let n = self.words.len() as f64;
        let mut entropy = 0.0;
        let mut sum_sq: usize = 0;

        for &count in &hist {
            if count > 0 {
                let p = count as f64 / n;
                entropy -= p * p.log2(); // Shannon entropy
                sum_sq += count * count;
            }
        }

        let exp_remaining = sum_sq as f64 / n;
        let p_win = hist[PID_ALL_GREEN] as f64 / n;

        (entropy, exp_remaining, p_win)
    }

    /// Find words with the most unique letter positions (good for gathering info)
    pub fn find_diverse_starters(&self, top_n: usize) -> Vec<(String, usize, f64)> {
        let mut scores: Vec<(String, usize, f64)> = Vec::new();

        for word in &self.words {
            let chars: HashSet<char> = word.chars().collect();
            let unique_count = chars.len();

            // Calculate position diversity bonus
            let mut position_score = 0.0;
            let letter_freqs = self.calculate_letter_frequencies();

            for (i, ch) in word.chars().enumerate() {
                if let Some(&freq) = letter_freqs.get(&(ch, i)) {
                    position_score += freq;
                }
            }

            scores.push((word.clone(), unique_count, position_score));
        }

        // Sort by unique letters (desc), then by position score (desc)
        scores.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.partial_cmp(&a.2).unwrap()));

        scores.into_iter().take(top_n).collect()
    }

    fn calculate_letter_frequencies(&self) -> std::collections::HashMap<(char, usize), f64> {
        use std::collections::HashMap;
        let mut freq_map: HashMap<(char, usize), usize> = HashMap::new();
        let total = self.words.len() as f64;

        for word in &self.words {
            for (i, ch) in word.chars().enumerate() {
                *freq_map.entry((ch, i)).or_insert(0) += 1;
            }
        }

        freq_map
            .into_iter()
            .map(|(key, count)| (key, count as f64 / total))
            .collect()
    }

    /// Analyze a specific word as an opener
    pub fn analyze_word(&self, word: &str) -> Option<(f64, f64, f64, Vec<(usize, usize)>)> {
        if word.len() != 5 {
            return None;
        }

        let word_upper = word.to_uppercase();
        if !self.words.contains(&word_upper) {
            eprintln!("Warning: '{}' is not in the word list", word);
        }

        let (entropy, exp_remaining, p_win) = self.score_as_opener(&word_upper);

        // Get pattern distribution
        let mut pattern_dist = Vec::new();
        let mut hist = [0usize; NUM_PATTERNS];

        for solution in &self.words {
            let pid = feedback_id(&word_upper, solution);
            hist[pid] += 1;
        }

        for (pid, &count) in hist.iter().enumerate() {
            if count > 0 {
                pattern_dist.push((pid, count));
            }
        }
        pattern_dist.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        Some((entropy, exp_remaining, p_win, pattern_dist))
    }
}

/// Return Wordle feedback encoded in base-3 (same as in solver.rs)
#[inline]
fn feedback_id(guess: &str, solution: &str) -> usize {
    let g = guess.as_bytes();
    let s = solution.as_bytes();

    if g.len() != WORD_LEN || s.len() != WORD_LEN {
        return 0;
    }

    let mut trits = [0u8; WORD_LEN];
    let mut counts = [0i8; 26];

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

/// Decode a pattern ID back to a visual string
pub fn pattern_to_string(pid: usize) -> String {
    let mut result = String::new();
    let mut p = pid;

    for _ in 0..5 {
        let trit = p % 3;
        result.push(match trit {
            0 => '⬜',
            1 => '🟨',
            2 => '🟩',
            _ => '?',
        });
        p /= 3;
    }

    result.chars().rev().collect()
}
