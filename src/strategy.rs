use ema; // Import the ema crate
use ema::Ema;
use moving_avg::Sma;
use crate::types::{Candle, Signal};
use return_quant::Returns;


pub trait Strategy {
    fn next(&mut self, candle: &Candle) -> Signal;
}

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
        let short = self.short_ema.next(candle.close);
        let long = self.long_ema.next(candle.close);

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
            _ => Signal::Hold, // Not enough data yet
        }
    }
}

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
        let short = self.short_sma.next(candle.close);
        let long = self.long_sma.next(candle.close);

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
            _ => Signal::Hold, // Not enough data yet
        }
    }
}

pub struct MeanReversion {
    pub returns: Returns,
    pub threshold: f64,
}

impl MeanReversion {
    pub fn new(threshold: f64) -> Self {
        Self {
            returns: Returns::new(),
            threshold,
        }
    }
}

impl Strategy for MeanReversion {
    fn next(&mut self, candle: &Candle) -> Signal {
        let ret = self.returns.next(candle.close);
        match ret {
            Some(r) if r < -self.threshold => Signal::Buy,
            Some(r) if r > self.threshold => Signal::Sell,
            Some(_) => Signal::Hold,
            None => Signal::Hold,
        }
    }
}

pub struct Momentum {
    pub returns: Returns,
    pub threshold: f64,
}

impl Momentum {
    pub fn new(threshold: f64) -> Self {
        Self {
            returns: Returns::new(),
            threshold,
        }
    }
}

impl Strategy for Momentum {
    fn next(&mut self, candle: &Candle) -> Signal {
        let ret = self.returns.next(candle.close);
        match ret {
            Some(r) if r > self.threshold => Signal::Buy,
            Some(r) if r < -self.threshold => Signal::Sell,
            Some(_) => Signal::Hold,
            None => Signal::Hold,
        }
    }
}