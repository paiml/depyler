#!/usr/bin/env python3
"""
Marco Polo CLI - A canonical example for Depyler transpilation.

This simple CLI game demonstrates:
- Command-line argument parsing
- Interactive user input
- Game logic and state management
- Score tracking
- Type annotations for clean transpilation

Usage:
    python marco_polo.py --rounds 5 --difficulty easy
    python marco_polo.py -r 10 -d hard --verbose
"""

import argparse
import random
import sys
from typing import Dict, List, Optional, Tuple


class MarcoPoloGame:
    """Main game logic for Marco Polo."""
    
    def __init__(self, difficulty: str = "medium", verbose: bool = False):
        self.difficulty = difficulty
        self.verbose = verbose
        self.score = 0
        self.attempts = 0
        self.difficulty_ranges = {
            "easy": (1, 10),
            "medium": (1, 50),
            "hard": (1, 100)
        }
        
    def generate_number(self) -> int:
        """Generate a random number based on difficulty."""
        min_val, max_val = self.difficulty_ranges[self.difficulty]
        return random.randint(min_val, max_val)
    
    def get_hint(self, guess: int, target: int) -> str:
        """Provide a hint based on the guess."""
        if guess < target:
            distance = target - guess
            if distance > 20:
                return "Marco! (Way too low)"
            elif distance > 10:
                return "Marco! (Too low)"
            else:
                return "Marco! (A bit low)"
        else:
            distance = guess - target
            if distance > 20:
                return "Marco! (Way too high)"
            elif distance > 10:
                return "Marco! (Too high)"
            else:
                return "Marco! (A bit high)"
    
    def play_round(self) -> bool:
        """Play a single round of Marco Polo."""
        target = self.generate_number()
        min_val, max_val = self.difficulty_ranges[self.difficulty]
        
        if self.verbose:
            print(f"\n[DEBUG] Target number: {target}")
            
        print(f"\nI'm thinking of a number between {min_val} and {max_val}...")
        print("When you guess, I'll say 'Marco!' and give you a hint.")
        print("Find the number to hear 'Polo!'")
        
        round_attempts = 0
        while True:
            try:
                guess_str = input("\nYour guess: ")
                guess = int(guess_str)
                
                if guess < min_val or guess > max_val:
                    print(f"Please guess between {min_val} and {max_val}")
                    continue
                    
                round_attempts += 1
                self.attempts += 1
                
                if guess == target:
                    print("🎉 Polo! You found it!")
                    self.score += 1
                    if self.verbose:
                        print(f"[DEBUG] Attempts this round: {round_attempts}")
                    return True
                else:
                    hint = self.get_hint(guess, target)
                    print(hint)
                    
            except ValueError:
                print("Please enter a valid number!")
            except KeyboardInterrupt:
                print("\n\nGame interrupted!")
                return False
    
    def calculate_performance(self) -> str:
        """Calculate performance rating."""
        if self.attempts == 0:
            return "No games played"
            
        avg_attempts = self.attempts / max(self.score, 1)
        
        if avg_attempts <= 5:
            return "🏆 Expert"
        elif avg_attempts <= 7:
            return "⭐ Good"
        elif avg_attempts <= 10:
            return "👍 Average"
        else:
            return "🎯 Keep practicing"


def parse_arguments() -> argparse.Namespace:
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Marco Polo CLI - A number guessing game",
        epilog="Example: marco_polo --rounds 5 --difficulty medium"
    )
    
    parser.add_argument(
        "-r", "--rounds",
        type=int,
        default=3,
        help="Number of rounds to play (default: 3)"
    )
    
    parser.add_argument(
        "-d", "--difficulty",
        choices=["easy", "medium", "hard"],
        default="medium",
        help="Game difficulty (default: medium)"
    )
    
    parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="Enable verbose output"
    )
    
    parser.add_argument(
        "--version",
        action="version",
        version="Marco Polo CLI v1.0.0"
    )
    
    return parser.parse_args()


def print_welcome() -> None:
    """Print welcome banner."""
    print("=" * 50)
    print("🎮 Welcome to Marco Polo CLI! 🎮".center(50))
    print("=" * 50)


def print_statistics(game: MarcoPoloGame, rounds: int) -> None:
    """Print game statistics."""
    print("\n" + "=" * 50)
    print("📊 Game Statistics 📊".center(50))
    print("=" * 50)
    print(f"Rounds played: {game.score}/{rounds}")
    print(f"Total attempts: {game.attempts}")
    if game.score > 0:
        print(f"Average attempts per round: {game.attempts/game.score:.1f}")
    print(f"Performance: {game.calculate_performance()}")
    print("=" * 50)


def main() -> int:
    """Main entry point."""
    args = parse_arguments()
    
    print_welcome()
    
    game = MarcoPoloGame(
        difficulty=args.difficulty,
        verbose=args.verbose
    )
    
    print(f"\nStarting {args.rounds} rounds on {args.difficulty} difficulty...")
    
    for round_num in range(1, args.rounds + 1):
        print(f"\n{'='*30}")
        print(f"Round {round_num} of {args.rounds}")
        print(f"{'='*30}")
        
        if not game.play_round():
            print("\nGame ended early.")
            break
    
    print_statistics(game, args.rounds)
    print("\nThanks for playing! 👋")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())