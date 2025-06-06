#!/usr/bin/env python3
"""
Marco Polo CLI - Simplified version for current Depyler capabilities.

This version demonstrates what Depyler can transpile today:
- Simple functions with type annotations
- Basic arithmetic and comparisons
- String operations
- Integer operations
"""

# @depyler: optimization_level = "standard"
# @depyler: string_strategy = "always_owned"
def generate_number(min_val: int, max_val: int) -> int:
    """Generate a number in range (simplified without random)."""
    # For demo purposes, return middle value
    return (min_val + max_val) // 2


# @depyler: string_strategy = "always_owned"
def get_hint(guess: int, target: int) -> str:
    """Provide a hint based on the guess."""
    if guess < target:
        return "Marco! (Too low)"
    elif guess > target:
        return "Marco! (Too high)"
    else:
        return "Polo!"


# @depyler: bounds_checking = "explicit"
def calculate_score(attempts: int, rounds: int) -> int:
    """Calculate final score."""
    if rounds == 0:
        return 0
    base_score = 100 * rounds
    penalty = attempts * 5
    score = base_score - penalty
    if score < 0:
        return 0
    return score


# @depyler: string_strategy = "zero_copy"
# @depyler: ownership = "borrowed"
def get_difficulty_name(level: int) -> str:
    """Get difficulty name from level."""
    if level == 1:
        return "Easy"
    elif level == 2:
        return "Medium"
    elif level == 3:
        return "Hard"
    else:
        return "Unknown"


# @depyler: optimization_level = "aggressive"
def calculate_average(total: int, count: int) -> float:
    """Calculate average with safety check."""
    if count == 0:
        return 0.0
    return total / count


# @depyler: string_strategy = "always_owned"
def format_statistics(score: int, attempts: int, rounds: int) -> str:
    """Format game statistics as string."""
    avg = calculate_average(attempts, rounds)
    result = "Game Statistics:\n"
    result = result + "Score: " + str(score) + "\n"
    result = result + "Attempts: " + str(attempts) + "\n"
    result = result + "Average: " + str(avg) + "\n"
    return result


# @depyler: bounds_checking = "explicit"
def validate_guess(guess: int, min_val: int, max_val: int) -> bool:
    """Check if guess is in valid range."""
    if guess < min_val:
        return False
    if guess > max_val:
        return False
    return True


# @depyler: optimization_level = "standard"
def play_simple_round(target: int, max_attempts: int) -> int:
    """Simulate a round with fixed guesses."""
    attempts = 0
    guess = 50  # Start with middle guess
    
    while attempts < max_attempts:
        attempts = attempts + 1
        
        if guess == target:
            return attempts
        elif guess < target:
            guess = guess + 10
        else:
            guess = guess - 5
    
    return attempts