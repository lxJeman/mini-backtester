pub mod ema_cross;
pub mod sma_cross;
pub mod mean_reversion;
pub mod momentum;
pub mod atr;
pub mod atr_breakout;

use crate::types::{Candle, Signal};

pub trait Strategy {
    fn next(&mut self, candle: &Candle) -> Signal;
}

pub use ema_cross::EmaCross;
pub use sma_cross::SmaCross;
pub use mean_reversion::MeanReversion;
pub use momentum::Momentum;
pub use atr::ATR;
pub use atr_breakout::ATRBreakout;