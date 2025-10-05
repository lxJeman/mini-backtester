pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn moving_avg(data: &[u64], window: usize) -> Vec<f64> {
    if window == 0 || data.len() < window {
        return vec![];
    }

    let mut result = Vec::new();
    let mut sum: u64 = data[..window].iter().sum();

    result.push(sum as f64 / window as f64);

    for i in window..data.len() {
        sum = sum - data[i - window] + data[i];
        result.push(sum as f64 / window as f64);
    }

    result
}

pub struct Sma {
    pub window: usize,
    pub values: std::collections::VecDeque<f64>,
    pub sum: f64,
}

impl Sma {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            values: std::collections::VecDeque::with_capacity(window),
            sum: 0.0,
        }
    }

    pub fn next(&mut self, price: f64) -> f64 {
        self.values.push_back(price);
        self.sum += price;
        if self.values.len() > self.window {
            if let Some(removed) = self.values.pop_front() {
                self.sum -= removed;
            }
        }
        self.sum / self.values.len() as f64
    }

    pub fn get(&self) -> Option<f64> {
        if self.values.len() == self.window {
            Some(self.sum / self.window as f64)
        } else {
            None
        }
    }
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
    fn test_moving_avg_basic() {
        let data = vec![1, 2, 3, 4, 5];
        let result = moving_avg(&data, 3);
        let expected = vec![2.0, 3.0, 4.0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_moving_avg_window_size_one() {
        let data = vec![10, 20, 30];
        let result = moving_avg(&data, 1);
        let expected = vec![10.0, 20.0, 30.0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_moving_avg_empty_data() {
        let data = vec![];
        let result = moving_avg(&data, 3);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_moving_avg_window_too_large() {
        let data = vec![1, 2];
        let result = moving_avg(&data, 5);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_moving_avg_zero_window() {
        let data = vec![1, 2, 3];
        let result = moving_avg(&data, 0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_moving_avg_same_values() {
        let data = vec![5, 5, 5, 5];
        let result = moving_avg(&data, 2);
        let expected = vec![5.0, 5.0, 5.0];
        assert_eq!(result, expected);
    }
}
