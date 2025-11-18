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
#[doc = "Generate sample data using normal distribution(simplified)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn generate_sample_data(size: i32, mean: f64, stddev: f64) -> Vec<f64> {
    let mut data: Vec<f64> = vec![];
    for _i in 0..size {
        let value: f64 = rand::random::<f64>() * stddev + mean;
        data.push(value);
    }
    data
}
#[doc = "Calculate comprehensive statistics on dataset"]
pub fn calculate_statistics(
    data: &mut Vec<f64>,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok({
            let map = HashMap::new();
            map
        });
    }
    let mut stats: HashMap<String, f64> = {
        let map = HashMap::new();
        map
    };
    let mut total: f64 = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_2 = (_cse_temp_0) as f64;
    let _cse_temp_3 = total / _cse_temp_2;
    let mean: f64 = _cse_temp_3;
    stats.insert("mean".to_string(), mean);
    let mut variance_sum: f64 = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_4 = variance_sum / _cse_temp_2;
    let variance: f64 = _cse_temp_4;
    stats.insert("variance".to_string(), variance);
    stats.insert("std_dev".to_string(), (variance as f64).sqrt());
    let mut min_val: f64 = data.get(0usize).cloned().unwrap_or_default();
    let mut max_val: f64 = data.get(0usize).cloned().unwrap_or_default();
    for value in data.iter().cloned() {
        if value < min_val {
            min_val = value;
        }
        if value > max_val {
            max_val = value;
        }
    }
    stats.insert("min".to_string(), min_val);
    stats.insert("max".to_string(), max_val);
    stats.insert("range".to_string(), max_val - min_val);
    let mut sorted_data: Vec<f64> = data.clone();
    for i in 0..sorted_data.len() as i32 {
        for j in i + 1..sorted_data.len() as i32 {
            if sorted_data.get(j as usize).cloned().unwrap_or_default()
                < sorted_data.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: f64 = sorted_data.get(i as usize).cloned().unwrap_or_default();
                sorted_data.insert(
                    (i) as usize,
                    sorted_data.get(j as usize).cloned().unwrap_or_default(),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    let _cse_temp_5 = sorted_data.len() as i32;
    let _cse_temp_6 = {
        let a = _cse_temp_5;
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
    let mid: i32 = _cse_temp_6;
    let _cse_temp_7 = _cse_temp_5 % 2;
    let _cse_temp_8 = _cse_temp_7 == 1;
    if _cse_temp_8 {
        stats.insert(
            "median".to_string(),
            sorted_data.get(mid as usize).cloned().unwrap_or_default(),
        );
    } else {
        let _cse_temp_9 = {
            let base = &sorted_data;
            let idx: i32 = mid - 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        } + sorted_data.get(mid as usize).cloned().unwrap_or_default();
        let _cse_temp_10 = _cse_temp_9 / 2.0;
        stats.insert("median".to_string(), _cse_temp_10);
    }
    Ok(stats)
}
#[doc = "Calculate quartiles using math and sorting"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_percentiles(
    data: &mut Vec<f64>,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok({
            let map = HashMap::new();
            map
        });
    }
    let mut sorted_data: Vec<f64> = data.clone();
    for i in 0..sorted_data.len() as i32 {
        for j in i + 1..sorted_data.len() as i32 {
            if sorted_data.get(j as usize).cloned().unwrap_or_default()
                < sorted_data.get(i as usize).cloned().unwrap_or_default()
            {
                let temp: f64 = sorted_data.get(i as usize).cloned().unwrap_or_default();
                sorted_data.insert(
                    (i) as usize,
                    sorted_data.get(j as usize).cloned().unwrap_or_default(),
                );
                sorted_data.insert((j) as usize, temp);
            }
        }
    }
    let mut percentiles: HashMap<String, f64> = {
        let map = HashMap::new();
        map
    };
    let _cse_temp_2 = sorted_data.len() as i32;
    let _cse_temp_3 = {
        let a = _cse_temp_2;
        let b = 4;
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
    let q1_idx: i32 = _cse_temp_3;
    percentiles.insert(
        "q1".to_string(),
        sorted_data
            .get(q1_idx as usize)
            .cloned()
            .unwrap_or_default(),
    );
    let _cse_temp_4 = {
        let a = _cse_temp_2;
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
    let q2_idx: i32 = _cse_temp_4;
    percentiles.insert(
        "q2".to_string(),
        sorted_data
            .get(q2_idx as usize)
            .cloned()
            .unwrap_or_default(),
    );
    let _cse_temp_5 = 3 * _cse_temp_2;
    let _cse_temp_6 = {
        let a = _cse_temp_5;
        let b = 4;
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
    let q3_idx: i32 = _cse_temp_6;
    percentiles.insert(
        "q3".to_string(),
        sorted_data
            .get(q3_idx as usize)
            .cloned()
            .unwrap_or_default(),
    );
    let _cse_temp_7 = percentiles.get("q3").cloned().unwrap_or_default()
        - percentiles.get("q1").cloned().unwrap_or_default();
    percentiles.insert("iqr".to_string(), _cse_temp_7);
    Ok(percentiles)
}
#[doc = "Detect outliers using IQR method(combines statistics + collections)"]
pub fn detect_outliers(mut data: Vec<f64>) -> Result<Vec<f64>, IndexError> {
    let mut percentiles: HashMap<String, f64> = calculate_percentiles(&data)?;
    let _cse_temp_0 = percentiles.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    let q1: f64 = percentiles.get("q1").cloned().unwrap_or_default();
    let q3: f64 = percentiles.get("q3").cloned().unwrap_or_default();
    let iqr: f64 = percentiles.get("iqr").cloned().unwrap_or_default();
    let _cse_temp_2 = 1.5 * iqr;
    let lower_bound: f64 = q1 - _cse_temp_2;
    let upper_bound: f64 = q3 + _cse_temp_2;
    let mut outliers: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        if (value < lower_bound) || (value > upper_bound) {
            outliers.push(value);
        }
    }
    Ok(outliers)
}
#[doc = "Create histogram bins(uses collections + math)"]
pub fn bin_data(
    data: &mut Vec<f64>,
    num_bins: i32,
) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    let _cse_temp_2 = num_bins <= 0;
    let _cse_temp_3 = (_cse_temp_1) || (_cse_temp_2);
    if _cse_temp_3 {
        return Ok({
            let map = HashMap::new();
            map
        });
    }
    let mut min_val: f64 = data.get(0usize).cloned().unwrap_or_default();
    let mut max_val: f64 = data.get(0usize).cloned().unwrap_or_default();
    for value in data.iter().cloned() {
        if value < min_val {
            min_val = value;
        }
        if value > max_val {
            max_val = value;
        }
    }
    let _cse_temp_4 = (num_bins) as f64;
    let _cse_temp_5 = (max_val - min_val) / _cse_temp_4;
    let bin_width: f64 = _cse_temp_5;
    let mut bins: HashMap<i32, i32> = {
        let map = HashMap::new();
        map
    };
    for i in 0..num_bins {
        bins.insert(i, 0);
    }
    for value in data.iter().cloned() {
        let mut bin_index: i32 = ((value - min_val) / bin_width) as i32;
        if bin_index >= num_bins {
            bin_index = num_bins - 1;
        }
        {
            let _key = bin_index;
            let _old_val = bins.get(&_key).cloned().unwrap_or_default();
            bins.insert(_key, _old_val + 1);
        }
    }
    Ok(bins)
}
#[doc = "Calculate Pearson correlation coefficient"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_correlation<'a, 'b>(
    x: &'a Vec<f64>,
    y: &'b Vec<f64>,
) -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = x.len() as i32;
    let _cse_temp_1 = y.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let _cse_temp_3 = _cse_temp_0 == 0;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    let mut x_sum: f64 = 0.0;
    let mut y_sum: f64 = 0.0;
    for i in 0..x.len() as i32 {
        x_sum = x_sum + x.get(i as usize).cloned().unwrap_or_default();
        y_sum = y_sum + y.get(i as usize).cloned().unwrap_or_default();
    }
    let _cse_temp_5 = (_cse_temp_0) as f64;
    let _cse_temp_6 = (x_sum as f64) / (_cse_temp_5 as f64);
    let x_mean: f64 = _cse_temp_6;
    let _cse_temp_7 = (_cse_temp_1) as f64;
    let _cse_temp_8 = (y_sum as f64) / (_cse_temp_7 as f64);
    let y_mean: f64 = _cse_temp_8;
    let mut numerator: f64 = 0.0;
    let mut x_variance_sum: f64 = 0.0;
    let mut y_variance_sum: f64 = 0.0;
    for i in 0..x.len() as i32 {
        let x_diff: f64 = x.get(i as usize).cloned().unwrap_or_default() - x_mean;
        let y_diff: f64 = y.get(i as usize).cloned().unwrap_or_default() - y_mean;
        numerator = numerator + x_diff * y_diff;
        x_variance_sum = x_variance_sum + x_diff * x_diff;
        y_variance_sum = y_variance_sum + y_diff * y_diff;
    }
    let denominator: f64 = (x_variance_sum * y_variance_sum as f64).sqrt();
    let _cse_temp_9 = denominator == 0.0;
    if _cse_temp_9 {
        return Ok(0.0);
    }
    let _cse_temp_10 = (numerator as f64) / (denominator as f64);
    let correlation: f64 = _cse_temp_10;
    Ok(correlation)
}
#[doc = "Z-score normalization using statistics"]
pub fn normalize_data(mut data: Vec<f64>) -> Result<Vec<f64>, ZeroDivisionError> {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
    }
    let mut total: f64 = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_2 = (_cse_temp_0) as f64;
    let _cse_temp_3 = (total as f64) / (_cse_temp_2 as f64);
    let mean: f64 = _cse_temp_3;
    let mut variance_sum: f64 = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let stddev: f64 = ((variance_sum as f64) / ((data.len() as i32) as f64 as f64) as f64).sqrt();
    let _cse_temp_4 = stddev == 0.0;
    if _cse_temp_4 {
        return Ok(data);
    }
    let mut normalized: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        let z_score: f64 = (value - mean as f64) / (stddev as f64);
        normalized.push(z_score);
    }
    Ok(normalized)
}
#[doc = "Group data by ranges using collections"]
pub fn group_by_range<'a, 'b>(
    data: &'a mut Vec<f64>,
    ranges: &'b Vec<(f64, f64)>,
) -> Result<HashMap<String, Vec<f64>>, IndexError> {
    let mut groups: HashMap<String, Vec<f64>> = {
        let map = HashMap::new();
        map
    };
    for i in 0..ranges.len() as i32 {
        let mut range_tuple: (f64, f64) = ranges.get(i as usize).cloned().unwrap_or_default();
        let mut range_key: String = format!(
            "{:?}-{:?}",
            range_tuple.get(0usize).cloned().unwrap_or_default(),
            range_tuple.get(1usize).cloned().unwrap_or_default()
        );
        groups.insert(range_key, vec![]);
    }
    for value in data.iter().cloned() {
        for i in 0..ranges.len() as i32 {
            let mut range_tuple: (f64, f64) = ranges.get(i as usize).cloned().unwrap_or_default();
            if (value >= range_tuple.get(0usize).cloned().unwrap_or_default())
                && (value < range_tuple.get(1usize).cloned().unwrap_or_default())
            {
                let mut range_key: String = format!(
                    "{:?}-{:?}",
                    range_tuple.get(0usize).cloned().unwrap_or_default(),
                    range_tuple.get(1usize).cloned().unwrap_or_default()
                );
                groups
                    .get(&range_key)
                    .cloned()
                    .unwrap_or_default()
                    .push(value);
                break;
            }
        }
    }
    Ok(groups)
}
#[doc = "Monte Carlo simulation combining random + math + statistics"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn monte_carlo_simulation(
    num_trials: i32,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut results: Vec<f64> = vec![];
    for _trial in 0..num_trials {
        let x: f64 = rand::random::<f64>() * 10.0;
        let y: f64 = rand::random::<f64>() * 10.0;
        let distance: f64 = (x * x + y * y as f64).sqrt();
        results.push(distance);
    }
    let mut stats: HashMap<String, f64> = calculate_statistics(&results)?;
    Ok(stats)
}
#[doc = "Main analysis pipeline combining all modules"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn analyze_dataset() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=== Comprehensive Data Analysis Demo ===");
    {
        let _seed = 42;
        ()
    };
    let sample_size: i32 = 100;
    let dataset: Vec<f64> = generate_sample_data(sample_size, 50.0, 10.0);
    let mut stats: HashMap<String, f64> = calculate_statistics(&dataset)?;
    println!(
        "{}",
        format!(
            "Mean: {:?}, StdDev: {:?}",
            stats.get("mean").cloned().unwrap_or_default(),
            stats.get("std_dev").cloned().unwrap_or_default()
        )
    );
    let mut percentiles: HashMap<String, f64> = calculate_percentiles(&dataset)?;
    println!(
        "{}",
        format!(
            "Q1: {:?}, Median: {:?}, Q3: {:?}",
            percentiles.get("q1").cloned().unwrap_or_default(),
            percentiles.get("q2").cloned().unwrap_or_default(),
            percentiles.get("q3").cloned().unwrap_or_default()
        )
    );
    let mut outliers: Vec<f64> = detect_outliers(dataset)?;
    println!("{}", format!("Outliers found: {:?}", outliers.len() as i32));
    let histogram: HashMap<i32, i32> = bin_data(&dataset, 10)?;
    println!(
        "{}",
        format!("Histogram bins created: {:?}", histogram.len() as i32)
    );
    let mut normalized: Vec<f64> = normalize_data(dataset)?;
    let normalized_stats: HashMap<String, f64> = calculate_statistics(&normalized)?;
    println!(
        "{}",
        format!(
            "Normalized mean: {:?}",
            normalized_stats.get("mean").cloned().unwrap_or_default()
        )
    );
    let dataset2: Vec<f64> = generate_sample_data(sample_size, 60.0, 12.0);
    let corr: f64 = calculate_correlation(&dataset, &dataset2)?;
    println!("{}", format!("Correlation: {:?}", corr));
    let ranges: Vec<(f64, f64)> = vec![(0.0, 25.0), (25.0, 50.0), (50.0, 75.0), (75.0, 100.0)];
    let mut groups: HashMap<String, Vec<f64>> = group_by_range(&dataset, &ranges)?;
    println!(
        "{}",
        format!("Range groups created: {:?}", groups.len() as i32)
    );
    let mc_stats: HashMap<String, f64> = monte_carlo_simulation(1000)?;
    println!(
        "{}",
        format!(
            "Monte Carlo mean: {:?}",
            mc_stats.get("mean").cloned().unwrap_or_default()
        )
    );
    println!("{}", "=== Analysis Complete ===");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_detect_outliers_examples() {
        assert_eq!(detect_outliers(vec![]), vec![]);
        assert_eq!(detect_outliers(vec![1]), vec![1]);
    }
    #[test]
    fn quickcheck_normalize_data() {
        fn prop(data: Vec<f64>) -> TestResult {
            let once = normalize_data(&data);
            let twice = normalize_data(once.clone());
            if once != twice {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(Vec<f64>) -> TestResult);
    }
    #[test]
    fn test_normalize_data_examples() {
        assert_eq!(normalize_data(vec![]), vec![]);
        assert_eq!(normalize_data(vec![1]), vec![1]);
    }
}
