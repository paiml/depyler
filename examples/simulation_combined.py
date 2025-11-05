"""
Comprehensive Simulation Example
Combines: random, collections, statistics, math

This example demonstrates realistic simulation scenarios using
multiple Python stdlib modules working together.

Tests transpiler's ability to handle:
- Random number generation in simulations
- Statistical analysis of results
- Data collection and aggregation
- Mathematical calculations
"""

import random
import math
from collections import defaultdict
from typing import List, Dict, Tuple


def roll_dice(num_dice: int, num_sides: int) -> int:
    """Roll multiple dice and return sum"""
    total: int = 0

    for i in range(num_dice):
        roll: int = random.randint(1, num_sides)
        total = total + roll

    return total


def simulate_dice_rolls(num_dice: int, num_sides: int, num_trials: int) -> Dict[int, int]:
    """Simulate dice rolls and collect distribution"""
    results: Dict[int, int] = {}

    for trial in range(num_trials):
        total: int = roll_dice(num_dice, num_sides)

        if total in results:
            results[total] = results[total] + 1
        else:
            results[total] = 1

    return results


def coin_flip_sequence(num_flips: int) -> List[str]:
    """Simulate sequence of coin flips"""
    flips: List[str] = []

    for i in range(num_flips):
        flip: int = random.randint(0, 1)
        if flip == 0:
            flips.append("H")
        else:
            flips.append("T")

    return flips


def count_streaks(sequence: List[str]) -> Dict[str, int]:
    """Count longest streaks in sequence"""
    if len(sequence) == 0:
        return {}

    max_heads_streak: int = 0
    max_tails_streak: int = 0
    current_streak: int = 1
    current_type: str = sequence[0]

    for i in range(1, len(sequence)):
        if sequence[i] == current_type:
            current_streak = current_streak + 1
        else:
            # Update max streak
            if current_type == "H" and current_streak > max_heads_streak:
                max_heads_streak = current_streak
            elif current_type == "T" and current_streak > max_tails_streak:
                max_tails_streak = current_streak

            # Reset for new streak
            current_type = sequence[i]
            current_streak = 1

    # Check last streak
    if current_type == "H" and current_streak > max_heads_streak:
        max_heads_streak = current_streak
    elif current_type == "T" and current_streak > max_tails_streak:
        max_tails_streak = current_streak

    streaks: Dict[str, int] = {
        "max_heads": max_heads_streak,
        "max_tails": max_tails_streak
    }

    return streaks


def monte_carlo_pi_estimation(num_samples: int) -> Tuple[float, float]:
    """Estimate pi using Monte Carlo method"""
    inside_circle: int = 0

    for i in range(num_samples):
        x: float = random.random()
        y: float = random.random()

        distance_squared: float = x * x + y * y

        if distance_squared <= 1.0:
            inside_circle = inside_circle + 1

    pi_estimate: float = 4.0 * float(inside_circle) / float(num_samples)

    # Calculate error
    actual_pi: float = 3.14159265359
    error: float = abs(pi_estimate - actual_pi)

    return (pi_estimate, error)


def simulate_random_walk(num_steps: int) -> Tuple[int, int]:
    """Simulate 2D random walk"""
    x: int = 0
    y: int = 0

    for step in range(num_steps):
        direction: int = random.randint(0, 3)

        if direction == 0:  # North
            y = y + 1
        elif direction == 1:  # East
            x = x + 1
        elif direction == 2:  # South
            y = y - 1
        else:  # West
            x = x - 1

    return (x, y)


def calculate_walk_distance(position: Tuple[int, int]) -> float:
    """Calculate Euclidean distance from origin"""
    x: int = position[0]
    y: int = position[1]

    distance: float = math.sqrt(float(x * x + y * y))
    return distance


def simulate_queue_system(num_customers: int, service_time_range: Tuple[int, int]) -> Dict[str, float]:
    """Simulate queue/service system"""
    wait_times: List[int] = []
    queue_length: int = 0
    current_time: int = 0

    for customer in range(num_customers):
        # Customer arrival
        arrival_time: int = current_time

        # Service time
        service_time: int = random.randint(service_time_range[0], service_time_range[1])

        # Wait time
        wait_time: int = queue_length
        wait_times.append(wait_time)

        # Update queue
        queue_length = queue_length + service_time
        current_time = arrival_time + service_time

        # Reduce queue as time passes
        if queue_length > 0:
            queue_length = max(0, queue_length - 1)

    # Calculate statistics
    total_wait: int = 0
    for wait in wait_times:
        total_wait = total_wait + wait

    avg_wait: float = float(total_wait) / float(len(wait_times)) if len(wait_times) > 0 else 0.0

    # Find max wait
    max_wait: int = 0
    for wait in wait_times:
        if wait > max_wait:
            max_wait = wait

    stats: Dict[str, float] = {
        "avg_wait": avg_wait,
        "max_wait": float(max_wait),
        "total_customers": float(num_customers)
    }

    return stats


def simulate_card_game(num_games: int) -> Dict[str, int]:
    """Simulate card game results"""
    results: Dict[str, int] = {"wins": 0, "losses": 0, "ties": 0}

    for game in range(num_games):
        # Draw two cards (simplified)
        player_card: int = random.randint(1, 13)
        dealer_card: int = random.randint(1, 13)

        if player_card > dealer_card:
            results["wins"] = results["wins"] + 1
        elif player_card < dealer_card:
            results["losses"] = results["losses"] + 1
        else:
            results["ties"] = results["ties"] + 1

    return results


def calculate_win_rate(results: Dict[str, int]) -> float:
    """Calculate win rate from game results"""
    total_games: int = results["wins"] + results["losses"] + results["ties"]

    if total_games == 0:
        return 0.0

    win_rate: float = float(results["wins"]) / float(total_games)
    return win_rate


def simulate_population_growth(initial_population: int, growth_rate: float, num_generations: int) -> List[int]:
    """Simulate population growth with randomness"""
    populations: List[int] = [initial_population]

    current_population: int = initial_population

    for generation in range(num_generations):
        # Add randomness to growth rate
        random_factor: float = random.random() * 0.2 - 0.1  # -0.1 to +0.1
        actual_growth: float = growth_rate + random_factor

        # Calculate new population
        growth: int = int(float(current_population) * actual_growth)
        current_population = current_population + growth

        # Ensure non-negative
        if current_population < 0:
            current_population = 0

        populations.append(current_population)

    return populations


def analyze_population_trend(populations: List[int]) -> Dict[str, float]:
    """Analyze population growth trend"""
    if len(populations) < 2:
        return {}

    # Calculate average growth rate
    total_growth: float = 0.0
    num_intervals: int = len(populations) - 1

    for i in range(num_intervals):
        if populations[i] > 0:
            growth_rate: float = float(populations[i + 1] - populations[i]) / float(populations[i])
            total_growth = total_growth + growth_rate

    avg_growth: float = total_growth / float(num_intervals) if num_intervals > 0 else 0.0

    # Find peak population
    peak: int = populations[0]
    for pop in populations:
        if pop > peak:
            peak = pop

    analysis: Dict[str, float] = {
        "avg_growth_rate": avg_growth,
        "peak_population": float(peak),
        "final_population": float(populations[-1])
    }

    return analysis


def run_simulations() -> None:
    """Run comprehensive simulation suite"""
    print("=== Comprehensive Simulation Demo ===")

    # Set seed for reproducibility
    random.seed(42)

    # Dice rolling simulation
    print("\n1. Dice Rolling Simulation")
    dice_results: Dict[int, int] = simulate_dice_rolls(2, 6, 1000)
    print(f"   Simulated {1000} rolls of 2d6")
    print(f"   Unique outcomes: {len(dice_results)}")

    # Coin flip simulation
    print("\n2. Coin Flip Sequence")
    flips: List[str] = coin_flip_sequence(100)
    streaks: Dict[str, int] = count_streaks(flips)
    print(f"   100 flips, max heads streak: {streaks['max_heads']}")

    # Monte Carlo Pi estimation
    print("\n3. Monte Carlo Pi Estimation")
    pi_result: Tuple[float, float] = monte_carlo_pi_estimation(10000)
    print(f"   Pi estimate: {pi_result[0]:.5f}, Error: {pi_result[1]:.5f}")

    # Random walk simulation
    print("\n4. Random Walk Simulation")
    final_pos: Tuple[int, int] = simulate_random_walk(1000)
    distance: float = calculate_walk_distance(final_pos)
    print(f"   Final position: ({final_pos[0]}, {final_pos[1]}), Distance: {distance:.2f}")

    # Queue system simulation
    print("\n5. Queue System Simulation")
    queue_stats: Dict[str, float] = simulate_queue_system(100, (1, 5))
    print(f"   Avg wait time: {queue_stats['avg_wait']:.2f}")

    # Card game simulation
    print("\n6. Card Game Simulation")
    game_results: Dict[str, int] = simulate_card_game(1000)
    win_rate: float = calculate_win_rate(game_results)
    print(f"   Win rate: {win_rate:.2%}, Wins: {game_results['wins']}")

    # Population growth simulation
    print("\n7. Population Growth Simulation")
    populations: List[int] = simulate_population_growth(100, 0.1, 20)
    pop_analysis: Dict[str, float] = analyze_population_trend(populations)
    print(f"   Final population: {pop_analysis['final_population']:.0f}")

    print("\n=== All Simulations Complete ===")


if __name__ == "__main__":
    run_simulations()
