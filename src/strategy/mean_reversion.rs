use crate::types::{Candle, Signal};
use return_quant::Returns;
use super::Strategy;

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
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}