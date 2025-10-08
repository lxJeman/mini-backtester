use crate::types::{Candle, Signal};
use moving_avg::Sma;
use super::Strategy;

pub struct SmaCross {
    pub short_sma: Sma,
    pub long_sma: Sma,
}

impl SmaCross {
    pub fn new(short_period: usize, long_period: usize) -> Self {
        Self {
            short_sma: Sma::new(short_period),
            long_sma: Sma::new(long_period),
        }
    }
}

impl Strategy for SmaCross {
    fn next(&mut self, candle: &Candle) -> Signal {
        let _short = self.short_sma.next(candle.close);
        let _long = self.long_sma.next(candle.close);
        match (self.short_sma.get(), self.long_sma.get()) {
            (Some(short), Some(long)) => {
                if short > long {
                    Signal::Buy
                } else if short < long {
                    Signal::Sell
                } else {
                    Signal::Hold
                }
            }
            _ => Signal::Hold,
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}