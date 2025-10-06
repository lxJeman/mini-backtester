use crate::types::Candle;

pub struct ATR {
    period: usize,
    tr_values: Vec<f64>,
    prev_close: Option<f64>,
}

impl ATR {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            tr_values: Vec::with_capacity(period),
            prev_close: None,
        }
    }

    pub fn update(&mut self, candle: &Candle) -> Option<f64> {
        let tr = self.true_range(candle);
        self.tr_values.push(tr);

        if self.tr_values.len() > self.period {
            self.tr_values.remove(0);
        }

        self.prev_close = Some(candle.close);

        if self.tr_values.len() < self.period {
            None // not enough data yet
        } else {
            Some(self.tr_values.iter().sum::<f64>() / self.period as f64)
        }
    }

    fn true_range(&self, candle: &Candle) -> f64 {
        let prev_close = self.prev_close.unwrap_or(candle.close);
        let high_low = candle.high - candle.low;
        let high_close = (candle.high - prev_close).abs();
        let low_close = (candle.low - prev_close).abs();
        high_low.max(high_close).max(low_close)
    }
}
