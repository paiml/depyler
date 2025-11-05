"""
Comprehensive test of Python random module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's random module
functions to their Rust equivalents (rand crate).

Expected Rust mappings:
- random.randint(a, b) -> rng.gen_range(a..=b)
- random.random() -> rng.gen::<f64>()
- random.choice(seq) -> seq[rng.gen_range(0..seq.len())]
- random.shuffle(seq) -> seq.shuffle(&mut rng)
- random.seed(n) -> StdRng::seed_from_u64(n)

Note: This tests the transpiler's ability to recognize and translate
random module patterns. Actual random number generation may require
the rand crate in Rust.
"""

import random
from typing import List


def test_random_integers() -> int:
    """Test random integer generation"""
    # Random integer in range [1, 10]
    rand_int: int = random.randint(1, 10)

    # Random integer in range [0, 100]
    rand_int2: int = random.randint(0, 100)

    # Random integer in negative range
    rand_int3: int = random.randint(-50, 50)

    return rand_int + rand_int2 + rand_int3


def test_random_floats() -> float:
    """Test random float generation"""
    # Random float in [0.0, 1.0)
    rand_float: float = random.random()

    # Random float in custom range using uniform
    rand_uniform: float = random.uniform(10.0, 20.0)

    # Random float with specific bounds
    rand_bounded: float = random.uniform(-1.0, 1.0)

    return rand_float + rand_uniform + rand_bounded


def test_random_choice(items: List[str]) -> str:
    """Test random choice from sequence"""
    if len(items) == 0:
        return ""

    # Choose one random item
    chosen: str = random.choice(items)

    return chosen


def test_random_sample(numbers: List[int], k: int) -> List[int]:
    """Test random sampling without replacement"""
    if len(numbers) < k:
        return []

    # Get k random items without replacement
    sample: List[int] = random.sample(numbers, k)

    return sample


def test_shuffle_list(items: List[int]) -> List[int]:
    """Test in-place list shuffling"""
    # Make a copy to avoid modifying original
    shuffled: List[int] = items.copy()

    # Shuffle in place
    random.shuffle(shuffled)

    return shuffled


def test_random_seed() -> List[int]:
    """Test seeded random generation for reproducibility"""
    # Set seed for reproducible results
    random.seed(42)

    results: List[int] = []

    # Generate sequence of random numbers
    for i in range(5):
        rand_num: int = random.randint(1, 100)
        results.append(rand_num)

    return results


def test_random_range() -> int:
    """Test randrange for step-based random selection"""
    # Random even number between 0 and 100
    even_num: int = random.randrange(0, 100, 2)

    # Random odd number between 1 and 100
    odd_num: int = random.randrange(1, 100, 2)

    # Random multiple of 5
    multiple_5: int = random.randrange(0, 100, 5)

    return even_num + odd_num + multiple_5


def simulate_dice_roll() -> int:
    """Simulate rolling a standard six-sided die"""
    return random.randint(1, 6)


def simulate_coin_flip() -> str:
    """Simulate a coin flip"""
    result: int = random.randint(0, 1)

    if result == 0:
        return "Heads"
    else:
        return "Tails"


def generate_random_password(length: int) -> str:
    """Generate a random password from alphanumeric characters"""
    # Simplified - using numbers only for type safety
    # In real implementation would use string characters
    password_chars: List[str] = []

    chars: str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"

    for i in range(length):
        # Get random index
        idx: int = random.randint(0, len(chars) - 1)
        char: str = chars[idx]
        password_chars.append(char)

    # Join characters
    password: str = "".join(password_chars)
    return password


def weighted_random_choice(items: List[str], weights: List[int]) -> str:
    """Simulate weighted random choice (manual implementation)"""
    if len(items) == 0 or len(items) != len(weights):
        return ""

    # Calculate total weight
    total_weight: int = 0
    for weight in weights:
        total_weight = total_weight + weight

    # Generate random number in range
    rand_value: int = random.randint(0, total_weight - 1)

    # Find which item corresponds to this value
    cumulative: int = 0
    for i in range(len(items)):
        cumulative = cumulative + weights[i]
        if rand_value < cumulative:
            return items[i]

    # Fallback
    return items[-1]


def monte_carlo_pi_estimation(num_samples: int) -> float:
    """Estimate pi using Monte Carlo method"""
    inside_circle: int = 0

    for i in range(num_samples):
        # Generate random point in [0, 1] x [0, 1]
        x: float = random.random()
        y: float = random.random()

        # Check if point is inside unit circle
        distance_sq: float = x * x + y * y

        if distance_sq <= 1.0:
            inside_circle = inside_circle + 1

    # Pi ≈ 4 * (points inside circle / total points)
    pi_estimate: float = 4.0 * float(inside_circle) / float(num_samples)

    return pi_estimate


def test_random_boolean_distribution(num_trials: int) -> float:
    """Test random boolean generation and calculate distribution"""
    true_count: int = 0

    for i in range(num_trials):
        # Generate random boolean
        rand_bool: bool = random.random() < 0.5

        if rand_bool:
            true_count = true_count + 1

    # Calculate percentage
    percentage: float = float(true_count) / float(num_trials)

    return percentage


def shuffle_deck() -> List[str]:
    """Create and shuffle a deck of cards"""
    # Simplified deck with just numbers
    deck: List[str] = []

    suits: List[str] = ["H", "D", "C", "S"]
    ranks: List[str] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"]

    for suit in suits:
        for rank in ranks:
            card: str = rank + suit
            deck.append(card)

    # Shuffle the deck
    random.shuffle(deck)

    return deck


def test_gauss_distribution() -> float:
    """Test Gaussian (normal) distribution"""
    # Generate random number from normal distribution
    # Mean = 0, Standard deviation = 1
    gauss_value: float = random.gauss(0.0, 1.0)

    # Generate with custom mean and std dev
    custom_gauss: float = random.gauss(100.0, 15.0)

    return gauss_value + custom_gauss


def test_triangular_distribution() -> float:
    """Test triangular distribution"""
    # Triangular distribution with low=0, high=10, mode=5
    tri_value: float = random.triangular(0.0, 10.0, 5.0)

    # Another triangular distribution
    tri_value2: float = random.triangular(1.0, 100.0, 50.0)

    return tri_value + tri_value2


def test_all_random_features() -> None:
    """Run all random module tests"""
    # Integer tests
    int_result: int = test_random_integers()

    # Float tests
    float_result: float = test_random_floats()

    # Choice tests
    colors: List[str] = ["red", "green", "blue", "yellow"]
    chosen_color: str = test_random_choice(colors)

    # Sample tests
    numbers: List[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sampled: List[int] = test_random_sample(numbers, 3)

    # Shuffle tests
    shuffled: List[int] = test_shuffle_list(numbers)

    # Seed tests
    seeded_results: List[int] = test_random_seed()

    # Range tests
    range_result: int = test_random_range()

    # Simulation tests
    dice: int = simulate_dice_roll()
    coin: str = simulate_coin_flip()

    # Password generation
    password: str = generate_random_password(8)

    # Weighted choice
    items: List[str] = ["common", "uncommon", "rare"]
    weights: List[int] = [70, 25, 5]
    weighted: str = weighted_random_choice(items, weights)

    # Monte Carlo estimation
    pi_est: float = monte_carlo_pi_estimation(1000)

    # Boolean distribution
    bool_dist: float = test_random_boolean_distribution(100)

    # Deck shuffling
    deck: List[str] = shuffle_deck()

    # Distribution tests
    gauss_result: float = test_gauss_distribution()
    tri_result: float = test_triangular_distribution()

    print("All random module tests completed successfully")
