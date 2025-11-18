#[doc = "Calculate the sum of a list of integers."]
#[doc = " Depyler: verified panic-free"]
pub fn calculate_sum(numbers: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for n in numbers.iter().cloned() {
        total = total + n;
    }
    total
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_calculate_sum_examples() {
        assert_eq!(calculate_sum(&vec![]), 0);
        assert_eq!(calculate_sum(&vec![1]), 1);
        assert_eq!(calculate_sum(&vec![1, 2, 3]), 6);
    }
}
