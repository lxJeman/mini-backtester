Perfect — let’s create a **step-by-step dev guide** with detailed todos. This will serve as your **backtesting project roadmap**, fully aware of your existing work, and built to help you move one clear step at a time.

---

## 🧭 Project: **Mini Backtester (single-threaded)**

You already have:

* ✅ Math libs (`sma`, `ema`, `returns`, `sharpe`) as separate crates
* ✅ A large OHLCV `.csv` dataset (multiple tokens, intervals)
* ✅ CSV-to-DB loader (optional for now)

We will now build the **core backtester crate** to connect all this.

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

### 🧱 **Step 1: Define Core Types**

Create a file `src/types.rs`

#### 📋 TODO:

* [ ] Define basic structs:

```rust
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub period: u32, // like 1, 5, 60, etc
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

* [ ] Import in `main.rs`:

```rust
mod types;
use types::*;
```

✅ Once you define these, we’ll use them everywhere.

---

### 📦 **Step 2: CSV Loader (token + interval selector)**

Create a file: `src/data.rs`

#### 📋 TODO:

* [ ] Load a `.csv` file using `polars` or `csv` crate
* [ ] Match filename using `token_prefix` and `interval`:

```rust
// e.g. load_csv("data/", "1INCHEUR", 60) => loads 1INCHEUR_60.csv
pub fn load_csv(path: &str, token_prefix: &str, interval: u32) -> Result<Vec<Candle>> {
    ...
}
```

* [ ] Use the format of your CSV:
  `timestamp, open, high, low, close, volume, num_trades`

✅ Once working, you can load any token + interval with:

```rust
let candles = load_csv("data/", "STORJEUR", 1440)?;
```

---

### ⚙️ **Step 3: Simple Strategy**

Create `src/strategy.rs`

#### 📋 TODO:

* [ ] Define a trait:

```rust
pub trait Strategy {
    fn next(&mut self, candle: &Candle) -> Signal;
}
```

* [ ] Implement a basic EMA crossover strategy using your `ema` crate:

```rust
pub struct EmaCross {
    short_period: usize,
    long_period: usize,
    short_values: Vec<f64>,
    long_values: Vec<f64>,
}

impl Strategy for EmaCross {
    fn next(&mut self, candle: &Candle) -> Signal {
        // update ema values
        // compare them
        // return Buy / Sell / Hold
    }
}
```

✅ Once implemented, your strategy is plug-and-play.

---

### 💰 **Step 4: Trade Model (fees, slippage)**

Create `src/trade_model.rs`

#### 📋 TODO:

* [ ] Track:

  * Cash
  * Position
  * Equity
* [ ] Add config: slippage, fee, starting balance
* [ ] Apply trading logic:

```rust
pub struct TradeModel {
    pub cash: f64,
    pub position: Option<Position>,
    pub slippage: f64,
    pub fee: f64,
    ...
}

impl TradeModel {
    pub fn apply(&mut self, signal: Signal, candle: &Candle) -> Option<Trade> {
        ...
    }

    pub fn equity(&self, price: f64) -> f64 {
        ...
    }
}
```

✅ This runs inside the main loop and returns `Trade` objects when applicable.

---

### 🔁 **Step 5: Main Event Loop**

In `src/backtest.rs`

#### 📋 TODO:

* [ ] Accept candles, a strategy, and a trade model
* [ ] Loop through all candles:

```rust
for candle in candles {
    let signal = strategy.next(&candle);
    let trade = trade_model.apply(signal, &candle);
    // log trade
    // track equity
}
```

* [ ] Collect:

  * Trades
  * EquitySnapshots

✅ At the end, export them to `.csv`

---

### 📊 **Step 6: Metrics & Reporting**

In `src/metrics.rs`

#### 📋 TODO:

* [ ] Use your `returns` and `sharpe` crate
* [ ] Compute:

  * Sharpe ratio
  * Max drawdown
  * Total return

Export as `.json` or print to console.

---

### 🧪 **Step 7: Testing / Sanity Check**

#### 📋 TODO:

* [ ] Run on 1 small token+interval (e.g., 1INCHEUR_60)
* [ ] Use CLI or hardcode params for now
* [ ] Check:

  * Trades make sense
  * Equity curve behaves
  * Metrics aren’t zero/NaN

---

## 🧾 Summary: Your Development Checklist

| Step | Task                                      | Status |
| ---- | ----------------------------------------- | ------ |
| 0    | Create binary project + add math libs     | 🔲     |
| 1    | Define core types                         | 🔲     |
| 2    | Implement CSV loader (token + interval)   | 🔲     |
| 3    | Implement Strategy trait + basic strategy | 🔲     |
| 4    | Implement TradeModel with slippage/fees   | 🔲     |
| 5    | Build main backtest loop                  | 🔲     |
| 6    | Compute metrics + export results          | 🔲     |
| 7    | Test run on real data                     | 🔲     |

---

Would you like to start with **Step 1 (types.rs)**, or is your project already partially scaffolded? Let’s move step-by-step from where you’re at.
