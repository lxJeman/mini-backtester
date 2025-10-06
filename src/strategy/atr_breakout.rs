use crate::types::{Candle, Signal};
use super::{Strategy, ATR};

pub struct ATRBreakout {
    atr: ATR,
    lookback: usize,
    closes: Vec<f64>,
    atr_multiplier: f64,
}

impl ATRBreakout {
    pub fn new(lookback: usize, atr_period: usize, atr_multiplier: f64) -> Self {
        Self {
            atr: ATR::new(atr_period),
            lookback,
            closes: Vec::with_capacity(lookback),
            atr_multiplier,
        }
    }
}

impl Strategy for ATRBreakout {
    fn next(&mut self, candle: &Candle) -> Signal {
        // Update ATR
        let atr_value = self.atr.update(candle);

        // Only act when we have enough data
        if atr_value.is_none() || self.closes.len() < self.lookback {
            self.closes.push(candle.close);
            return Signal::Hold;
        }

        let atr = atr_value.unwrap();
        let highest = self.closes.iter().cloned().fold(f64::MIN, f64::max);
        let lowest = self.closes.iter().cloned().fold(f64::MAX, f64::min);

        let volatility_ok = atr > self.atr_multiplier * candle.close;

        let signal = if candle.close > highest && volatility_ok {
            Signal::Buy
        } else if candle.close < lowest && volatility_ok {
            Signal::Sell
        } else {
            Signal::Hold
        };

        // Now update close history
        self.closes.push(candle.close);
        if self.closes.len() > self.lookback {
            self.closes.remove(0);
        }

        signal
    }
}