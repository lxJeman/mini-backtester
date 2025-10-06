use crate::types::{Candle, Signal, Trade, Position};

pub struct TradeModel {
    pub cash: f64,
    pub position: Option<Position>,
    pub slippage: f64,
    pub fee: f64,
    pub min_trade_size: f64,
}

impl TradeModel {
    pub fn new(starting_cash: f64, slippage: f64, fee: f64, min_trade_size: f64) -> Self {
        Self {
            cash: starting_cash,
            position: None,
            slippage,
            fee,
            min_trade_size,
        }
    }

    pub fn apply(&mut self, signal: Signal, candle: &Candle) -> Option<Trade> {
        match signal {
            Signal::Buy => {
                if self.position.is_none() && self.cash > self.min_trade_size {
                    // Open long position
                    let price = candle.close * (1.0 + self.slippage);
                    let size = self.cash / price;
                    let fee = price * size * self.fee;
                    self.cash = 0.0;
                    self.position = Some(Position {
                        entry_price: price,
                        size,
                        is_long: true,
                    });
                    Some(Trade {
                        timestamp: candle.timestamp,
                        action: Signal::Buy,
                        price,
                        size,
                        fee,
                    })
                } else {
                    None
                }
            }
            Signal::Sell => {
                if let Some(pos) = &self.position {
                    if pos.is_long {
                        // Close long position
                        let price = candle.close * (1.0 - self.slippage);
                        let size = pos.size;
                        let fee = price * size * self.fee;
                        let proceeds = price * size - fee;
                        self.cash = proceeds;
                        self.position = None;
                        Some(Trade {
                            timestamp: candle.timestamp,
                            action: Signal::Sell,
                            price,
                            size,
                            fee,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Signal::Hold => None,
        }
    }

    pub fn equity(&self, price: f64) -> f64 {
        match &self.position {
            Some(pos) if pos.is_long => self.cash + pos.size * price,
            _ => self.cash,
        }
    }
}