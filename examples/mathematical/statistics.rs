use std::collections::HashMap;
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
#[doc = "Calculate arithmetic mean"]
#[doc = " Depyler: proven to terminate"]
pub fn mean(numbers: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    if numbers.is_empty() {
        return Ok(0.0);
    }
    Ok((numbers.iter().sum::<f64>() as f64) / (numbers.len() as i32 as f64))
}
#[doc = "Calculate median value"]
#[doc = " Depyler: proven to terminate"]
pub fn median(numbers: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    if numbers.is_empty() {
        return Ok(0.0);
    }
    let sorted_nums = {
        let mut sorted_vec = numbers.into_iter().collect::<Vec<_>>();
        sorted_vec.sort();
        sorted_vec
    };
    let _cse_temp_0 = sorted_nums.len() as i32;
    let n = _cse_temp_0;
    let _cse_temp_1 = n % 2;
    let _cse_temp_2 = _cse_temp_1 == 0;
    if _cse_temp_2 {
        Ok(({
            let base = &sorted_nums;
            let idx: i32 = {
                let a = n;
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
            } - 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        } + {
            let base = &sorted_nums;
            let idx: i32 = {
                let a = n;
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
            };
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        } as f64)
            / (2.0 as f64))
    } else {
        Ok({
            let base = &sorted_nums;
            let idx: i32 = {
                let a = n;
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
            };
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        })
    }
}
#[doc = "Find the most frequently occurring number"]
pub fn mode(numbers: &Vec<i32>) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    if numbers.is_empty() {
        return Ok(None);
    }
    let mut frequency: HashMap<i32, i32> = {
        let map = HashMap::new();
        map
    };
    for num in numbers.iter().cloned() {
        if frequency.get(&num).is_some() {
            {
                let _key = num;
                let _old_val = frequency.get(&_key).cloned().unwrap_or_default();
                frequency.insert(_key, _old_val + 1);
            }
        } else {
            frequency.insert(num, 1);
        }
    }
    let mut max_count = 0;
    let mut mode_value = numbers.get(0usize).cloned().unwrap_or_default();
    for (num, count) in frequency
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        if count > max_count {
            max_count = count;
            mode_value = num;
        }
    }
    Ok(Some(mode_value))
}
#[doc = "Calculate sample variance"]
pub fn variance(numbers: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = numbers.len() as i32;
    let _cse_temp_1 = _cse_temp_0 < 2;
    if _cse_temp_1 {
        return Ok(0.0);
    }
    let avg = mean(numbers)?;
    let mut sum_squared_diff = 0.0;
    for num in numbers.iter().cloned() {
        let diff = num - avg;
        sum_squared_diff = sum_squared_diff + diff * diff;
    }
    Ok((sum_squared_diff as f64) / ((numbers.len() as i32).saturating_sub(1) as f64))
}
#[doc = "Calculate sample standard deviation"]
#[doc = " Depyler: proven to terminate"]
pub fn standard_deviation(numbers: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    let var = variance(numbers)?;
    let _cse_temp_0 = var == 0.0;
    if _cse_temp_0 {
        return Ok(0.0);
    }
    let _cse_temp_1 = (var as f64) / (2.0 as f64);
    let mut x = _cse_temp_1.clone();
    for __sanitized in 0..10 {
        x = Vector::from_vec(
            x + (var as f64) / (x as f64).as_slice().iter().map(|&x| x / 2.0).collect(),
        );
    }
    Ok(x)
}
#[doc = "Calculate Pearson correlation coefficient"]
#[doc = " Depyler: proven to terminate"]
pub fn correlation(mut x: Vec<f64>, y: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = x.len() as i32;
    let _cse_temp_1 = y.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let _cse_temp_3 = _cse_temp_0 < 2;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    let n = _cse_temp_0;
    let mean_x = mean(x)?;
    let mean_y = mean(y)?;
    let mut numerator = 0.0;
    let mut sum_x_squared = 0.0;
    let mut sum_y_squared = 0.0;
    for i in 0..n {
        let dx = x.get(i as usize).cloned().unwrap_or_default() - mean_x;
        let dy = y.get(i as usize).cloned().unwrap_or_default() - mean_y;
        numerator = numerator + dx * dy;
        sum_x_squared = sum_x_squared + dx * dx;
        sum_y_squared = sum_y_squared + dy * dy;
    }
    let _cse_temp_5 = sum_x_squared * sum_y_squared;
    let denominator_squared = _cse_temp_5;
    let _cse_temp_6 = denominator_squared == 0.0;
    if _cse_temp_6 {
        return Ok(0.0);
    }
    let _cse_temp_7 = (denominator_squared as f64) / (2.0 as f64);
    let mut denominator = _cse_temp_7.clone();
    for __sanitized in 0..10 {
        denominator = (denominator + (denominator_squared as f64) / (denominator as f64) as f64)
            / (2.0 as f64);
    }
    Ok((numerator as f64) / (denominator as f64))
}
