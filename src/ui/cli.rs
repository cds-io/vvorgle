use crate::wordle::{core, solver::SolverStrategy, types::*};
use std::io::{self, Write};

pub fn run_solver_mode(solver: Box<dyn SolverStrategy>) {
    println!("🔍 Wordle Solver Mode");
    println!("====================");
    println!("I'll help you solve today's Wordle!");
    println!("Enter your guesses and feedback (e.g., 'CRANE BYYBB')\n");

    // Load embedded word list
    let all_words = core::load_words().expect("Failed to load embedded word list");
    println!("✅ Loaded {} words\n", all_words.len());

    let mut state = GameState::new(all_words.clone());

    println!("📝 Starting candidates: {}", state.candidates.len());
    let suggestion = solver.suggest_guess(&state, &all_words);
    println!("\n💡 Suggested first guess: {}\n", suggestion);

    // Solver loop
    loop {
        print!(
            "🎲 Attempt #{} - Enter 'GUESS FEEDBACK' (or /h for help): ",
            state.attempt_count + 1
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        // Handle commands

        match input.to_lowercase().as_str() {
            "/q" | "/quit" => {
                println!("👋 Thanks for playing!");
                break;
            }
            "/r" | "/reset" => {
                println!("🔄 Restarting solver...");
                state = GameState::new(all_words.clone());
                println!("📝 Candidates reset to: {}", state.candidates.len());
                let suggestion = solver.suggest_guess(&state, &all_words);
                println!("\n💡 Suggested first guess: {}\n", suggestion);
                continue;
            }
            "/h" | "/help" => {
                println!("\n📚 Available commands:");
                println!("  /h, /help   - Show this help message");
                println!("  /s, /stats  - Show current game statistics");
                println!("  /r, /reset  - Start over with a fresh word list");
                println!("  /q, /quit   - Exit the solver");
                println!("\n📝 Input format: WORD FEEDBACK");
                println!("  Example: CRANE BYYGG");
                println!("  G=Green(🟩), Y=Yellow(🟨), B=Black(⬜)\n");
                continue;
            }
            "/s" | "/stats" => {
                println!("\n📊 Current Statistics:");
                println!("  Attempt:     #{}", state.attempt_count + 1);
                println!("  Candidates:  {} words remaining", state.candidates.len());
                println!(
                    "  Reduction:   {:.1}% eliminated",
                    (1.0 - state.candidates.len() as f64 / all_words.len() as f64) * 100.0
                );

                let available = state.get_available_letters_sorted();
                if !available.is_empty() {
                    println!("\n🔤 Available Letters ({}):", available.len());
                    print!("  ");
                    for (i, letter) in available.iter().enumerate() {
                        if i > 0 && i % 13 == 0 {
                            println!();
                            print!("  ");
                        }
                        print!("{} ", letter);
                    }
                    println!();
                }

                if !state.attempts.is_empty() {
                    println!("\n📝 Previous guesses:");
                    for (i, attempt) in state.attempts.iter().enumerate() {
                        print!("  {}. {} → ", i + 1, attempt.word);
                        for &fb in attempt.feedback.iter() {
                            print!(
                                "{}",
                                match fb {
                                    'G' => "🟩",
                                    'Y' => "🟨",
                                    _ => "⬜",
                                }
                            );
                        }
                        println!();
                    }
                }

                if state.candidates.len() <= 20 && state.candidates.len() > 0 {
                    println!("\n💡 Current candidates:");
                    for chunk in state.candidates.chunks(5) {
                        println!("  {}", chunk.join(", "));
                    }
                }

                println!();
                continue;
            }
            _ => {
                // Not a command, continue to parse as guess/feedback
            }
        }

        // Parse the input
        match core::parse_input(input) {
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
                    println!(
                        "\n🎉 Congratulations! You solved it in {} attempts!",
                        state.attempt_count + 1
                    );
                    println!("✨ The word was: {}", guess.word);
                    break;
                }

                // Update state and filter candidates
                state.add_attempt(guess.clone());
                core::filter_words(&mut state.candidates, &guess);

                println!("\n📝 Candidates remaining: {}", state.candidates.len());

                if state.candidates.is_empty() {
                    println!(
                        "❌ No candidates left! Check your input or the word might not be in our list."
                    );
                    break;
                } else if state.candidates.len() == 1 {
                    println!("🎯 Only one possibility left: {}", state.candidates[0]);
                    println!("💡 Try this word next!");
                } else if state.candidates.len() <= 20 {
                    println!("💡 Possible words:");
                    for chunk in state.candidates.chunks(10) {
                        println!("   {}", chunk.join(", "));
                    }

                    // Use solver for suggestion
                    let suggestion = solver.suggest_guess(&state, &all_words);
                    println!("\n💡 Suggested next guess: {}", suggestion);
                } else if state.candidates.len() <= 200 {
                    println!("💡 Top candidates:");
                    for chunk in state.candidates[..20.min(state.candidates.len())].chunks(10) {
                        println!("   {}", chunk.join(", "));
                    }
                    println!("   ... and {} more", state.candidates.len() - 20);

                    // Use solver for suggestion
                    let suggestion = solver.suggest_guess(&state, &all_words);
                    println!("\n💡 Suggested next guess: {}", suggestion);
                } else {
                    println!(
                        "💡 Too many candidates to display ({} words)",
                        state.candidates.len()
                    );
                    // Use solver for suggestion
                    let suggestion = solver.suggest_guess(&state, &all_words);
                    println!("💡 Suggested next guess: {}", suggestion);
                }

                if state.attempt_count >= 6 {
                    println!("\n😔 Reached maximum attempts!");
                    if state.candidates.len() <= 10 {
                        println!(
                            "The word was likely one of: {}",
                            state.candidates.join(", ")
                        );
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

pub fn run_game_mode() {
    println!("🎮 Wordle Game Mode");
    println!("==================");

    // Load embedded word list
    let mut candidates = core::load_words().expect("Failed to load embedded word list");
    println!("✅ Loaded {} words\n", candidates.len());

    // Get the solution word
    print!("Enter the solution word (5 letters): ");
    io::stdout().flush().unwrap();

    let mut solution = String::new();
    io::stdin()
        .read_line(&mut solution)
        .expect("Failed to read solution");
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
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read guess");
        let guess_word = guess_input.trim().to_uppercase();

        if guess_word.len() != 5 {
            println!("❌ Guess must be exactly 5 letters!");
            continue;
        }

        // Calculate feedback
        let feedback = core::calculate_feedback(&guess_word, &solution);

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
        core::filter_words(&mut candidates, &guess);

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
