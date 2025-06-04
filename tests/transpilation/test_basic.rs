#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, 5), 2);
        assert_eq!(binary_search(&arr, 1), 0);
        assert_eq!(binary_search(&arr, 9), 4);
        assert_eq!(binary_search(&arr, 2), -1);
        assert_eq!(binary_search(&arr, 10), -1);
        
        let empty: Vec<i32> = vec![];
        assert_eq!(binary_search(&empty, 5), -1);
    }

    #[test]
    fn test_calculate_sum() {
        assert_eq!(calculate_sum(&vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(calculate_sum(&vec![10, -5, 3]), 8);
        assert_eq!(calculate_sum(&vec![]), 0);
        assert_eq!(calculate_sum(&vec![42]), 42);
        assert_eq!(calculate_sum(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn test_process_config() {
        let mut config1 = HashMap::new();
        config1.insert("debug".to_string(), "true".to_string());
        assert_eq!(process_config(&config1), Some("true".to_string()));
        
        let mut config2 = HashMap::new();
        config2.insert("verbose".to_string(), "yes".to_string());
        assert_eq!(process_config(&config2), None);
        
        let config3 = HashMap::new();
        assert_eq!(process_config(&config3), None);
    }

    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(42), "positive");
        assert_eq!(classify_number(-42), "negative");
        assert_eq!(classify_number(1), "positive");
        assert_eq!(classify_number(-1), "negative");
    }
}