pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn ema(data: &[u64], a: f64) -> Vec<f64> {
    if data.is_empty() || a <= 0.0 || a > 1.0 {
        return vec![];
    }

    let mut r = Vec::with_capacity(data.len());
    let mut prev_ema = data[0] as f64;
    r.push(prev_ema);

    for &value in &data[1..] {
        let ema = a * (value as f64) + (1.0 - a) * prev_ema;
        r.push(ema);
        prev_ema = ema;
    }

    r
}

pub struct Ema {
    pub alpha: f64,
    pub value: Option<f64>,
}

impl Ema {
    pub fn new(period: usize) -> Self {
        let alpha = 2.0 / (period as f64 + 1.0);
        Self { alpha, value: None }
    }

    pub fn next(&mut self, price: f64) -> f64 {
        let new_ema = match self.value {
            Some(prev) => self.alpha * price + (1.0 - self.alpha) * prev,
            None => price,
        };
        self.value = Some(new_ema);
        new_ema
    }

    pub fn get(&self) -> Option<f64> {
        self.value
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
    fn test_ema_basic() {
        let data = vec![10, 20, 30];
        let result = ema(&data, 0.5);
        let expected = vec![10.0, 15.0, 22.5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ema_single_value() {
        let data = vec![100];
        let result = ema(&data, 0.3);
        let expected = vec![100.0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ema_empty_data() {
        let data = vec![];
        let result = ema(&data, 0.5);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_ema_invalid_alpha_zero() {
        let data = vec![1, 2, 3];
        let result = ema(&data, 0.0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_ema_invalid_alpha_greater_than_one() {
        let data = vec![1, 2, 3];
        let result = ema(&data, 1.5);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_ema_alpha_one() {
        let data = vec![10, 20, 30];
        let result = ema(&data, 1.0);
        let expected = vec![10.0, 20.0, 30.0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ema_small_alpha() {
        let data = vec![100, 200];
        let result = ema(&data, 0.1);
        // EMA = 0.1 * 200 + 0.9 * 100 = 20 + 90 = 110
        let expected = vec![100.0, 110.0];
        assert_eq!(result, expected);
    }
}
