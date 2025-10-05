pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn average(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let sum: f64 = data.iter().sum();
    sum / (data.len() as f64)
}

pub fn std_deviation(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    
    let avg = average(data);
    let variance = data.iter()
        .map(|value| {
            let diff = *value - avg;
            diff * diff
        })
        .sum::<f64>() / ((data.len() - 1) as f64);  // sample std dev uses n-1
    
    let std_dev = variance.sqrt();
    
    // Handle floating point precision issues - if very close to zero, return 0
    if std_dev < 1e-10 {
        0.0
    } else {
        std_dev
    }
}

pub fn sharpe_ratio(returns: &[f64], risk_free_rate: f64) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }
    
    let avg_return = average(returns);
    let std_dev = std_deviation(returns);
    
    if std_dev == 0.0 {
        return 0.0;  // Avoid division by zero
    }
    
    (avg_return - risk_free_rate) / std_dev
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
    fn test_average() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = average(&data);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_average_empty() {
        let data = vec![];
        let result = average(&data);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_average_single_value() {
        let data = vec![42.0];
        let result = average(&data);
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_average_negative_values() {
        let data = vec![-1.0, -2.0, -3.0];
        let result = average(&data);
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_std_deviation_basic() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let result = std_deviation(&data);
        // Sample std dev should be around 2.138
        assert!((result - 2.138).abs() < 0.01);
    }

    #[test]
    fn test_std_deviation_empty() {
        let data = vec![];
        let result = std_deviation(&data);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_std_deviation_single_value() {
        let data = vec![5.0];
        let result = std_deviation(&data);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_std_deviation_same_values() {
        let data = vec![3.0, 3.0, 3.0, 3.0];
        let result = std_deviation(&data);
        assert_eq!(result, 0.0);
    }
    
    #[test]
    fn test_sharpe_ratio() {
        let returns = vec![0.1, 0.05, 0.15, -0.02, 0.08];
        let risk_free_rate = 0.03;
        let result = sharpe_ratio(&returns, risk_free_rate);
        assert!(result > 0.0); // Should be positive for this example
    }

    #[test]
    fn test_sharpe_ratio_empty_returns() {
        let returns = vec![];
        let risk_free_rate = 0.03;
        let result = sharpe_ratio(&returns, risk_free_rate);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sharpe_ratio_zero_std_dev() {
        let returns = vec![0.05, 0.05, 0.05]; // Same returns = zero std dev
        let risk_free_rate = 0.03;
        let result = sharpe_ratio(&returns, risk_free_rate);
        // When std dev is very close to zero, we should get a very large number
        // Let's check that std_deviation returns 0 for same values
        let std_dev = std_deviation(&returns);
        assert_eq!(std_dev, 0.0);
        assert_eq!(result, 0.0); // Should handle division by zero
    }

    #[test]
    fn test_sharpe_ratio_negative() {
        let returns = vec![-0.1, -0.05, -0.15]; // All negative returns
        let risk_free_rate = 0.03;
        let result = sharpe_ratio(&returns, risk_free_rate);
        assert!(result < 0.0); // Should be negative
    }

    #[test]
    fn test_sharpe_ratio_high_risk_free_rate() {
        let returns = vec![0.02, 0.03, 0.01]; // Low returns
        let risk_free_rate = 0.10; // High risk-free rate
        let result = sharpe_ratio(&returns, risk_free_rate);
        assert!(result < 0.0); // Should be negative when returns < risk-free rate
    }
}
