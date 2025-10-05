pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn simple_return(data: &[u64]) -> Vec<f64> {
    if data.len() < 2 {
        return vec![];
    }

    let mut returns_quant = Vec::new();

    for i in 1..data.len() {
        let c = data[i] as f64;
        let p = data[i-1] as f64;

        if p != 0.0 {
            let r = (c - p) / p;
            returns_quant.push(r);
        } else {
            returns_quant.push(0.0);
        }
    }    

    returns_quant
}

pub fn log_return(data: &[u64]) -> Vec<f64> {
    if data.len() < 2 {
        return vec![];
    }

    let mut returns_quant = Vec::new();

    for i in 1..data.len() {
        let c = data[i] as f64;
        let p = data[i - 1] as f64;

        if p > 0.0 && c > 0.0 {
            let r = (c.ln()) - (p.ln()); // r = ln(c/p)
            returns_quant.push(r);
        } else {
            // if price is zero or negative, log return undefined, push 0 or skip
            returns_quant.push(0.0);
        }
    }

    returns_quant
}

pub struct Returns {
    pub prev: Option<f64>,
    pub last_return: Option<f64>,
}

impl Returns {
    pub fn new() -> Self {
        Self { prev: None, last_return: None }
    }

    pub fn next(&mut self, price: f64) -> Option<f64> {
        let ret = match self.prev {
            Some(p) if p != 0.0 => Some((price - p) / p),
            Some(_) => Some(0.0),
            None => None,
        };
        self.prev = Some(price);
        self.last_return = ret;
        ret
    }

    pub fn get(&self) -> Option<f64> {
        self.last_return
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
    fn test_simple_return_basic() {
        let data = vec![100, 110, 99];
        let result = simple_return(&data);
        let expected = vec![0.1, -0.1];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_return_single_value() {
        let data = vec![100];
        let result = simple_return(&data);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_simple_return_empty_data() {
        let data = vec![];
        let result = simple_return(&data);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_simple_return_zero_price() {
        let data = vec![0, 100];
        let result = simple_return(&data);
        let expected = vec![0.0]; // Division by zero handled
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_return_doubling() {
        let data = vec![50, 100];
        let result = simple_return(&data);
        let expected = vec![1.0]; // 100% return
        assert_eq!(result, expected);
    }

    #[test]
    fn test_log_return_basic() {
        let data = vec![100, 110];
        let result = log_return(&data);
        // ln(110/100) = ln(1.1) ≈ 0.09531
        assert!((result[0] - 0.09531).abs() < 0.001);
    }

    #[test]
    fn test_log_return_single_value() {
        let data = vec![100];
        let result = log_return(&data);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_log_return_empty_data() {
        let data = vec![];
        let result = log_return(&data);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_log_return_zero_price() {
        let data = vec![0, 100];
        let result = log_return(&data);
        let expected = vec![0.0]; // Zero price handled
        assert_eq!(result, expected);
    }

    #[test]
    fn test_log_return_same_prices() {
        let data = vec![100, 100];
        let result = log_return(&data);
        let expected = vec![0.0]; // ln(100/100) = ln(1) = 0
        assert_eq!(result, expected);
    }
}
