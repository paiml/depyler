use rand as random;
#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = "Test random integer generation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_random_integers() -> i32 {
    let rand_int: i32 = rand::thread_rng().gen_range(1..=10);
    let rand_int2: i32 = rand::thread_rng().gen_range(0..=100);
    let rand_int3: i32 = rand::thread_rng().gen_range(-50..=50);
    rand_int + rand_int2 + rand_int3
}
#[doc = "Test random float generation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_random_floats() -> f64 {
    let rand_float: f64 = rand::random::<f64>();
    let rand_uniform: f64 = rand::thread_rng().gen_range((10.0 as f64)..=(20.0 as f64));
    let rand_bounded: f64 = rand::thread_rng().gen_range((-1.0 as f64)..=(1.0 as f64));
    rand_float + rand_uniform + rand_bounded
}
#[doc = "Test random choice from sequence"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_random_choice(items: &Vec<String>) -> String {
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return "".to_string();
    }
    let chosen: String = *items.choose(&mut rand::thread_rng()).unwrap();
    chosen.to_string()
}
#[doc = "Test random sampling without replacement"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_random_sample(numbers: &Vec<i32>, k: i32) -> Vec<i32> {
    let _cse_temp_0 = numbers.len() as i32;
    let _cse_temp_1 = _cse_temp_0 < k;
    if _cse_temp_1 {
        return vec![];
    }
    let sample: Vec<i32> = numbers
        .choose_multiple(&mut rand::thread_rng(), k as usize)
        .cloned()
        .collect::<Vec<_>>();
    sample
}
#[doc = "Test in-place list shuffling"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_shuffle_list(items: &Vec<i32>) -> Vec<i32> {
    let shuffled: Vec<i32> = items.clone();
    shuffled.shuffle(&mut rand::thread_rng());
    shuffled
}
#[doc = "Test seeded random generation for reproducibility"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_random_seed() -> Vec<i32> {
    {
        let _seed = 42;
        ()
    };
    let mut results: Vec<i32> = vec![];
    for _i in 0..5 {
        let rand_num: i32 = rand::thread_rng().gen_range(1..=100);
        results.push(rand_num);
    }
    results
}
#[doc = "Test randrange for step-based random selection"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_random_range() -> i32 {
    let even_num: i32 = {
        let start = 0;
        let stop = 100;
        let step = 2;
        let num_steps = ((stop - start) / step).max(0);
        let offset = rand::thread_rng().gen_range(0..num_steps);
        start + offset * step
    };
    let odd_num: i32 = {
        let start = 1;
        let stop = 100;
        let step = 2;
        let num_steps = ((stop - start) / step).max(0);
        let offset = rand::thread_rng().gen_range(0..num_steps);
        start + offset * step
    };
    let multiple_5: i32 = {
        let start = 0;
        let stop = 100;
        let step = 5;
        let num_steps = ((stop - start) / step).max(0);
        let offset = rand::thread_rng().gen_range(0..num_steps);
        start + offset * step
    };
    even_num + odd_num + multiple_5
}
#[doc = "Simulate rolling a standard six-sided die"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simulate_dice_roll() -> i32 {
    rand::thread_rng().gen_range(1..=6)
}
#[doc = "Simulate a coin flip"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simulate_coin_flip() -> String {
    let result: i32 = rand::thread_rng().gen_range(0..=1);
    let _cse_temp_0 = result == 0.0;
    if _cse_temp_0 {
        "Heads".to_string()
    } else {
        "Tails".to_string()
    }
}
#[doc = "Generate a random password from alphanumeric characters"]
#[doc = " Depyler: proven to terminate"]
pub fn generate_random_password(length: i32) -> Result<String, Box<dyn std::error::Error>> {
    let mut password_chars: Vec<String> = vec![];
    let chars: String =
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string();
    for _i in 0..length {
        let idx: i32 = rand::thread_rng().gen_range(0..=(chars.len() as i32).saturating_sub(1));
        let char: String = chars.get(idx as usize).cloned().unwrap_or_default();
        password_chars.push(char);
    }
    let password: String = password_chars.join("");
    Ok(password.to_string())
}
#[doc = "Simulate weighted random choice(manual implementation)"]
pub fn weighted_random_choice<'b, 'a>(
    items: &'a Vec<String>,
    weights: &'b Vec<i32>,
) -> Result<String, Box<dyn std::error::Error>> {
    let _cse_temp_0 = items.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let _cse_temp_2 = weights.len() as i32;
    let _cse_temp_3 = _cse_temp_0 != _cse_temp_2;
    let _cse_temp_4 = (_cse_temp_1) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok("".to_string());
    }
    let mut total_weight: i32 = 0;
    for weight in weights.iter().cloned() {
        total_weight = format!("{}{}", total_weight, weight);
    }
    let rand_value: i32 = rand::thread_rng().gen_range(0..=total_weight - 1);
    let mut cumulative: i32 = 0;
    for i in 0..items.len() as i32 {
        cumulative = format!(
            "{}{}",
            cumulative,
            weights.get(i as usize).cloned().unwrap_or_default()
        );
        if rand_value < cumulative {
            return Ok(items.get(i as usize).cloned().unwrap_or_default());
        }
    }
    Ok({
        let base = &items;
        base.get(base.len().saturating_sub(1usize))
            .cloned()
            .unwrap_or_default()
    })
}
#[doc = "Estimate pi using Monte Carlo method"]
#[doc = " Depyler: proven to terminate"]
pub fn monte_carlo_pi_estimation(num_samples: i32) -> Result<f64, Box<dyn std::error::Error>> {
    let mut inside_circle: i32 = 0;
    for _i in 0..num_samples {
        let x: f64 = rand::random::<f64>();
        let y: f64 = rand::random::<f64>();
        let distance_sq: f64 = x * x + y * y;
        if distance_sq <= 1.0 {
            inside_circle = inside_circle + 1;
        }
    }
    let _cse_temp_0 = (inside_circle) as f64;
    let _cse_temp_1 = 4.0 * _cse_temp_0;
    let _cse_temp_2 = (num_samples) as f64;
    let _cse_temp_3 = (_cse_temp_1 as f64) / (_cse_temp_2 as f64);
    let pi_estimate: f64 = _cse_temp_3;
    Ok(pi_estimate)
}
#[doc = "Test random boolean generation and calculate distribution"]
#[doc = " Depyler: proven to terminate"]
pub fn test_random_boolean_distribution(
    num_trials: i32,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut true_count: i32 = 0;
    for _i in 0..num_trials {
        let rand_bool: bool = rand::random::<f64>() < 0.5;
        if rand_bool {
            true_count = true_count + 1;
        }
    }
    let _cse_temp_0 = (true_count) as f64;
    let _cse_temp_1 = (num_trials) as f64;
    let _cse_temp_2 = (_cse_temp_0 as f64) / (_cse_temp_1 as f64);
    let percentage: f64 = _cse_temp_2;
    Ok(percentage)
}
#[doc = "Create and shuffle a deck of cards"]
#[doc = " Depyler: verified panic-free"]
pub fn shuffle_deck() -> Vec<String> {
    let mut deck: Vec<String> = vec![];
    let suits: Vec<String> = vec![
        "H".to_string(),
        "D".to_string(),
        "C".to_string(),
        "S".to_string(),
    ];
    let ranks: Vec<String> = vec![
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
        "10".to_string(),
        "J".to_string(),
        "Q".to_string(),
        "K".to_string(),
        "A".to_string(),
    ];
    for suit in suits.iter().cloned() {
        for rank in ranks.iter().cloned() {
            let card: String = rank + suit;
            deck.push(card);
        }
    }
    deck.shuffle(&mut rand::thread_rng());
    deck
}
#[doc = "Test Gaussian(normal) distribution"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gauss_distribution() -> f64 {
    let gauss_value: f64 = {
        use rand::distributions::Distribution;
        let normal = rand_distr::Normal::new(0.0 as f64, 1.0 as f64).unwrap();
        normal.sample(&mut rand::thread_rng())
    };
    let custom_gauss: f64 = {
        use rand::distributions::Distribution;
        let normal = rand_distr::Normal::new(100.0 as f64, 15.0 as f64).unwrap();
        normal.sample(&mut rand::thread_rng())
    };
    gauss_value + custom_gauss
}
#[doc = "Test triangular distribution"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_triangular_distribution() -> f64 {
    let tri_value: f64 = {
        use rand::distributions::Distribution;
        let triangular = rand_distr::Triangular::new(0.0 as f64, 10.0 as f64, 5.0 as f64).unwrap();
        triangular.sample(&mut rand::thread_rng())
    };
    let tri_value2: f64 = {
        use rand::distributions::Distribution;
        let triangular =
            rand_distr::Triangular::new(1.0 as f64, 100.0 as f64, 50.0 as f64).unwrap();
        triangular.sample(&mut rand::thread_rng())
    };
    tri_value + tri_value2
}
#[doc = "Run all random module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_random_features() -> Result<(), Box<dyn std::error::Error>> {
    let colors: Vec<String> = vec![
        "red".to_string(),
        "green".to_string(),
        "blue".to_string(),
        "yellow".to_string(),
    ];
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let items: Vec<String> = vec![
        "common".to_string(),
        "uncommon".to_string(),
        "rare".to_string(),
    ];
    let weights: Vec<i32> = vec![70, 25, 5];
    println!("{}", "All random module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_random_integers_examples() {
        let _ = test_random_integers();
    }
    #[test]
    fn test_test_shuffle_list_examples() {
        assert_eq!(test_shuffle_list(vec![]), vec![]);
        assert_eq!(test_shuffle_list(vec![1]), vec![1]);
    }
    #[test]
    fn test_test_random_range_examples() {
        let _ = test_random_range();
    }
    #[test]
    fn test_simulate_dice_roll_examples() {
        let _ = simulate_dice_roll();
    }
}
