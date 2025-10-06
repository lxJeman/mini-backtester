pub mod ema_cross;
pub mod sma_cross;
pub mod mean_reversion;
pub mod momentum;

use crate::types::{Candle, Signal};

pub trait Strategy {
    fn next(&mut self, candle: &Candle) -> Signal;
}

pub use ema_cross::EmaCross;
pub use sma_cross::SmaCross;
pub use mean_reversion::MeanReversion;
pub use momentum::Momentum;