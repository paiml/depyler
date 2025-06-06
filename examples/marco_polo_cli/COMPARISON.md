# Python to Rust: Side-by-Side Comparison

## CLI Argument Parsing

### Python (argparse)
```python
def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Marco Polo CLI - A number guessing game"
    )
    parser.add_argument("-r", "--rounds", type=int, default=3)
    parser.add_argument("-d", "--difficulty", 
                       choices=["easy", "medium", "hard"])
    return parser.parse_args()
```

### Rust (clap)
```rust
#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    rounds: u32,
    
    #[arg(short, long, value_enum)]
    difficulty: Difficulty,
}
```

## Game State Management

### Python (Class)
```python
class MarcoPoloGame:
    def __init__(self, difficulty: str, verbose: bool):
        self.difficulty = difficulty
        self.verbose = verbose
        self.score = 0
        self.attempts = 0
```

### Rust (Struct)
```rust
struct MarcoPoloGame {
    difficulty: Difficulty,
    verbose: bool,
    score: u32,
    attempts: u32,
}
```

## Error Handling

### Python (Try/Except)
```python
try:
    guess = int(input("Your guess: "))
except ValueError:
    print("Please enter a valid number!")
```

### Rust (Result<T, E>)
```rust
let guess = match input.trim().parse::<i32>() {
    Ok(num) => num,
    Err(_) => {
        println!("Please enter a valid number!");
        continue;
    }
};
```

## String Operations

### Python
```python
# Always creates new strings
hint = f"Marco! ({description})"
result = "Score: " + str(score)
```

### Rust
```rust
// Can use references for efficiency
let hint: &'static str = "Marco! (Too low)";
// Or owned strings when needed
let result = format!("Score: {}", score);
```

## Pattern Matching

### Python (If/Elif)
```python
if guess < target:
    return "Too low"
elif guess > target:
    return "Too high"
else:
    return "Correct!"
```

### Rust (Match)
```rust
match guess.cmp(&target) {
    Ordering::Less => "Too low",
    Ordering::Greater => "Too high",
    Ordering::Equal => "Correct!",
}
```

## Memory Efficiency

| Aspect | Python | Rust |
|--------|--------|------|
| String allocation | Always heap | Stack when possible |
| Integer size | Dynamic | Fixed (i32/u32) |
| Collections | Reference counted | Owned by default |
| Function calls | Stack + heap | Stack only |

## Depyler Annotations Impact

### String Strategy
```python
# @depyler: string_strategy = "zero_copy"
def get_message() -> str:
    return "Hello"  # → &'static str in Rust
```

### Ownership Model
```python
# @depyler: ownership = "borrowed"
def process(data: List[int]) -> int:
    return sum(data)  # → &[i32] in Rust
```

### Error Handling
```python
# @depyler: error_strategy = "result_type"
def safe_divide(a: int, b: int) -> Optional[float]:
    # → Result<f64, Error> in Rust
```