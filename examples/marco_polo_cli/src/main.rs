//! Marco Polo CLI - A canonical example for Depyler transpilation
//!
//! This is the ideal Rust output that Depyler aims to generate from Python code.
//! It demonstrates:
//! - Command-line argument parsing with clap
//! - Game state management
//! - Error handling with Result types
//! - Idiomatic Rust patterns

use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

/// Marco Polo CLI - A number guessing game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of rounds to play
    #[arg(short, long, default_value_t = 3)]
    rounds: u32,

    /// Game difficulty
    #[arg(short, long, value_enum, default_value_t = Difficulty::Medium)]
    difficulty: Difficulty,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn range(&self) -> (i32, i32) {
        match self {
            Difficulty::Easy => (1, 10),
            Difficulty::Medium => (1, 50),
            Difficulty::Hard => (1, 100),
        }
    }
}

struct MarcoPoloGame {
    difficulty: Difficulty,
    verbose: bool,
    score: u32,
    attempts: u32,
}

impl MarcoPoloGame {
    fn new(difficulty: Difficulty, verbose: bool) -> Self {
        Self {
            difficulty,
            verbose,
            score: 0,
            attempts: 0,
        }
    }

    fn generate_number(&self) -> i32 {
        let (min, max) = self.difficulty.range();
        rand::thread_rng().gen_range(min..=max)
    }

    fn get_hint(&self, guess: i32, target: i32) -> &'static str {
        match guess.cmp(&target) {
            Ordering::Less => {
                let distance = target - guess;
                match distance {
                    d if d > 20 => "Marco! (Way too low)",
                    d if d > 10 => "Marco! (Too low)",
                    _ => "Marco! (A bit low)",
                }
            }
            Ordering::Greater => {
                let distance = guess - target;
                match distance {
                    d if d > 20 => "Marco! (Way too high)",
                    d if d > 10 => "Marco! (Too high)",
                    _ => "Marco! (A bit high)",
                }
            }
            Ordering::Equal => unreachable!("This should be handled before calling get_hint"),
        }
    }

    fn play_round(&mut self) -> Result<bool> {
        let target = self.generate_number();
        let (min, max) = self.difficulty.range();

        if self.verbose {
            println!("\n{} Target number: {}", "[DEBUG]".bright_black(), target);
        }

        println!(
            "\nI'm thinking of a number between {} and {}...",
            min.to_string().bright_cyan(),
            max.to_string().bright_cyan()
        );
        println!("When you guess, I'll say 'Marco!' and give you a hint.");
        println!("Find the number to hear 'Polo!'");

        let mut round_attempts = 0;

        loop {
            print!("\n{} ", "Your guess:".bright_yellow());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let guess = match input.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", "Please enter a valid number!".red());
                    continue;
                }
            };

            if guess < min || guess > max {
                println!(
                    "{}",
                    format!("Please guess between {} and {}", min, max).red()
                );
                continue;
            }

            round_attempts += 1;
            self.attempts += 1;

            match guess.cmp(&target) {
                Ordering::Equal => {
                    println!("{}", "🎉 Polo! You found it!".bright_green().bold());
                    self.score += 1;
                    if self.verbose {
                        println!(
                            "{} Attempts this round: {}",
                            "[DEBUG]".bright_black(),
                            round_attempts
                        );
                    }
                    return Ok(true);
                }
                _ => {
                    let hint = self.get_hint(guess, target);
                    println!("{}", hint.bright_blue());
                }
            }
        }
    }

    fn calculate_performance(&self) -> &'static str {
        if self.attempts == 0 {
            return "No games played";
        }

        let avg_attempts = self.attempts as f64 / self.score.max(1) as f64;

        match avg_attempts {
            avg if avg <= 5.0 => "🏆 Expert",
            avg if avg <= 7.0 => "⭐ Good",
            avg if avg <= 10.0 => "👍 Average",
            _ => "🎯 Keep practicing",
        }
    }
}

fn print_welcome() {
    let banner = "=".repeat(50);
    println!("{}", banner.bright_blue());
    println!(
        "{}",
        "🎮 Welcome to Marco Polo CLI! 🎮"
            .bright_yellow()
            .bold()
    );
    println!("{}", banner.bright_blue());
}

fn print_statistics(game: &MarcoPoloGame, rounds: u32) {
    let banner = "=".repeat(50);
    println!("\n{}", banner.bright_blue());
    println!("{}", "📊 Game Statistics 📊".bright_yellow().bold());
    println!("{}", banner.bright_blue());
    
    println!(
        "Rounds played: {}/{}",
        game.score.to_string().bright_green(),
        rounds.to_string().bright_green()
    );
    println!(
        "Total attempts: {}",
        game.attempts.to_string().bright_cyan()
    );
    
    if game.score > 0 {
        let avg = game.attempts as f64 / game.score as f64;
        println!(
            "Average attempts per round: {}",
            format!("{:.1}", avg).bright_cyan()
        );
    }
    
    println!(
        "Performance: {}",
        game.calculate_performance()
    );
    println!("{}", banner.bright_blue());
}

fn main() -> Result<()> {
    let args = Args::parse();

    print_welcome();

    let mut game = MarcoPoloGame::new(args.difficulty, args.verbose);

    println!(
        "\nStarting {} rounds on {} difficulty...",
        args.rounds.to_string().bright_green(),
        format!("{:?}", args.difficulty).to_lowercase().bright_yellow()
    );

    for round_num in 1..=args.rounds {
        let separator = "=".repeat(30);
        println!("\n{}", separator.bright_black());
        println!(
            "Round {} of {}",
            round_num.to_string().bright_cyan(),
            args.rounds.to_string().bright_cyan()
        );
        println!("{}", separator.bright_black());

        match game.play_round() {
            Ok(true) => continue,
            Ok(false) => unreachable!(),
            Err(_) => {
                println!("\n{}", "Game interrupted!".red());
                break;
            }
        }
    }

    print_statistics(&game, args.rounds);
    println!("\n{} 👋", "Thanks for playing!".bright_green());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_ranges() {
        assert_eq!(Difficulty::Easy.range(), (1, 10));
        assert_eq!(Difficulty::Medium.range(), (1, 50));
        assert_eq!(Difficulty::Hard.range(), (1, 100));
    }

    #[test]
    fn test_hint_generation() {
        let game = MarcoPoloGame::new(Difficulty::Medium, false);
        
        assert_eq!(game.get_hint(10, 50), "Marco! (Way too low)");
        assert_eq!(game.get_hint(35, 50), "Marco! (Too low)");
        assert_eq!(game.get_hint(45, 50), "Marco! (A bit low)");
        
        assert_eq!(game.get_hint(90, 50), "Marco! (Way too high)");
        assert_eq!(game.get_hint(65, 50), "Marco! (Too high)");
        assert_eq!(game.get_hint(55, 50), "Marco! (A bit high)");
    }

    #[test]
    fn test_performance_calculation() {
        let mut game = MarcoPoloGame::new(Difficulty::Medium, false);
        
        assert_eq!(game.calculate_performance(), "No games played");
        
        game.score = 1;
        game.attempts = 5;
        assert_eq!(game.calculate_performance(), "🏆 Expert");
        
        game.attempts = 10;
        assert_eq!(game.calculate_performance(), "👍 Average");
    }
}