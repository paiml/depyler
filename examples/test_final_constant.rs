#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_field_goal() -> i32 {
    const FIELD_GOAL: i32 = 600;
    FIELD_GOAL
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_get_field_goal_examples() {
        let _ = get_field_goal();
    }
}
