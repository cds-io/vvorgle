# Wordle Solver CLI

A Rust-based Wordle solver with multiple solving strategies, including an entropy maximization algorithm for optimal word selection.

## Features

- **Interactive CLI** with two modes:
  - **Solver Mode**: Helps you solve any Wordle puzzle
  - **Game Mode**: Play Wordle with a known solution

- **Multiple Solving Strategies**:
  - **Simple**: Fast, picks alphabetically first candidate
  - **Entropy Maximizer**: Smart algorithm that maximizes information gain (see `algo.md` for details)
  - **Frequency Analyzer**: (Placeholder for letter frequency analysis)

- **Smart Filtering**:
  - Green letters (correct position)
  - Yellow letters (wrong position) 
  - Black/Gray letters (not in word)
  - Handles duplicate letters correctly

- **Letter Pool Tracking**:
  - Automatically tracks available letters (A-Z initially)
  - Removes eliminated letters based on gray feedback
  - Displays remaining letters in stats view

## Installation

```bash
cargo build --release
```

### Setting up Git Hooks

This project includes a pre-commit hook that automatically runs `cargo fmt` before each commit. To set it up:

```bash
# Run the setup script
./setup-hooks.sh

# Or manually set git to use the .githooks directory
git config core.hooksPath .githooks
```

The hook will ensure all Rust code is properly formatted before commits. To bypass it for a single commit, use `git commit --no-verify`.

## Usage

### Solver Mode (Help solve a Wordle)

```bash
cargo run
# Choose option 1 for Solver Mode
# Choose option 2 for Entropy strategy (recommended)
# Enter your guesses and feedback like: CRANE BYYGG
```

### Game Mode (Play with known solution)

```bash
cargo run
# Choose option 2 for Game Mode
# Enter the solution word
# Make guesses and see feedback
```

## Example Session

```
🎮 Wordle CLI
=============

Choose mode:
1. Solver Mode - I'll help you solve a Wordle
2. Game Mode - Play Wordle with a known solution

Enter choice (1 or 2): 1

Choose solver strategy:
1. Simple (fast, picks first candidate)
2. Entropy (smart, maximizes information gain)
3. Frequency (uses letter frequency analysis)

Enter strategy (1-3, default=2): 2

🔍 Wordle Solver Mode
====================
✅ Loaded 2309 words

💡 Suggested first guess: SALET

🎲 Attempt #1 - Enter 'GUESS FEEDBACK' (or 'quit'): SALET BBBGY
📊 Your feedback: ⬜⬜⬜🟩🟨

📝 Candidates remaining: 45
💡 Suggested next guess: CRYPT
```

## How the Entropy Maximizer Works

The entropy maximizer selects words that maximize information gain by:

1. **Calculating pattern distribution**: For each potential guess, it calculates how the remaining candidates would be distributed across different feedback patterns

2. **Computing entropy**: Uses Shannon entropy formula: `-Σ p * log2(p)` where p is the probability of each pattern

3. **Selecting optimal word**: Chooses the word that produces the highest entropy (most even distribution of outcomes)

4. **Adaptive strategy**:
   - Large candidate pools (>100): Samples from candidates only
   - Medium pools (20-100): Considers broader word list
   - Small pools (<20): Exhaustive search for optimal elimination

## Architecture

The codebase is organized into modules:

```
src/
├── main.rs           # Entry point
├── wordle/
│   ├── types.rs      # Core data structures (including letter pool)
│   ├── core.rs       # Filtering and feedback logic
│   ├── solver.rs     # Solver strategies (entropy, simple, etc.)
│   └── analyzer.rs   # Starting word analyzer tool
└── ui/
    └── cli.rs        # User interface with letter pool display
```

Key documentation:
- `CLAUDE.md`: Development guidelines for Claude Code
- `algo.md`: Mathematical explanation of Shannon entropy for Wordle

## Testing

```bash
cargo test                    # Run all tests
cargo test calculate_feedback # Test feedback calculation
cargo test filter             # Test filtering logic
```

## Performance

The entropy solver can analyze hundreds of words in milliseconds:
- First guess: Pre-computed optimal starter (SALET)
- Subsequent guesses: Dynamic entropy calculation
- Adaptive sampling for large candidate pools

## License

MIT