use std::io;

use crate::data::load_token_csvs;
use strategy::{EmaCross, SmaCross, MeanReversion, Momentum, ATRBreakout, VolatilityTargeting, Strategy};
use backtest::{backtest, BacktestResult};
use trade_model::TradeModel;
use metrics::compute_metrics;

mod types;
mod data;
mod strategy;
mod trade_model;
mod backtest;
mod metrics;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // EXAMPLE CODE FOR MATH LIBS

    // let data = ["23", "12", "87", "39", "95", "121", "16"];
    // let data_int: Vec<u64> = data.iter()
    //     .map(|s| s.parse::<u64>().expect("Invalid number"))
    //     .collect();

    // println!("---------------- SMA ----------------");
    // let result = moving_avg::moving_avg(&data_int, 3);
    // println!("Moving Avg of data is: {:?}", result);


    // println!("---------------- EMA ----------------");
    // let result2 = ema::ema(&data_int, 0.3); // You can change alpha to test other values
    // println!("EMA of data is: {:?}", result2);


    // println!("---------------- RETURNS -------------");
    // println!("----------- SIMPLE RETURNS -----------");

    // let result3 = return_quant::simple_return(&data_int);
    // println!("SIMPLE RETURNS of data is: {:?}", result3);

    // println!("------------- LOG RETURNS ------------");
    // let reslut4 = return_quant::log_return(&data_int);
    // println!("LOG RETURN of data is {:?}", reslut4);


    // println!("---------- COMPOUND GROWTH ----------");
    // let mut start = 1500.0; // 1.5k EURO
    // let daily_percent = 1.0; // 1% Profit Return Daily
    // let days = 365; // One year
    
    // let result5 = compound_growth::compund_growth(start, daily_percent, days);
    // println!("COMPUND GROWTH of starting capital of {}, and Daily % of {}, and a period of time of: {} equals to: {}", start, daily_percent, days, result5);


    // println!("-------- COMPOUND GROWTH LOOP --------");
    // let times = 5; // 5 years starting with original capital

    // for i in 1..=times {
    //     start = compound_growth::compund_growth(start, daily_percent, days);
    //     println!("After iteration {}: {:.2}", i, start);
    // }


    // println!("------------ SHARPE RATIO ------------");
    // let simple_returns = return_quant::simple_return(&data_int);
    // let risk_free_rate = 0.05; // Example: 5% annual risk-free rate as decimal
    
    // let sharpe = sharp_ratio::sharpe_ratio(&simple_returns, risk_free_rate);
    // println!("Simple returns: {:?}", simple_returns);
    // println!("Risk-free rate: {:.2}%", risk_free_rate * 100.0);
    // println!("Sharpe Ratio: {:.4}", sharpe);

    println!("------------ MINI BACKTESTER ------------");

    let mut input = String::new();
    println!("Enter token name (e.g. ETCUSD): ");
    io::stdin().read_line(&mut input)?;
    let token = input.trim();

    let files = load_token_csvs("Kraken_OHLCVT_Q4_2024/", token)?;

    println!("Found files: ");
    for (filename, candles) in &files {
        println!("- {} ({} candles)", filename, candles.len());
    }

    let mut strategies: Vec<Box<dyn Strategy>> = vec![
        Box::new(EmaCross::new(9, 21)),
        Box::new(SmaCross::new(10, 30)),
        Box::new(MeanReversion::new(0.01)),
        Box::new(Momentum::new(0.01)),
        Box::new(ATRBreakout::new(10, 14, 0.01)),
        // Volatility Targeting with EMA Cross as base strategy
        Box::new(VolatilityTargeting::new(
            0.01,  // 1% target daily volatility
            20,    // 20-day lookback for volatility calculation
            Box::new(EmaCross::new(9, 21)), // Base strategy for signals
            0.1,   // Minimum position multiplier (10% of normal size)
            3.0,   // Maximum position multiplier (300% of normal size)
        )),
    ];

    println!("Available strategies: {}", strategies.len());

    // 🔁 NEW: loop through each file
    for (filename, candles) in &files {
        println!("\n--- Running strategies on file: {} ---", filename);

        // Reset each strategy to a new instance for this file
        let mut fresh_strategies: Vec<Box<dyn Strategy>> = vec![
            Box::new(EmaCross::new(9, 21)),
            Box::new(SmaCross::new(10, 30)),
            Box::new(MeanReversion::new(0.01)),
            Box::new(Momentum::new(0.01)),
            Box::new(ATRBreakout::new(10, 14, 0.01)),
            // Volatility Targeting with EMA Cross as base strategy
            Box::new(VolatilityTargeting::new(
                0.01,  // 1% target daily volatility
                20,    // 20-day lookback for volatility calculation
                Box::new(EmaCross::new(9, 21)), // Base strategy for signals
                0.1,   // Minimum position multiplier (10% of normal size)
                3.0,   // Maximum position multiplier (300% of normal size)
            )),
        ];

        for (i, strat) in fresh_strategies.iter_mut().enumerate() {
            let strategy_name = match i {
                0 => "EMA Cross (9,21)",
                1 => "SMA Cross (10,30)", 
                2 => "Mean Reversion",
                3 => "Momentum",
                4 => "ATR Breakout",
                5 => "Volatility Targeting + EMA Cross",
                _ => "Unknown Strategy",
            };
            
            println!("\nStrategy {}: {}", i + 1, strategy_name);
            
            // Step 7: Run a full backtest on a small sample
            let mut trade_model = TradeModel::new(1000.0, 0.001, 0.001, 1.0);
            let result = backtest(&candles[..candles.len().min(100)], strat.as_mut(), &mut trade_model);
            let metrics = compute_metrics(&result.equity_curve);
            
            println!("Total trades: {}", result.trades.len());
            println!("Final equity: {:.2}", result.equity_curve.last().map(|e| e.equity).unwrap_or(0.0));
            println!("Sharpe ratio: {:.4}", metrics.sharpe_ratio);
            println!("Max drawdown: {:.2}%", metrics.max_drawdown * 100.0);
            println!("Total P&L: {:.2}", metrics.total_pnl);
            
            // Special output for volatility targeting strategy
            if i == 5 {
                if let Some(vol_strat) = strat.as_any().downcast_ref::<VolatilityTargeting>() {
                    println!("Current position multiplier: {:.2}x", vol_strat.get_position_multiplier());
                    println!("Current realized volatility: {:.4} ({:.2}%)", 
                        vol_strat.get_realized_volatility(), 
                        vol_strat.get_realized_volatility() * 100.0);
                }
            }
        }
    }


    Ok(())
}
