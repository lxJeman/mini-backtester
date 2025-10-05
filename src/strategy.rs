use ema; // Import the ema crate
use crate::types::{Candle, Signal};


pub trait Strategy {
    fn next(&mut self, candle: &Candle) -> Signal;
}

pub struct EmaCross {
    pub short_period: usize,
    pub long_period: usize,
    pub short_values: Vec<f64>,
    pub long_values: Vec<f64>,
    pub short_ema: Option<f64>,
    pub long_ema: Option<f64>,
}

impl EmaCross {
    pub fn new(short_period: usize, long_period: usize) -> Self {
        Self {
            short_period,
            long_period,
            short_values: Vec::with_capacity(short_period),
            long_values: Vec::with_capacity(long_period),
            short_ema: None,
            long_ema: None,
        }
    }
}

impl Strategy for EmaCross {
    fn next(&mut self, candle: &Candle) -> Signal {
        // Update rolling close prices
        self.short_values.push(candle.close);
        if self.short_values.len() > self.short_period {
            self.short_values.remove(0);
        }
        self.long_values.push(candle.close);
        if self.long_values.len() > self.long_period {
            self.long_values.remove(0);
        }

        // Calculate smoothing factor alpha
        let short_alpha = 2.0 / (self.short_period as f64 + 1.0);
        let long_alpha = 2.0 / (self.long_period as f64 + 1.0);

        // Update EMAs
        if self.short_values.len() == self.short_period {
            let prev_short_ema = self.short_ema.unwrap_or(self.short_values[0]);
            let new_short_ema = short_alpha * candle.close + (1.0 - short_alpha) * prev_short_ema;
            self.short_ema = Some(new_short_ema);
        }
        if self.long_values.len() == self.long_period {
            let prev_long_ema = self.long_ema.unwrap_or(self.long_values[0]);
            let new_long_ema = long_alpha * candle.close + (1.0 - long_alpha) * prev_long_ema;
            self.long_ema = Some(new_long_ema);
        }

        // Only generate signals if both EMAs are available
        match (self.short_ema, self.long_ema) {
            (Some(short), Some(long)) => {
                if short > long {
                    Signal::Buy
                } else if short < long {
                    Signal::Sell
                } else {
                    Signal::Hold
                }
            }
            _ => Signal::Hold, // Not enough data yet
        }
    }
}