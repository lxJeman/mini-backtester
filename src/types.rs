use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub period: f64, // 1, 5, 15, 30, 60, 120, 240, 740, 1440
}

#[derive(Debug)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub struct Trade {
    pub timestamp: i64,
    pub action: Signal,
    pub price: f64,
    pub size: f64,
    pub fee: f64,
}

pub struct Position {
    pub entry_price: f64,
    pub size: f64,
    pub is_long: bool,
    /*
| `is_long` | Meaning                                                    |
| --------- | ---------------------------------------------------------- |
| `true`    | You **bought first**, expecting price to go up (**long**)  |
| `false`   | You **sold first**, expecting price to go down (**short**) |
   */
}

pub struct EquitySnapshot {
    pub timestamp: i64,
    pub cash: f64,
    pub equity: f64,
    pub position_value: f64,
    pub drawdown: f64,
}

/*
| Field            | Purpose                                      |
| ---------------- | -------------------------------------------- |
| `volume`         | How much was traded during the candle        |
| `is_long`        | Are we betting price goes **up** or **down** |
| `cash`           | Unused capital                               |
| `position_value` | Market value of open position                |
| `equity`         | Total account value (cash + position)        |
| `drawdown`       | How far equity has fallen from its peak      |

*/