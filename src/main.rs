use std::io;

use crate::data::load_token_csvs;

mod types;
mod data;
mod strategy;

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

        // 🧠 Here you'll call: backtest(&candles, ...)
    }

    Ok(())
}
