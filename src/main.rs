#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod ui;
mod wordle;

use ui::cli;
use wordle::{analyzer::StartingWordAnalyzer, solver};

fn main() {
    use std::env;
    use std::io::{self, Write};

    // Check for command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut strategy = "entropy";
    let mut analyze_starters = false;
    let mut analyze_word: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--simple" => strategy = "simple",
            "--entropy" => strategy = "entropy",
            "--frequency" => strategy = "frequency",
            "--analyze-starters" => analyze_starters = true,
            "--analyze" => {
                if i + 1 < args.len() {
                    analyze_word = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --analyze requires a word argument");
                    return;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Handle analysis modes
    if analyze_starters {
        println!("🔬 Analyzing word list to find best starting words...");
        println!("This may take a minute...\n");

        let analyzer = StartingWordAnalyzer::new().expect("Failed to load words");
        let best = analyzer.find_best_starters(20);

        println!("\n📊 Top 20 Starting Words by Entropy:\n");
        println!("Rank  Word   Entropy  E[remaining]  P(win)");
        println!("─────────────────────────────────────────────");

        for (i, (word, entropy, exp_rem, p_win)) in best.iter().enumerate() {
            println!(
                "{:3}   {}   {:.3}     {:.1}        {:.4}",
                i + 1,
                word,
                entropy,
                exp_rem,
                p_win
            );
        }

        println!("\n💡 Best overall starter: {}\n", best[0].0);
        return;
    }

    if let Some(word) = analyze_word {
        println!("🔬 Analyzing '{}' as a starting word...\n", word);

        let analyzer = StartingWordAnalyzer::new().expect("Failed to load words");

        if let Some((entropy, exp_rem, p_win, patterns)) = analyzer.analyze_word(&word) {
            println!("📊 Statistics for '{}':", word.to_uppercase());
            println!("  Entropy: {:.3} bits", entropy);
            println!("  Expected remaining: {:.1} words", exp_rem);
            println!("  P(instant win): {:.4}\n", p_win);

            println!("🎯 Top 10 most common patterns:");
            for (i, (pid, count)) in patterns.iter().take(10).enumerate() {
                let pattern = wordle::analyzer::pattern_to_string(*pid);
                let percentage = (*count as f64 / 2316.0) * 100.0;
                println!(
                    "  {:2}. {} - {} times ({:.1}%)",
                    i + 1,
                    pattern,
                    count,
                    percentage
                );
            }
        } else {
            eprintln!("Error: Could not analyze '{}'", word);
        }
        return;
    }

    // Normal game modes
    println!("🎮 Wordle CLI");
    println!("=============\n");
    println!("Choose mode:");
    println!("1. Solver Mode - I'll help you solve a Wordle");
    println!("2. Game Mode - Play Wordle with a known solution\n");

    print!("Enter choice (1 or 2): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice");

    match choice.trim() {
        "1" => {
            let solver = solver::create_solver(strategy);
            println!(
                "\n📊 Using {} strategy",
                match strategy {
                    "simple" => "Simple",
                    "frequency" => "Frequency Analyzer",
                    _ => "Entropy Maximizer (optimal)",
                }
            );

            cli::run_solver_mode(solver);
        }
        "2" => cli::run_game_mode(),
        _ => {
            println!("Invalid choice. Defaulting to Solver Mode.");
            let solver = solver::create_solver(strategy);
            println!(
                "📊 Using {} strategy",
                match strategy {
                    "simple" => "Simple",
                    "frequency" => "Frequency Analyzer",
                    _ => "Entropy Maximizer (optimal)",
                }
            );
            cli::run_solver_mode(solver);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wordle::{core::*, types::*};

    #[test]
    fn test_calculate_feedback_all_green() {
        let feedback = calculate_feedback("CRANE", "CRANE");
        assert_eq!(feedback, vec!['G', 'G', 'G', 'G', 'G']);
    }

    #[test]
    fn test_calculate_feedback_mixed() {
        let feedback = calculate_feedback("TRACE", "CRANE");
        assert_eq!(feedback, vec!['B', 'G', 'G', 'Y', 'G']);
    }

    #[test]
    fn test_calculate_feedback_duplicate_letters() {
        let feedback = calculate_feedback("LLAMA", "LEVEL");
        assert_eq!(feedback, vec!['G', 'Y', 'B', 'B', 'B']);
    }

    #[test]
    fn test_calculate_feedback_all_yellow() {
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
        let result = parse_input(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_input_invalid_feedback_length() {
        let input = "CRANE GYBB";
        let result = parse_input(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_input_invalid_feedback_char() {
        let input = "CRANE GYxBx";
        let result = parse_input(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_by_green_only() {
        let mut words = vec![
            "CRANE".to_string(),
            "CRATE".to_string(),
            "BRAKE".to_string(),
            "DRAKE".to_string(),
        ];

        let guess = Guess::new("CRANE".to_string(), vec!['G', 'B', 'B', 'B', 'B']);

        filter_by_green(&mut words, &guess);

        assert!(words.contains(&"CRANE".to_string()));
        assert!(words.contains(&"CRATE".to_string()));
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

        let guess = Guess::new("CRANE".to_string(), vec!['B', 'Y', 'B', 'B', 'B']);

        filter_by_yellow(&mut words, &guess);

        assert!(words.contains(&"RAVEN".to_string()));
        assert!(words.contains(&"SOBER".to_string()));
        assert!(!words.contains(&"CRANE".to_string()));
        assert!(!words.contains(&"BRAKE".to_string()));
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

        let guess = Guess::new("CRANE".to_string(), vec!['B', 'B', 'B', 'B', 'B']);

        filter_by_black(&mut words, &guess);

        assert!(!words.contains(&"CRANE".to_string()));
        assert!(!words.contains(&"STAIR".to_string()));
        assert!(!words.contains(&"POUND".to_string()));
        assert!(words.contains(&"LIGHT".to_string()));
        assert!(words.contains(&"DUMPS".to_string()));
        assert!(words.contains(&"GLYPH".to_string()));
    }

    #[test]
    fn test_filter_integration_realistic_wordle() {
        let mut words = vec![
            "CRANE".to_string(),
            "BRAIN".to_string(),
            "GRAIN".to_string(),
            "TRAIN".to_string(),
            "STAIN".to_string(),
            "PLAIN".to_string(),
            "CHAIN".to_string(),
        ];

        let guess = Guess::new("CRANE".to_string(), vec!['B', 'G', 'G', 'Y', 'B']);

        filter_words(&mut words, &guess);

        assert!(words.contains(&"BRAIN".to_string()));
        assert!(words.contains(&"GRAIN".to_string()));
        assert!(words.contains(&"TRAIN".to_string()));
        assert!(!words.contains(&"CRANE".to_string()));
        assert!(!words.contains(&"STAIN".to_string()));
        assert!(!words.contains(&"PLAIN".to_string()));
        assert!(!words.contains(&"CHAIN".to_string()));
    }
}
