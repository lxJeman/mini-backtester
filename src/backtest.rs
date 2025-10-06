use crate::types::{Candle, Trade, EquitySnapshot};
use crate::strategy::Strategy;
use crate::trade_model::TradeModel;

pub struct BacktestResult {
    pub trades: Vec<Trade>,
    pub equity_curve: Vec<EquitySnapshot>,
}

pub fn backtest(
    candles: &[Candle],
    strategy: &mut dyn Strategy,
    trade_model: &mut TradeModel,
) -> BacktestResult {
    let mut trades = Vec::new();
    let mut equity_curve = Vec::new();

    for candle in candles {
        let signal = strategy.next(candle);
        if let Some(trade) = trade_model.apply(signal, candle) {
            trades.push(trade);
        }
        let equity = trade_model.equity(candle.close);
        equity_curve.push(EquitySnapshot {
            timestamp: candle.timestamp,
            cash: trade_model.cash,
            equity,
            position_value: equity - trade_model.cash,
            drawdown: 0.0, // to be computed in metrics
        });
    }

    BacktestResult { trades, equity_curve }
}