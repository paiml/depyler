#[doc = "Calculate sum using nested helper."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate(values: &Vec<i32>) -> i32 {
    let mut sum_values;
    sum_values = |lst: &Vec<i32>| -> i32 {
        let mut total = 0;
        for x in lst.iter().cloned() {
            total = total + x;
        }
        return total;
    };
    sum_values(&values)
}
#[doc = "Main entry point."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let result = calculate(&vec![1, 2, 3, 4, 5]);
    println!("{}", format!("Sum: {:?}", result));
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_calculate_examples() {
        assert_eq!(calculate(&vec![]), 0);
        assert_eq!(calculate(&vec![1]), 1);
        assert_eq!(calculate(&vec![1, 2, 3]), 3);
    }
}
