Here’s your **updated and combined `todo.md` file** — now fully aware of your completed steps, and with improvements for integrating your existing `sma`, `returns`, and `sharpe` crates at the right moments.

---

## 🧭 Project: **Mini Backtester (single-threaded)**

You already have:

* ✅ Math libs (`sma`, `ema`, `returns`, `sharpe`) as separate crates
* ✅ A large OHLCV `.csv` dataset (multiple tokens, intervals)
* ✅ CSV-to-DB loader (optional for now)
* ✅ Strategy interface and working `EmaCross` strategy

Now we’ll build out the **core backtesting engine**, and integrate metrics.

---

# 📌 STEP-BY-STEP DEV GUIDE

---

### ✅ **Step 0: Project Setup**

#### 📋 TODO:

* [ ] Create a new binary project:

  ```bash
  cargo new backtester
  ```

* [ ] Add your math libs as local dependencies in `Cargo.toml`:

  ```toml
  [dependencies]
  sma = { path = "../sma" }
  ema = { path = "../ema" }
  returns = { path = "../returns" }
  sharpe = { path = "../sharpe" }
  ```

✅ Once this is set, `cargo build` should compile without errors.

---

### ✅ **Step 1: Define Core Types**

Create a file: `src/types.rs`

#### 📋 TODO:

* [ ] Define your core types:

  ```rust
  pub struct Candle {
      pub timestamp: i64,
      pub open: f64,
      pub high: f64,
      pub low: f64,
      pub close: f64,
      pub volume: f64,
      pub period: u32,
  }

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
  }

  pub struct EquitySnapshot {
      pub timestamp: i64,
      pub cash: f64,
      pub equity: f64,
      pub position_value: f64,
      pub drawdown: f64,
  }
  ```

✅ These types will be shared across modules.

---

### ✅ **Step 2: CSV Loader (token + interval selector)**

Create: `src/data.rs`

#### 📋 TODO:

* [ ] Load `.csv` using `csv` or `polars`

* [ ] Match files like `"ETCUSD_60.csv"` using `token_prefix` and `interval`:

  ```rust
  pub fn load_csv(path: &str, token_prefix: &str, interval: u32) -> Result<Vec<Candle>> { ... }
  ```

* [ ] Expect format:

  ```
  timestamp, open, high, low, close, volume, num_trades
  ```

✅ Test with:

```rust
let candles = load_csv("data/", "STORJEUR", 1440)?;
```

---

### ✅ **Step 3: Strategy Trait + First Strategy**

Create: `src/strategy.rs`

#### 📋 TODO:

* [ ] Define a trait:

  ```rust
  pub trait Strategy {
      fn next(&mut self, candle: &Candle) -> Signal;
  }
  ```

* [x] Implement `EmaCross` strategy using your `ema` crate ✅

✅ Already done.

#### 🔄 Future Integration Plan:

* [ ] SMA crossover strategy using your `sma` crate
* [ ] Mean reversion or momentum using `returns` crate
* [ ] Swap strategies easily thanks to trait-based design

---

### 🔄 **Step 4: Trade Model (slippage, fees, capital)**

Create: `src/trade_model.rs` + add balance starting at 1000$ (if balance hits 0 game over :()

#### 📋 TODO:

* [ ] Define the model:

  ```rust
  pub struct TradeModel {
      pub cash: f64,
      pub position: Option<Position>,
      pub slippage: f64,
      pub fee: f64,
      ...
  }
  ```

* [ ] Implement logic:

  ```rust
  impl TradeModel {
      pub fn apply(&mut self, signal: Signal, candle: &Candle) -> Option<Trade> { ... }
      pub fn equity(&self, price: f64) -> f64 { ... }
  }
  ```

✅ No need for math crate use here.

---

### 🔁 **Step 5: Main Backtest Loop**

Create: `src/backtest.rs`

#### 📋 TODO:

* [ ] Accept:

  * `Vec<Candle>`
  * a strategy
  * a trade model

* [ ] For each candle:

  ```rust
  for candle in candles {
      let signal = strategy.next(&candle);
      let trade = trade_model.apply(signal, &candle);
      // log trades, update equity
  }
  ```

* [ ] Collect:

  * [ ] All `Trade` events
  * [ ] `EquitySnapshot`s (useful for `returns` + `sharpe`)

✅ This structure lets you easily plug in new strategies and generate full backtest history.

---

### 📊 **Step 6: Metrics & Reporting**

Create: `src/metrics.rs`

#### 📋 TODO:

* [ ] Use your math crates:

  * `returns::log_returns(&Vec<f64>) -> Vec<f64>`
  * `sharpe::sharpe_ratio(&Vec<f64>) -> f64`

* [ ] Extract `Vec<f64>` from your equity snapshots

* [ ] Compute:

  * Sharpe ratio
  * Max drawdown (can write your own or use `returns`)
  * Total P&L

* [ ] Export to `.json` or print to console

✅ This is where `returns` and `sharpe` really shine.

---

### 🧪 **Step 7: Testing / Sanity Check**

#### 📋 TODO:

* [ ] Run on small data (e.g., `"1INCHEUR_60.csv"`)
* [ ] Use hardcoded params or simple CLI
* [ ] Check:

  * [ ] Are trades correctly placed?
  * [ ] Does the equity curve look smooth/reasonable?
  * [ ] Are any stats `NaN` or clearly broken?

✅ If this works, you're ready to scale to more intervals or tokens.

---

## ✅ Development Checklist

| Step | Task                                       | Status |
| ---- | ------------------------------------------ | ------ |
| 0    | Create binary project + add math libs      | ✅      |
| 1    | Define core types                          | ✅      |
| 2    | Implement CSV loader                       | ✅      |
| 3    | Implement strategy trait + EMA crossover   | ✅      |
| 4    | Implement trade model w/ slippage & fees   | 🔲     |
| 5    | Build main backtest loop                   | 🔲     |
| 6    | Compute metrics using `returns` + `sharpe` | 🔲     |
| 7    | Run test on small data, validate behavior  | 🔲     |

---

Would you like to:

1. Move on to **Step 4: TradeModel**, or
2. Start drafting **metrics.rs** early using sample `Vec<f64>`?

You're in a strong position — clean base, modular crates, ready to scale.
