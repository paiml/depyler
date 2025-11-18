#[doc = "// TODO: Map Python module 'statistics'"]
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
#[doc = "Test calculating arithmetic mean"]
pub fn test_mean() -> Result<f64, ZeroDivisionError> {
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut total: f64 = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = (_cse_temp_0) as f64;
    let _cse_temp_2 = (total as f64) / (_cse_temp_1 as f64);
    let mean: f64 = _cse_temp_2;
    Ok(mean)
}
#[doc = "Test median with odd number of elements"]
#[doc = " Depyler: proven to terminate"]
pub fn test_median_odd() -> Result<f64, Box<dyn std::error::Error>> {
    let data: Vec<f64> = vec![1.0, 3.0, 5.0, 7.0, 9.0];
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
    let _cse_temp_0 = sorted_data.len() as i32;
    let _cse_temp_1 = {
        let a = _cse_temp_0;
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
    let mid: i32 = _cse_temp_1;
    let median: f64 = sorted_data.get(mid as usize).cloned().unwrap_or_default();
    Ok(median)
}
#[doc = "Test median with even number of elements"]
#[doc = " Depyler: proven to terminate"]
pub fn test_median_even() -> Result<f64, Box<dyn std::error::Error>> {
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
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
    let _cse_temp_0 = sorted_data.len() as i32;
    let _cse_temp_1 = {
        let a = _cse_temp_0;
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
    let mid: i32 = _cse_temp_1;
    let _cse_temp_2 = {
        let base = &sorted_data;
        let idx: i32 = mid - 1;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx).cloned().unwrap_or_default()
    } + sorted_data.get(mid as usize).cloned().unwrap_or_default();
    let _cse_temp_3 = (_cse_temp_2 as f64) / (2.0 as f64);
    let median: f64 = _cse_temp_3;
    Ok(median)
}
#[doc = "Test finding mode(most common value)"]
#[doc = " Depyler: proven to terminate"]
pub fn test_mode() -> Result<i32, IndexError> {
    let data: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4];
    let mut max_count: i32 = 0;
    let mut mode_value: i32 = data.get(0usize).cloned().unwrap_or_default();
    for i in 0..data.len() as i32 {
        let mut count: i32 = 0;
        for j in 0..data.len() as i32 {
            if data.get(j as usize).cloned().unwrap_or_default()
                == data.get(i as usize).cloned().unwrap_or_default()
            {
                count = count + 1;
            }
        }
        if count > max_count {
            max_count = count;
            mode_value = data.get(i as usize).cloned().unwrap_or_default();
        }
    }
    Ok(mode_value)
}
#[doc = "Test calculating variance"]
pub fn test_variance() -> Result<f64, ZeroDivisionError> {
    let data: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let mut total: f64 = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = (_cse_temp_0) as f64;
    let _cse_temp_2 = (total as f64) / (_cse_temp_1 as f64);
    let mean: f64 = _cse_temp_2;
    let mut variance_sum: f64 = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_3 = (variance_sum as f64) / (_cse_temp_1 as f64);
    let variance: f64 = _cse_temp_3;
    Ok(variance)
}
#[doc = "Test calculating standard deviation"]
pub fn test_stdev() -> Result<f64, ZeroDivisionError> {
    let data: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let mut total: f64 = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = (_cse_temp_0) as f64;
    let _cse_temp_2 = (total as f64) / (_cse_temp_1 as f64);
    let mean: f64 = _cse_temp_2;
    let mut variance_sum: f64 = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_3 = (variance_sum as f64) / (_cse_temp_1 as f64);
    let variance: f64 = _cse_temp_3;
    let stdev: f64 = (variance as f64).sqrt();
    Ok(stdev)
}
#[doc = "Test finding min and max"]
pub fn test_min_max() -> Result<(), IndexError> {
    let data: Vec<f64> = vec![3.5, 1.2, 7.8, 2.4, 9.1];
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok((0.0, 0.0));
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
    Ok((min_val, max_val))
}
#[doc = "Test calculating range(max - min)"]
pub fn test_range() -> Result<f64, IndexError> {
    let data: Vec<f64> = vec![1.0, 5.0, 3.0, 9.0, 2.0];
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0.0);
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
    let data_range: f64 = max_val - min_val;
    Ok(data_range)
}
#[doc = "Test sum calculation"]
#[doc = " Depyler: verified panic-free"]
pub fn test_sum() -> f64 {
    let data: Vec<f64> = vec![1.5, 2.5, 3.5, 4.5];
    let mut total: f64 = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    total
}
#[doc = "Calculate percentile(simplified)"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_percentile(
    data: &Vec<f64>,
    percentile: i32,
) -> Result<f64, Box<dyn std::error::Error>> {
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
    let _cse_temp_0 = sorted_data.len() as i32;
    let _cse_temp_1 = percentile * _cse_temp_0;
    let _cse_temp_2 = {
        let a = _cse_temp_1;
        let b = 100;
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
    let mut index: i32 = _cse_temp_2;
    let _cse_temp_3 = index >= _cse_temp_0;
    if _cse_temp_3 {
        index = _cse_temp_0 - 1;
    }
    Ok(sorted_data.get(index as usize).cloned().unwrap_or_default())
}
#[doc = "Calculate Q1, Q2(median), Q3"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_quartiles(data: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let q1: f64 = calculate_percentile(&data, 25)?;
    let q2: f64 = calculate_percentile(&data, 50)?;
    let q3: f64 = calculate_percentile(&data, 75)?;
    Ok((q1, q2, q3))
}
#[doc = "Calculate interquartile range(IQR)"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_iqr(data: Vec<f64>) -> Result<f64, IndexError> {
    let quartiles: () = calculate_quartiles(data)?;
    let q1: f64 = quartiles.get(0usize).cloned().unwrap_or_default();
    let q3: f64 = quartiles.get(2usize).cloned().unwrap_or_default();
    let iqr: f64 = q3 - q1;
    Ok(iqr)
}
#[doc = "Detect outliers using IQR method"]
pub fn detect_outliers(data: Vec<f64>) -> Result<Vec<f64>, IndexError> {
    let quartiles: () = calculate_quartiles(data)?;
    let q1: f64 = quartiles.get(0usize).cloned().unwrap_or_default();
    let q3: f64 = quartiles.get(2usize).cloned().unwrap_or_default();
    let iqr: f64 = q3 - q1;
    let _cse_temp_0 = 1.5 * iqr;
    let lower_bound: f64 = q1 - _cse_temp_0;
    let upper_bound: f64 = q3 + _cse_temp_0;
    let mut outliers: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        if (value < lower_bound) || (value > upper_bound) {
            outliers.push(value);
        }
    }
    Ok(outliers)
}
#[doc = "Normalize data to 0-1 range"]
pub fn normalize_data(data: Vec<f64>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(vec![]);
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
    let data_range: f64 = max_val - min_val;
    let _cse_temp_2 = data_range == 0.0;
    if _cse_temp_2 {
        return Ok(data);
    }
    let mut normalized: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        let norm_value: f64 = (value - min_val as f64) / (data_range as f64);
        normalized.push(norm_value);
    }
    Ok(normalized)
}
#[doc = "Standardize data(z-score)"]
pub fn standardize_data(data: Vec<f64>) -> Result<Vec<f64>, ZeroDivisionError> {
    let mut total: f64 = 0.0;
    for value in data.iter().cloned() {
        total = total + value;
    }
    let _cse_temp_0 = data.len() as i32;
    let _cse_temp_1 = (_cse_temp_0) as f64;
    let _cse_temp_2 = (total as f64) / (_cse_temp_1 as f64);
    let mean: f64 = _cse_temp_2;
    let mut variance_sum: f64 = 0.0;
    for value in data.iter().cloned() {
        let diff: f64 = value - mean;
        variance_sum = variance_sum + diff * diff;
    }
    let _cse_temp_3 = (variance_sum as f64) / (_cse_temp_1 as f64);
    let variance: f64 = _cse_temp_3;
    let stdev: f64 = (variance as f64).sqrt();
    let _cse_temp_4 = stdev == 0.0;
    if _cse_temp_4 {
        return Ok(data);
    }
    let mut standardized: Vec<f64> = vec![];
    for value in data.iter().cloned() {
        let z_score: f64 = (value - mean as f64) / (stdev as f64);
        standardized.push(z_score);
    }
    Ok(standardized)
}
#[doc = "Calculate covariance between two datasets"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_covariance<'a, 'b>(
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
    let mut x_total: f64 = 0.0;
    let mut y_total: f64 = 0.0;
    for i in 0..x.len() as i32 {
        x_total = x_total + x.get(i as usize).cloned().unwrap_or_default();
        y_total = y_total + y.get(i as usize).cloned().unwrap_or_default();
    }
    let _cse_temp_5 = (_cse_temp_0) as f64;
    let _cse_temp_6 = (x_total as f64) / (_cse_temp_5 as f64);
    let x_mean: f64 = _cse_temp_6;
    let _cse_temp_7 = (_cse_temp_1) as f64;
    let _cse_temp_8 = (y_total as f64) / (_cse_temp_7 as f64);
    let y_mean: f64 = _cse_temp_8;
    let mut cov_sum: f64 = 0.0;
    for i in 0..x.len() as i32 {
        let x_diff: f64 = x.get(i as usize).cloned().unwrap_or_default() - x_mean;
        let y_diff: f64 = y.get(i as usize).cloned().unwrap_or_default() - y_mean;
        cov_sum = cov_sum + x_diff * y_diff;
    }
    let _cse_temp_9 = (cov_sum as f64) / (_cse_temp_5 as f64);
    let covariance: f64 = _cse_temp_9;
    Ok(covariance)
}
#[doc = "Calculate Pearson correlation coefficient"]
pub fn calculate_correlation(x: Vec<f64>, y: Vec<f64>) -> Result<f64, ZeroDivisionError> {
    let _cse_temp_0 = x.len() as i32;
    let _cse_temp_1 = y.len() as i32;
    let _cse_temp_2 = _cse_temp_0 != _cse_temp_1;
    let _cse_temp_3 = _cse_temp_0 == 0;
    let _cse_temp_4 = (_cse_temp_2) || (_cse_temp_3);
    if _cse_temp_4 {
        return Ok(0.0);
    }
    let cov: f64 = calculate_covariance(&x, &y)?;
    let mut x_total: f64 = 0.0;
    for val in x.iter().cloned() {
        x_total = x_total + val;
    }
    let _cse_temp_5 = (_cse_temp_0) as f64;
    let _cse_temp_6 = (x_total as f64) / (_cse_temp_5 as f64);
    let x_mean: f64 = _cse_temp_6;
    let mut x_var_sum: f64 = 0.0;
    for val in x.iter().cloned() {
        let mut diff: f64 = val - x_mean;
        x_var_sum = x_var_sum + diff * diff;
    }
    let x_stdev: f64 = ((x_var_sum as f64) / ((x.len() as i32) as f64 as f64) as f64).sqrt();
    let mut y_total: f64 = 0.0;
    for val in y.iter().cloned() {
        y_total = y_total + val;
    }
    let _cse_temp_7 = (_cse_temp_1) as f64;
    let _cse_temp_8 = (y_total as f64) / (_cse_temp_7 as f64);
    let y_mean: f64 = _cse_temp_8;
    let mut y_var_sum: f64 = 0.0;
    for val in y.iter().cloned() {
        let mut diff: f64 = val - y_mean;
        y_var_sum = y_var_sum + diff * diff;
    }
    let y_stdev: f64 = ((y_var_sum as f64) / ((y.len() as i32) as f64 as f64) as f64).sqrt();
    let _cse_temp_9 = x_stdev == 0.0;
    let _cse_temp_10 = y_stdev == 0.0;
    let _cse_temp_11 = (_cse_temp_9) || (_cse_temp_10);
    if _cse_temp_11 {
        return Ok(0.0);
    }
    let _cse_temp_12 = x_stdev * y_stdev;
    let _cse_temp_13 = (cov as f64) / (_cse_temp_12 as f64);
    let correlation: f64 = _cse_temp_13;
    Ok(correlation)
}
#[doc = "Run all statistics module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_statistics_features() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mean: f64 = test_mean()?;
    let median_odd: f64 = test_median_odd()?;
    let median_even: f64 = test_median_even()?;
    let mode_data: Vec<i32> = vec![1, 2, 2, 3, 3, 3];
    let mode: i32 = test_mode()?;
    let variance: f64 = test_variance()?;
    let stdev: f64 = test_stdev()?;
    let minmax: () = test_min_max()?;
    let data_range: f64 = test_range()?;
    let mut total: f64 = test_sum()?;
    let sample: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let p50: f64 = calculate_percentile(&sample, 50)?;
    let quartiles: () = calculate_quartiles(sample)?;
    let iqr: f64 = calculate_iqr(sample)?;
    let outlier_data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0];
    let mut outliers: Vec<f64> = detect_outliers(outlier_data)?;
    let mut normalized: Vec<f64> = normalize_data(sample)?;
    let mut standardized: Vec<f64> = standardize_data(sample)?;
    let x_data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_data: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let cov: f64 = calculate_covariance(&x_data, &y_data)?;
    let corr: f64 = calculate_correlation(x_data, y_data)?;
    println!("{}", "All statistics module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_mode_examples() {
        let _ = test_mode();
    }
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
    #[test]
    fn test_standardize_data_examples() {
        assert_eq!(standardize_data(vec![]), vec![]);
        assert_eq!(standardize_data(vec![1]), vec![1]);
    }
}
