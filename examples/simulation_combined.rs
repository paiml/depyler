use rand as random;
use std::collections::HashMap;
use std::f64 as math;
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
#[doc = "Roll multiple dice and return sum"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn roll_dice(num_dice: i32, num_sides: i32) -> i32 {
    let mut total: i32 = 0;
    for _i in 0..num_dice {
        let roll: i32 = rand::thread_rng().gen_range(1..=num_sides);
        total = total + roll;
    }
    total
}
#[doc = "Simulate dice rolls and collect distribution"]
#[doc = " Depyler: proven to terminate"]
pub fn simulate_dice_rolls(
    num_dice: i32,
    num_sides: i32,
    num_trials: i32,
) -> Result<HashMap<i32, i32>, IndexError> {
    let mut results: HashMap<i32, i32> = {
        let map = HashMap::new();
        map
    };
    for _trial in 0..num_trials {
        let mut total: i32 = roll_dice(num_dice, num_sides)?;
        if results.contains_key(&total) {
            {
                let _key = total;
                let _old_val = results.get(&_key).cloned().unwrap_or_default();
                results.insert(_key, _old_val + 1);
            }
        } else {
            results.insert(total, 1);
        }
    }
    Ok(results)
}
#[doc = "Simulate sequence of coin flips"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn coin_flip_sequence(num_flips: i32) -> Vec<String> {
    let mut flips: Vec<String> = vec![];
    for _i in 0..num_flips {
        let flip: i32 = rand::thread_rng().gen_range(0..=1);
        if flip == 0 {
            flips.push("H");
        } else {
            flips.push("T");
        }
    }
    flips
}
#[doc = "Count longest streaks in sequence"]
#[doc = " Depyler: proven to terminate"]
pub fn count_streaks(sequence: &Vec<String>) -> Result<HashMap<String, i32>, IndexError> {
    let _cse_temp_0 = sequence.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok({
            let map = HashMap::new();
            map
        });
    }
    let mut max_heads_streak: i32 = 0;
    let mut max_tails_streak: i32 = 0;
    let mut current_streak: i32 = 1;
    let mut current_type: String = sequence.get(0usize).cloned().unwrap_or_default();
    for i in 1..sequence.len() as i32 {
        let mut current_streak;
        if sequence.get(i as usize).cloned().unwrap_or_default() == current_type {
            current_streak = current_streak + 1;
        } else {
            if (current_type == "H") && (current_streak > max_heads_streak) {
                max_heads_streak = current_streak;
            } else {
                if (current_type == "T") && (current_streak > max_tails_streak) {
                    max_tails_streak = current_streak;
                }
            }
            current_type = sequence.get(i as usize).cloned().unwrap_or_default();
            current_streak = 1;
        }
    }
    let _cse_temp_2 = current_type == "H";
    let _cse_temp_3 = current_streak > max_heads_streak;
    let _cse_temp_4 = (_cse_temp_2) && (_cse_temp_3);
    if _cse_temp_4 {
        max_heads_streak = current_streak;
    } else {
        let _cse_temp_5 = current_type == "T";
        let _cse_temp_6 = current_streak > max_tails_streak;
        let _cse_temp_7 = (_cse_temp_5) && (_cse_temp_6);
        if _cse_temp_7 {
            max_tails_streak = current_streak;
        }
    }
    let streaks: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("max_heads".to_string(), max_heads_streak);
        map.insert("max_tails".to_string(), max_tails_streak);
        map
    };
    Ok(streaks)
}
#[doc = "Estimate pi using Monte Carlo method"]
#[doc = " Depyler: proven to terminate"]
pub fn monte_carlo_pi_estimation(num_samples: i32) -> Result<(f64, f64), ZeroDivisionError> {
    let mut inside_circle: i32 = 0;
    for _i in 0..num_samples {
        let x: f64 = rand::random::<f64>();
        let y: f64 = rand::random::<f64>();
        let distance_squared: f64 = x * x + y * y;
        if distance_squared <= 1.0 {
            inside_circle = inside_circle + 1;
        }
    }
    let _cse_temp_0 = (inside_circle) as f64;
    let _cse_temp_1 = 4.0 * _cse_temp_0;
    let _cse_temp_2 = (num_samples) as f64;
    let _cse_temp_3 = (_cse_temp_1 as f64) / (_cse_temp_2 as f64);
    let pi_estimate: f64 = _cse_temp_3;
    let actual_pi: f64 = 3.14159265359;
    let _cse_temp_4 = pi_estimate - actual_pi.abs();
    let error: f64 = _cse_temp_4;
    Ok((pi_estimate, error))
}
#[doc = "Simulate 2D random walk"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simulate_random_walk(num_steps: i32) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for _step in 0..num_steps {
        let direction: i32 = rand::thread_rng().gen_range(0..=3);
        let mut y;
        if direction == 0 {
            y = y + 1;
        } else {
            let mut x;
            if direction == 1 {
                x = x + 1;
            } else {
                if direction == 2 {
                    y = y - 1;
                } else {
                    x = x - 1;
                }
            }
        }
    }
    (x, y)
}
#[doc = "Calculate Euclidean distance from origin"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_walk_distance(position: (i32, i32)) -> Result<f64, IndexError> {
    let mut x: i32 = position.get(0usize).cloned().unwrap_or_default();
    let mut y: i32 = position.get(1usize).cloned().unwrap_or_default();
    let distance: f64 = ((x * x + y * y) as f64 as f64).sqrt();
    Ok(distance)
}
#[doc = "Simulate queue/service system"]
#[doc = " Depyler: verified panic-free"]
pub fn simulate_queue_system(
    num_customers: i32,
    service_time_range: (i32, i32),
) -> HashMap<String, f64> {
    let mut wait_times: Vec<i32> = vec![];
    let mut queue_length: i32 = 0;
    let mut current_time: i32 = 0;
    for _customer in 0..num_customers {
        let arrival_time: i32 = current_time;
        let service_time: i32 = rand::thread_rng().gen_range(
            service_time_range.get(0usize).cloned().unwrap_or_default()
                ..=service_time_range.get(1usize).cloned().unwrap_or_default(),
        );
        let wait_time: i32 = queue_length;
        wait_times.push(wait_time);
        queue_length = queue_length + service_time;
        current_time = arrival_time + service_time;
        if queue_length > 0 {
            queue_length = std::cmp::max(0, queue_length - 1);
        }
    }
    let mut total_wait: i32 = 0;
    for wait in wait_times.iter().cloned() {
        total_wait = total_wait + wait;
    }
    let avg_wait: f64 = if wait_times.len() as i32 > 0 {
        (total_wait) as f64 / (wait_times.len() as i32) as f64
    } else {
        0.0
    };
    let mut max_wait: i32 = 0;
    for wait in wait_times.iter().cloned() {
        if wait > max_wait {
            max_wait = wait;
        }
    }
    let stats: HashMap<String, f64> = {
        let mut map = HashMap::new();
        map.insert("avg_wait".to_string(), avg_wait);
        map.insert("max_wait".to_string(), (max_wait) as f64);
        map.insert("total_customers".to_string(), (num_customers) as f64);
        map
    };
    stats
}
#[doc = "Simulate card game results"]
#[doc = " Depyler: proven to terminate"]
pub fn simulate_card_game(num_games: i32) -> Result<HashMap<String, i32>, IndexError> {
    let mut results: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("wins".to_string(), 0);
        map.insert("losses".to_string(), 0);
        map.insert("ties".to_string(), 0);
        map
    };
    for _game in 0..num_games {
        let player_card: i32 = rand::thread_rng().gen_range(1..=13);
        let dealer_card: i32 = rand::thread_rng().gen_range(1..=13);
        if player_card > dealer_card {
            results.insert("wins", results.get("wins").cloned().unwrap_or_default() + 1);
        } else {
            if player_card < dealer_card {
                results.insert(
                    "losses",
                    results.get("losses").cloned().unwrap_or_default() + 1,
                );
            } else {
                results.insert("ties", results.get("ties").cloned().unwrap_or_default() + 1);
            }
        }
    }
    Ok(results)
}
#[doc = "Calculate win rate from game results"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_win_rate(
    results: &mut HashMap<String, i32>,
) -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = results.get("wins").cloned().unwrap_or_default()
        + results.get("losses").cloned().unwrap_or_default();
    let _cse_temp_1 = _cse_temp_0 + results.get("ties").cloned().unwrap_or_default();
    let total_games: i32 = _cse_temp_1;
    let _cse_temp_2 = total_games == 0;
    if _cse_temp_2 {
        return Ok(0.0);
    }
    let _cse_temp_3 = (results.get("wins").cloned().unwrap_or_default()) as f64;
    let _cse_temp_4 = (total_games) as f64;
    let _cse_temp_5 = (_cse_temp_3 as f64) / (_cse_temp_4 as f64);
    let win_rate: f64 = _cse_temp_5;
    Ok(win_rate)
}
#[doc = "Simulate population growth with randomness"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn simulate_population_growth(
    initial_population: i32,
    growth_rate: f64,
    num_generations: i32,
) -> Vec<i32> {
    let mut populations: Vec<i32> = vec![initial_population];
    let mut current_population: i32 = initial_population;
    for _generation in 0..num_generations {
        let random_factor: f64 = rand::random::<f64>() * 0.2 - 0.1;
        let actual_growth: f64 = growth_rate + random_factor;
        let growth: i32 = ((current_population) as f64 * actual_growth) as i32;
        current_population = current_population + growth;
        if current_population < 0 {
            current_population = 0;
        }
        populations.push(current_population);
    }
    populations
}
#[doc = "Analyze population growth trend"]
pub fn analyze_population_trend(
    populations: &mut Vec<i32>,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = populations.len() as i32;
    let _cse_temp_1 = _cse_temp_0 < 2;
    if _cse_temp_1 {
        return Ok({
            let map = HashMap::new();
            map
        });
    }
    let mut total_growth: f64 = 0.0;
    let num_intervals: i32 = _cse_temp_0 - 1;
    for i in 0..num_intervals {
        if populations.get(i as usize).cloned().unwrap_or_default() > 0 {
            let growth_rate: f64 = ({
                let base = &populations;
                let idx: i32 = i + 1;
                let actual_idx = if idx < 0 {
                    base.len().saturating_sub(idx.abs() as usize)
                } else {
                    idx as usize
                };
                base.get(actual_idx).cloned().unwrap_or_default()
            } - populations.get(i as usize).cloned().unwrap_or_default())
                as f64
                / (populations.get(i as usize).cloned().unwrap_or_default()) as f64;
            total_growth = total_growth + growth_rate;
        }
    }
    let avg_growth: f64 = if num_intervals > 0 {
        total_growth / (num_intervals) as f64
    } else {
        0.0
    };
    let mut peak: i32 = populations.get(0usize).cloned().unwrap_or_default();
    for pop in populations.iter().cloned() {
        if pop > peak {
            peak = pop;
        }
    }
    let analysis: HashMap<String, f64> = {
        let mut map = HashMap::new();
        map.insert("avg_growth_rate".to_string(), avg_growth);
        map.insert("peak_population".to_string(), (peak) as f64);
        map.insert(
            "final_population".to_string(),
            ({
                let base = &populations;
                base.get(base.len().saturating_sub(1usize))
                    .cloned()
                    .unwrap_or_default()
            }) as f64,
        );
        map
    };
    Ok(analysis)
}
#[doc = "Run comprehensive simulation suite"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn run_simulations() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=== Comprehensive Simulation Demo ===");
    {
        let _seed = 42;
        ()
    };
    println!("{}", "\n1. Dice Rolling Simulation");
    let dice_results: HashMap<i32, i32> = simulate_dice_rolls(2, 6, 1000)?;
    println!("{}", format!("   Simulated {:?} rolls of 2d6", 1000));
    println!(
        "{}",
        format!("   Unique outcomes: {:?}", dice_results.len() as i32)
    );
    println!("{}", "\n2. Coin Flip Sequence");
    let mut flips: Vec<String> = coin_flip_sequence(100)?;
    let streaks: HashMap<String, i32> = count_streaks(&flips)?;
    println!(
        "{}",
        format!(
            "   100 flips, max heads streak: {:?}",
            streaks.get("max_heads").cloned().unwrap_or_default()
        )
    );
    println!("{}", "\n3. Monte Carlo Pi Estimation");
    let pi_result: (f64, f64) = monte_carlo_pi_estimation(10000)?;
    println!(
        "{}",
        format!(
            "   Pi estimate: {:?}, Error: {:?}",
            pi_result.get(0usize).cloned().unwrap_or_default(),
            pi_result.get(1usize).cloned().unwrap_or_default()
        )
    );
    println!("{}", "\n4. Random Walk Simulation");
    let final_pos: (i32, i32) = simulate_random_walk(1000)?;
    let distance: f64 = calculate_walk_distance(final_pos)?;
    println!(
        "{}",
        format!(
            "   Final position:({:?}, {:?}), Distance: {:?}",
            final_pos.get(0usize).cloned().unwrap_or_default(),
            final_pos.get(1usize).cloned().unwrap_or_default(),
            distance
        )
    );
    println!("{}", "\n5. Queue System Simulation");
    let queue_stats: HashMap<String, f64> = simulate_queue_system(100, (1, 5))?;
    println!(
        "{}",
        format!(
            "   Avg wait time: {:?}",
            queue_stats.get("avg_wait").cloned().unwrap_or_default()
        )
    );
    println!("{}", "\n6. Card Game Simulation");
    let game_results: HashMap<String, i32> = simulate_card_game(1000)?;
    let win_rate: f64 = calculate_win_rate(&game_results)?;
    println!(
        "{}",
        format!(
            "   Win rate: {:?}, Wins: {:?}",
            win_rate,
            game_results.get("wins").cloned().unwrap_or_default()
        )
    );
    println!("{}", "\n7. Population Growth Simulation");
    let mut populations: Vec<i32> = simulate_population_growth(100, 0.1, 20)?;
    let pop_analysis: HashMap<String, f64> = analyze_population_trend(&populations)?;
    println!(
        "{}",
        format!(
            "   Final population: {:?}",
            pop_analysis
                .get("final_population")
                .cloned()
                .unwrap_or_default()
        )
    );
    println!("{}", "\n=== All Simulations Complete ===");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_roll_dice_examples() {
        assert_eq!(roll_dice(0, 0), 0);
        assert_eq!(roll_dice(1, 2), 3);
        assert_eq!(roll_dice(-1, 1), 0);
    }
}
