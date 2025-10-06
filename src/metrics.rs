use crate::types::EquitySnapshot;
use return_quant;
use sharp_ratio;

pub struct Metrics {
    pub log_returns: Vec<f64>,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub total_pnl: f64,
}

pub fn compute_metrics(equity_curve: &[EquitySnapshot]) -> Metrics {
    let equity: Vec<f64> = equity_curve.iter().map(|snap| snap.equity).collect();
    let log_returns = return_quant::log_return(&equity.iter().map(|v| *v as u64).collect::<Vec<u64>>());
    let sharpe_ratio = sharp_ratio::sharpe_ratio(&log_returns, 0.0);
    let max_drawdown = compute_max_drawdown(&equity);
    let total_pnl = equity.last().unwrap_or(&0.0) - equity.first().unwrap_or(&0.0);
    Metrics {
        log_returns,
        sharpe_ratio,
        max_drawdown,
        total_pnl,
    }
}

fn compute_max_drawdown(equity: &[f64]) -> f64 {
    let mut max_drawdown = 0.0;
    let mut peak = std::f64::MIN;
    for &value in equity {
        if value > peak {
            peak = value;
        }
        let drawdown = (peak - value) / peak;
        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
    }
    max_drawdown
}