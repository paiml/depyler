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
#[doc = "Generate a number in range(simplified without random)."]
#[doc = " Depyler: proven to terminate"]
pub fn generate_number(min_val: i32, max_val: i32) -> Result<i32, ZeroDivisionError> {
    Ok({
        let a = min_val + max_val;
        let b = 2;
        let q = a / b;
        let r = a % b;
        let r_negative = r < 0;
        let b_negative = b < 0;
        let r_nonzero = r != 0;
        let signs_differ = r_negative != b_negative;
        let needs_adjustment = r_nonzero && signs_differ;
        if needs_adjustment {
            q - 1
        } else {
            q
        }
    })
}
#[doc = "Provide a hint based on the guess."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_hint(guess: i32, target: i32) -> String {
    let _cse_temp_0 = guess < target;
    if _cse_temp_0 {
        "Marco!(Too low)".to_string()
    } else {
        let _cse_temp_1 = guess > target;
        if _cse_temp_1 {
            "Marco!(Too high)".to_string()
        } else {
            "Polo!".to_string()
        }
    }
}
#[doc = "Calculate final score."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_score(attempts: i32, rounds: i32) -> i32 {
    let _cse_temp_0 = rounds == 0;
    if _cse_temp_0 {
        return 0;
    }
    let _cse_temp_1 = 100 * rounds;
    let base_score = _cse_temp_1;
    let _cse_temp_2 = attempts * 5;
    let penalty = _cse_temp_2;
    let score = base_score - penalty;
    let _cse_temp_3 = score < 0;
    if _cse_temp_3 {
        return 0;
    }
    score
}
#[doc = "Get difficulty name from level."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_difficulty_name(level: i32) -> &'a str {
    let _cse_temp_0 = level == 1;
    if _cse_temp_0 {
        "Easy".to_string()
    } else {
        let _cse_temp_1 = level == 2;
        if _cse_temp_1 {
            "Medium".to_string()
        } else {
            let _cse_temp_2 = level == 3;
            if _cse_temp_2 {
                "Hard".to_string()
            } else {
                "Unknown".to_string()
            }
        }
    }
}
#[doc = "Calculate average with safety check."]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_average(total: i32, count: i32) -> Result<f64, ZeroDivisionError> {
    let _cse_temp_0 = count == 0;
    if _cse_temp_0 {
        return Ok(0.0);
    }
    Ok((total as f64) / (count as f64))
}
#[doc = "Format game statistics as string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn format_statistics(score: i32, attempts: i32, rounds: i32) -> String {
    let avg = calculate_average(attempts, rounds);
    let mut result = "Game Statistics:\n";
    let _cse_temp_0 = score.to_string();
    let _cse_temp_1 = format!("{}{}", format!("{}{}", result, "Score: "), _cse_temp_0);
    result = format!("{}{}", _cse_temp_1, "\n");
    let _cse_temp_2 = attempts.to_string();
    let _cse_temp_3 = format!("{}{}", format!("{}{}", result, "Attempts: "), _cse_temp_2);
    result = format!("{}{}", _cse_temp_3, "\n");
    let _cse_temp_4 = avg.to_string();
    let _cse_temp_5 = format!("{}{}", format!("{}{}", result, "Average: "), _cse_temp_4);
    result = format!("{}{}", _cse_temp_5, "\n");
    result.unwrap()
}
#[doc = "Check if guess is in valid range."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_guess(guess: i32, min_val: i32, max_val: i32) -> bool {
    let _cse_temp_0 = guess < min_val;
    if _cse_temp_0 {
        return false;
    }
    let _cse_temp_1 = guess > max_val;
    if _cse_temp_1 {
        return false;
    }
    true
}
#[doc = "Simulate a round with fixed guesses."]
#[doc = " Depyler: verified panic-free"]
pub fn play_simple_round(target: i32, max_attempts: i32) -> i32 {
    let mut attempts = 0;
    let mut guess = 50;
    while attempts < max_attempts {
        attempts = attempts + 1;
        if guess == target {
            return attempts;
        } else {
            if guess < target {
                guess = guess + 10;
            } else {
                guess = guess - 5;
            }
        }
    }
    attempts
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_generate_number_examples() {
        assert_eq!(generate_number(0, 0), 0);
        assert_eq!(generate_number(1, 2), 3);
        assert_eq!(generate_number(-1, 1), 0);
    }
    #[test]
    fn test_calculate_score_examples() {
        assert_eq!(calculate_score(0, 0), 0);
        assert_eq!(calculate_score(1, 2), 3);
        assert_eq!(calculate_score(-1, 1), 0);
    }
    #[test]
    fn test_play_simple_round_examples() {
        assert_eq!(play_simple_round(0, 0), 0);
        assert_eq!(play_simple_round(1, 2), 3);
        assert_eq!(play_simple_round(-1, 1), 0);
    }
}
