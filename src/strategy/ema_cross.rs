use crate::types::{Candle, Signal};
use ema::Ema;
use super::Strategy;

pub struct EmaCross {
    pub short_ema: Ema,
    pub long_ema: Ema,
}

impl EmaCross {
    pub fn new(short_period: usize, long_period: usize) -> Self {
        Self {
            short_ema: Ema::new(short_period),
            long_ema: Ema::new(long_period),
        }
    }
}

impl Strategy for EmaCross {
    fn next(&mut self, candle: &Candle) -> Signal {
        let _short = self.short_ema.next(candle.close);
        let _long = self.long_ema.next(candle.close);
        match (self.short_ema.get(), self.long_ema.get()) {
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
}