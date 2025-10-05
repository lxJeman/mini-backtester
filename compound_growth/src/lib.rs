pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn compund_growth(start: f64, daily_percent: f64, days: u32) -> f64 {
    let daily_rate = daily_percent / 100.0;
    let r = start * (1.0 + daily_rate).powi(days as i32);
    let rounded = (r * 100.0).round() / 100.0;

    rounded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_compound_growth_basic() {
        let result = compund_growth(1000.0, 10.0, 1);
        let expected = 1100.0; // 1000 * (1 + 0.1)^1
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compound_growth_zero_percent() {
        let result = compund_growth(1000.0, 0.0, 365);
        let expected = 1000.0; // No growth
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compound_growth_zero_days() {
        let result = compund_growth(1000.0, 5.0, 0);
        let expected = 1000.0; // No time passed
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compound_growth_multiple_days() {
        let result = compund_growth(100.0, 1.0, 2);
        let expected = 102.01; // 100 * (1.01)^2 = 102.01
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compound_growth_negative_percent() {
        let result = compund_growth(1000.0, -10.0, 1);
        let expected = 900.0; // 1000 * (1 - 0.1)^1
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compound_growth_rounding() {
        let result = compund_growth(100.0, 1.0, 3);
        // 100 * (1.01)^3 = 103.0301, rounded to 103.03
        let expected = 103.03;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compound_growth_large_numbers() {
        let result = compund_growth(10000.0, 5.0, 10);
        // Should handle larger calculations properly
        assert!(result > 10000.0);
        assert!(result < 20000.0); // Reasonable bounds check
    }
}
