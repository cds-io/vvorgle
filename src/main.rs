#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[derive(Debug, PartialEq)]
struct Guess {
    word: String,
    feedback: Vec<char>,
}

impl Guess {
    fn new(word: String, feedback: Vec<char>) -> Self {
        Guess { word, feedback }
    }
}

fn parse_input(input: &str) -> Result<Guess, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Input must be WORD FEEDBACK".to_string());
    }

    let word = parts[0].to_string();
    let feedback: Vec<char> = parts[1].chars().collect();

    if word.len() != 5 {
        return Err("Word must be 5 letters".to_string());
    }
    if feedback.len() != 5 || !feedback.iter().all(|&c| c == 'G' || c == 'Y' || c == 'B') {
        return Err("Feedback must be 5 characters of G, Y, or B".to_string());
    }
    Ok(Guess::new(word, feedback))
}

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

fn filter_by_green(words: &mut Vec<String>, guess: &Guess) {
    let greens = get_positions(&guess.feedback, 'G', &guess.word);
    words.retain(|word| {
        greens
            .iter()
            .all(|&(i, c)| word.chars().nth(i).unwrap() == c)
    });
}

fn filter_by_yellow(words: &mut Vec<String>, guess: &Guess) {
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

fn filter_by_black(words: &mut Vec<String>, guess: &Guess) {
    let blacks = get_positions(&guess.feedback, 'B', &guess.word);
    
    // Get unique black letters that are NOT also green or yellow
    let green_yellow_letters: Vec<char> = guess.feedback.iter()
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

fn filter_words(words: &mut Vec<String>, guess: &Guess) {
    filter_by_green(words, guess);
    filter_by_yellow(words, guess);
    filter_by_black(words, guess);
}

fn calculate_feedback(guess: &str, solution: &str) -> Vec<char> {
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

fn load_words() -> Result<Vec<String>, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    let file = File::open("words.txt")?;
    let reader = BufReader::new(file);
    let words: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|word| word.to_uppercase())
        .filter(|word| word.len() == 5)
        .collect();
    
    Ok(words)
}

fn solver_mode() {
    use std::io::{self, Write};
    
    println!("🔍 Wordle Solver Mode");
    println!("====================");
    println!("I'll help you solve today's Wordle!");
    println!("Enter your guesses and feedback (e.g., 'CRANE BYYBB')\n");
    
    // Load word list
    let mut candidates = match load_words() {
        Ok(words) => {
            println!("✅ Loaded {} words from words.txt\n", words.len());
            words
        }
        Err(e) => {
            eprintln!("❌ Error loading words.txt: {}", e);
            eprintln!("Creating a small default word list...");
            vec![
                "CRANE", "SLATE", "CRISP", "TRACE", "BRAKE",
                "GRAPE", "PRIME", "STALE", "BREAD", "GREAT"
            ].iter().map(|&s| s.to_string()).collect()
        }
    };
    
    println!("📝 Starting candidates: {}", candidates.len());
    println!("\n💡 Suggested first guess: CRANE or SLATE\n");
    
    let mut attempt = 1;
    
    // Solver loop
    loop {
        print!("🎲 Attempt #{} - Enter 'GUESS FEEDBACK' (or 'quit'): ", attempt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        
        if input.to_lowercase() == "quit" {
            println!("👋 Thanks for playing!");
            break;
        }
        
        // Parse the input
        match parse_input(input) {
            Ok(guess) => {
                // Display feedback visualization
                print!("📊 Your feedback: ");
                for &fb in guess.feedback.iter() {
                    let symbol = match fb {
                        'G' => "🟩",
                        'Y' => "🟨",
                        _ => "⬜",
                    };
                    print!("{}", symbol);
                }
                println!();
                
                // Check for win
                if guess.feedback.iter().all(|&c| c == 'G') {
                    println!("\n🎉 Congratulations! You solved it in {} attempts!", attempt);
                    println!("✨ The word was: {}", guess.word);
                    break;
                }
                
                // Filter candidates
                filter_words(&mut candidates, &guess);
                
                println!("\n📝 Candidates remaining: {}", candidates.len());
                
                if candidates.is_empty() {
                    println!("❌ No candidates left! Check your input or the word might not be in our list.");
                    break;
                } else if candidates.len() == 1 {
                    println!("🎯 Only one possibility left: {}", candidates[0]);
                    println!("💡 Try this word next!");
                } else if candidates.len() <= 20 {
                    println!("💡 Possible words:");
                    for chunk in candidates.chunks(10) {
                        println!("   {}", chunk.join(", "));
                    }
                    
                    // Suggest best next guess (simple strategy: pick first candidate)
                    println!("\n💡 Suggested next guess: {}", candidates[0]);
                } else if candidates.len() <= 200 {
                    println!("💡 Top candidates:");
                    for chunk in candidates[..20.min(candidates.len())].chunks(10) {
                        println!("   {}", chunk.join(", "));
                    }
                    println!("   ... and {} more", candidates.len() - 20);
                    
                    // Suggest a good elimination word
                    println!("\n💡 Suggested next guess: {} (good for elimination)", 
                            if attempt == 1 { "SLATE" } else { &candidates[0] });
                } else {
                    println!("💡 Too many candidates to display ({} words)", candidates.len());
                    println!("💡 Suggested next guess: {} (common word for elimination)", 
                            if attempt == 1 { "SLATE" } else if attempt == 2 { "CRIMP" } else { &candidates[0] });
                }
                
                attempt += 1;
                
                if attempt > 6 {
                    println!("\n😔 Reached maximum attempts!");
                    if candidates.len() <= 10 {
                        println!("The word was likely one of: {}", candidates.join(", "));
                    }
                    break;
                }
            }
            Err(e) => {
                println!("❌ {}", e);
                println!("Format: WORD FEEDBACK (e.g., 'CRANE BYYGG')");
                println!("Feedback: G=Green(🟩), Y=Yellow(🟨), B=Black(⬜)");
            }
        }
        
        println!();
    }
}

fn game_mode() {
    use std::io::{self, Write};
    
    println!("🎮 Wordle Game Mode");
    println!("==================");
    
    // Load word list
    let mut candidates = match load_words() {
        Ok(words) => {
            println!("✅ Loaded {} words from words.txt\n", words.len());
            words
        }
        Err(e) => {
            eprintln!("❌ Error loading words.txt: {}", e);
            eprintln!("Creating a small default word list...");
            vec![
                "CRANE", "SLATE", "CRISP", "TRACE", "BRAKE",
                "GRAPE", "PRIME", "STALE", "BREAD", "GREAT"
            ].iter().map(|&s| s.to_string()).collect()
        }
    };
    
    // Get the solution word
    print!("Enter the solution word (5 letters): ");
    io::stdout().flush().unwrap();
    
    let mut solution = String::new();
    io::stdin().read_line(&mut solution).expect("Failed to read solution");
    let solution = solution.trim().to_uppercase();
    
    if solution.len() != 5 {
        eprintln!("❌ Solution must be exactly 5 letters!");
        return;
    }
    
    println!("\n🎯 Solution set! Let's start guessing.\n");
    println!("📝 Candidates remaining: {}", candidates.len());
    
    let mut attempt = 1;
    
    // Game loop
    loop {
        print!("\n🎲 Attempt #{}: Enter your guess: ", attempt);
        io::stdout().flush().unwrap();
        
        let mut guess_input = String::new();
        io::stdin().read_line(&mut guess_input).expect("Failed to read guess");
        let guess_word = guess_input.trim().to_uppercase();
        
        if guess_word.len() != 5 {
            println!("❌ Guess must be exactly 5 letters!");
            continue;
        }
        
        // Calculate feedback
        let feedback = calculate_feedback(&guess_word, &solution);
        
        // Display feedback
        print!("📊 Feedback: ");
        for &fb in feedback.iter() {
            let symbol = match fb {
                'G' => "🟩",
                'Y' => "🟨",
                _ => "⬜",
            };
            print!("{}", symbol);
        }
        println!(" ({})", feedback.iter().collect::<String>());
        
        // Check for win
        if feedback.iter().all(|&c| c == 'G') {
            println!("\n🎉 Congratulations! You found the word: {}", solution);
            println!("✨ Solved in {} attempts!", attempt);
            break;
        }
        
        // Filter candidates based on feedback
        let guess = Guess::new(guess_word.clone(), feedback);
        filter_words(&mut candidates, &guess);
        
        println!("\n📝 Candidates remaining: {}", candidates.len());
        
        if candidates.is_empty() {
            println!("❌ No candidates left! Something went wrong.");
            break;
        } else if candidates.len() <= 200 {
            println!("💡 Possible words:");
            // Display words in rows of 10 for better readability
            for chunk in candidates.chunks(10) {
                println!("   {}", chunk.join(", "));
            }
        } else {
            println!("💡 First 100 candidates:");
            for chunk in candidates[..100.min(candidates.len())].chunks(10) {
                println!("   {}", chunk.join(", "));
            }
        }
        
        attempt += 1;
        
        if attempt > 6 {
            println!("\n😔 Game over! The word was: {}", solution);
            break;
        }
    }
}

fn main() {
    use std::io::{self, Write};
    
    println!("🎮 Wordle CLI");
    println!("=============\n");
    println!("Choose mode:");
    println!("1. Solver Mode - I'll help you solve a Wordle");
    println!("2. Game Mode - Play Wordle with a known solution\n");
    
    print!("Enter choice (1 or 2): ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");
    
    match choice.trim() {
        "1" => solver_mode(),
        "2" => game_mode(),
        _ => {
            println!("Invalid choice. Defaulting to Solver Mode.");
            solver_mode();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_feedback_all_green() {
        let feedback = calculate_feedback("CRANE", "CRANE");
        assert_eq!(feedback, vec!['G', 'G', 'G', 'G', 'G']);
    }

    #[test]
    fn test_calculate_feedback_mixed() {
        // Solution: CRANE, Guess: TRACE
        // T not in solution -> B
        // R in correct position -> G
        // A in correct position -> G
        // C in solution but wrong position -> Y
        // E in correct position -> G
        let feedback = calculate_feedback("TRACE", "CRANE");
        assert_eq!(feedback, vec!['B', 'G', 'G', 'Y', 'G']);
    }

    #[test]
    fn test_calculate_feedback_duplicate_letters() {
        // Solution: LEVEL, Guess: LLAMA
        // L in correct position -> G
        // L in solution at position 4, so -> Y
        // A not in solution -> B
        // M not in solution -> B
        // A not in solution -> B
        let feedback = calculate_feedback("LLAMA", "LEVEL");
        assert_eq!(feedback, vec!['G', 'Y', 'B', 'B', 'B']);
    }

    #[test]
    fn test_calculate_feedback_all_yellow() {
        // Solution: ABCDE, Guess: EABCD
        // All letters present but in wrong positions
        let feedback = calculate_feedback("EABCD", "ABCDE");
        assert_eq!(feedback, vec!['Y', 'Y', 'Y', 'Y', 'Y']);
    }

    #[test]
    fn test_parse_input() {
        let word = "CRANE";
        let feedback = "GYYBB";
        let expected_feedback: Vec<char> = feedback.chars().collect();

        let input = format!("{} {}", word, feedback);
        let result = parse_input(&input);

        assert!(result.is_ok());
        let guess = result.unwrap();

        assert_eq!(expected_feedback, guess.feedback);
        assert_eq!(word, guess.word);
    }

    #[test]
    fn test_parse_input_invalid_word_length() {
        let input = "CRAN GYYBB";
        let result = parse_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_input_invalid_feedback_length() {
        let input = "CRANE GYBB";
        let result = parse_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_input_invalid_feedback_char() {
        let input = "CRANE GYxBx";
        let result = parse_input(input);
        assert!(result.is_err());
    }

    // Unit tests for individual filters
    #[test]
    fn test_filter_by_green_only() {
        let mut words = vec![
            "CRANE".to_string(),
            "CRATE".to_string(),
            "BRAKE".to_string(),
            "DRAKE".to_string(),
        ];

        // Only C is green at position 0
        let guess = Guess::new("CRANE".to_string(), vec!['G', 'B', 'B', 'B', 'B']);

        filter_by_green(&mut words, &guess);

        // Should keep words starting with C
        assert!(words.contains(&"CRANE".to_string()));
        assert!(words.contains(&"CRATE".to_string()));
        // Should remove words not starting with C
        assert!(!words.contains(&"BRAKE".to_string()));
        assert!(!words.contains(&"DRAKE".to_string()));
    }

    #[test]
    fn test_filter_by_yellow_only() {
        let mut words = vec![
            "CRANE".to_string(),
            "BRAKE".to_string(),
            "RAVEN".to_string(),
            "SOBER".to_string(),
            "THINK".to_string(),
        ];

        // R is yellow at position 1
        let guess = Guess::new("CRANE".to_string(), vec!['B', 'Y', 'B', 'B', 'B']);

        filter_by_yellow(&mut words, &guess);

        // Should keep words with R but not at position 1
        assert!(words.contains(&"RAVEN".to_string())); // R at position 0
        assert!(words.contains(&"SOBER".to_string())); // R at position 4
        // Should remove words with R at position 1
        assert!(!words.contains(&"CRANE".to_string()));
        assert!(!words.contains(&"BRAKE".to_string()));
        // Should remove words without R
        assert!(!words.contains(&"THINK".to_string()));
    }

    #[test]
    fn test_filter_by_black_only() {
        let mut words = vec![
            "CRANE".to_string(),
            "STAIR".to_string(),
            "POUND".to_string(),
            "LIGHT".to_string(),
            "DUMPS".to_string(),
            "GLYPH".to_string(),
        ];

        // All letters are black
        let guess = Guess::new("CRANE".to_string(), vec!['B', 'B', 'B', 'B', 'B']);

        filter_by_black(&mut words, &guess);

        // Should remove words containing C, R, A, N, or E
        assert!(!words.contains(&"CRANE".to_string())); // Has C, R, A, N, E
        assert!(!words.contains(&"STAIR".to_string())); // Has A and R
        assert!(!words.contains(&"POUND".to_string())); // Has N
        // Should keep words without any of those letters
        assert!(words.contains(&"LIGHT".to_string())); // No C, R, A, N, E
        assert!(words.contains(&"DUMPS".to_string())); // No C, R, A, N, E
        assert!(words.contains(&"GLYPH".to_string())); // No C, R, A, N, E
    }

    #[test]
    fn test_filter_by_black_simple() {
        let mut words = vec![
            "WORLD".to_string(),
            "WORTH".to_string(), 
            "WIMPY".to_string(),
            "POUND".to_string(),
            "DUMPS".to_string(),
            "GLYPH".to_string(),
        ];

        // Testing with all black letters from guess "CRANE"
        let guess = Guess::new("CRANE".to_string(), vec!['B', 'B', 'B', 'B', 'B']);

        filter_by_black(&mut words, &guess);

        // Should remove all words containing C, R, A, N, or E
        assert!(!words.contains(&"WORLD".to_string())); // Has R
        assert!(!words.contains(&"WORTH".to_string())); // Has R
        assert!(!words.contains(&"POUND".to_string())); // Has N
        // Should keep words without any of those letters
        assert!(words.contains(&"WIMPY".to_string())); // No C, R, A, N, E
        assert!(words.contains(&"DUMPS".to_string())); // No C, R, A, N, E
        assert!(words.contains(&"GLYPH".to_string())); // No C, R, A, N, E
    }

    #[test]
    fn test_filter_green_letters() {
        let mut words = vec![
            "CRANE".to_string(),
            "CRATE".to_string(),
            "BRAKE".to_string(),
            "DRAKE".to_string(),
        ];

        let guess = Guess::new("CRANE".to_string(), vec!['G', 'B', 'B', 'B', 'B']);

        filter_words(&mut words, &guess);

        // Note: This now also filters out words with R, A, N, E due to black filter
        assert!(!words.contains(&"CRANE".to_string())); // Has R, A, N, E
        assert!(!words.contains(&"CRATE".to_string())); // Has R, A, T, E
        assert!(!words.contains(&"BRAKE".to_string())); // Has R, A, K, E
        assert!(!words.contains(&"DRAKE".to_string())); // Has R, A, K, E
    }

    #[test]
    fn test_filter_integration_complete() {
        let mut words = vec![
            "FROST".to_string(),
            "SPORT".to_string(), 
            "TRUCK".to_string(),
            "GROUT".to_string(),
            "BROTH".to_string(),
            "DRUMS".to_string(),
            "SHIRT".to_string(),
            "QUIRK".to_string(),
        ];

        // Guess CRANE with R yellow at position 1, everything else black
        let guess = Guess::new("CRANE".to_string(), vec!['B', 'Y', 'B', 'B', 'B']);

        filter_words(&mut words, &guess);

        // Should keep words with R but NOT at position 1, and without C, A, N, E
        // FROST has R at position 1 - filtered out
        // SPORT has R at position 3 - good to keep
        // TRUCK has R at position 1 - filtered out, also has C
        // GROUT has R at position 1 - filtered out
        // BROTH has R at position 1 - filtered out
        // DRUMS has R at position 1 - filtered out
        // SHIRT has R at position 3 - good to keep
        // QUIRK has R at position 4 - good to keep
        
        assert!(words.contains(&"SPORT".to_string())); // R at position 3, no C,A,N,E
        assert!(words.contains(&"SHIRT".to_string())); // R at position 3, no C,A,N,E
        assert!(words.contains(&"QUIRK".to_string())); // R at position 4, no C,A,N,E
        
        assert!(!words.contains(&"FROST".to_string())); // R at position 1
        assert!(!words.contains(&"TRUCK".to_string())); // R at position 1, has C
        assert!(!words.contains(&"GROUT".to_string())); // R at position 1
        assert!(!words.contains(&"BROTH".to_string())); // R at position 1
        assert!(!words.contains(&"DRUMS".to_string())); // R at position 1
    }

    #[test]
    fn test_integration_realistic_wordle() {
        let mut words = vec![
            "CRANE".to_string(),
            "BRAIN".to_string(),
            "GRAIN".to_string(),
            "TRAIN".to_string(),
            "STAIN".to_string(),
            "PLAIN".to_string(),
            "CHAIN".to_string(),
        ];

        // Guess CRANE, get: C=Black, R=Green, A=Green, N=Yellow, E=Black
        let guess = Guess::new("CRANE".to_string(), vec!['B', 'G', 'G', 'Y', 'B']);

        filter_words(&mut words, &guess);

        // Should keep words with:
        // - R at position 1 (Green)
        // - A at position 2 (Green)
        // - N somewhere but not position 3 (Yellow)
        // - No C or E (Black)
        
        assert!(words.contains(&"BRAIN".to_string())); // Fits all criteria
        assert!(words.contains(&"GRAIN".to_string())); // Fits all criteria
        assert!(words.contains(&"TRAIN".to_string())); // Fits all criteria
        
        // These should be filtered out:
        assert!(!words.contains(&"CRANE".to_string())); // Has C and E
        assert!(!words.contains(&"STAIN".to_string())); // No R at position 1
        assert!(!words.contains(&"PLAIN".to_string())); // No R at position 1
        assert!(!words.contains(&"CHAIN".to_string())); // Has C, no R at position 1
    }

    #[test]
    fn test_integration_with_mixed_feedback() {
        let mut words = vec![
            "LIGHT".to_string(),
            "SIGHT".to_string(),
            "MIGHT".to_string(),
            "NIGHT".to_string(),
            "FIGHT".to_string(),
            "TIGHT".to_string(),
        ];

        // Guess LIGHT, get: L=Black, I=Green, G=Green, H=Green, T=Green
        // This means only L is wrong
        let guess = Guess::new("LIGHT".to_string(), vec!['B', 'G', 'G', 'G', 'G']);

        filter_words(&mut words, &guess);

        // Should keep words with I,G,H,T in positions 1,2,3,4 but no L
        assert!(words.contains(&"SIGHT".to_string()));
        assert!(words.contains(&"MIGHT".to_string()));
        assert!(words.contains(&"NIGHT".to_string()));
        assert!(words.contains(&"FIGHT".to_string()));
        assert!(words.contains(&"TIGHT".to_string()));
        // Should remove words with L
        assert!(!words.contains(&"LIGHT".to_string()));
    }
}
