# Mini Backtester

## 📚 How to Add a New Strategy (For Contributors)

### 1. Create a New File for Your Strategy

Create a new file in `src/strategy/`, e.g. `my_strategy.rs`.

```rust
// src/strategy/my_strategy.rs
use crate::types::{Candle, Signal};
use super::Strategy;

pub struct MyStrategy {
    // Add your fields here
}

impl MyStrategy {
    pub fn new(/* params */) -> Self {
        Self {
            // Initialize fields
        }
    }
}

impl Strategy for MyStrategy {
    fn next(&mut self, candle: &Candle) -> Signal {
        // Your logic here
        Signal::Hold
    }
}
```

---

### 2. Register Your Strategy in `strategy.rs`

At the top of `src/strategy.rs`, add:

```rust
pub mod my_strategy;
```

At the bottom, re-export it:

```rust
pub use my_strategy::MyStrategy;
```

---

### 3. Use Your Strategy in `main.rs` or Anywhere

Now you can use your new strategy just like the built-ins:

```rust
use strategy::MyStrategy;

let mut strat = MyStrategy::new(/* params */);
let signal = strat.next(&candle);
```

Or add it to your strategies vector:

```rust
let mut strategies: Vec<Box<dyn Strategy>> = vec![
    Box::new(MyStrategy::new(/* params */)),
    // ...other strategies
];
```

---

### 4. Strategy Trait Reference

All strategies must implement:

```rust
pub trait Strategy {
    fn next(&mut self, candle: &Candle) -> Signal;
}
```

---

### 5. Strategy File Structure Example

```
src/
  strategy.rs
  strategy/
    ema_cross.rs
    sma_cross.rs
    mean_reversion.rs
    momentum.rs
    my_strategy.rs   <-- your new file
```

---

### 6. Tips for Advanced Strategies

- You can use any math crate or custom logic inside your strategy.
- If you need to share helpers, create a `strategy/utils.rs` and add `pub mod utils;` in `strategy.rs`.
- For parameterized strategies, expose a `new()` constructor with your desired arguments.

---

## 🏗️ Example: Adding a Bollinger Bands Strategy

1. **Create `src/strategy/bollinger.rs`:**

```rust
use crate::types::{Candle, Signal};
use super::Strategy;

pub struct Bollinger {
    // fields for SMA, stddev, etc.
}

impl Bollinger {
    pub fn new(/* params */) -> Self {
        // ...
    }
}

impl Strategy for Bollinger {
    fn next(&mut self, candle: &Candle) -> Signal {
        // ...
        Signal::Hold
    }
}
```

2. **Register in `strategy.rs`:**

```rust
pub mod bollinger;
pub use bollinger::Bollinger;
```

3. **Use in `main.rs`:**

```rust
use strategy::Bollinger;
let mut strat = Bollinger::new(/* params */);
```

---

## 🧑‍💻 Why This Is Modular

- Each strategy is self-contained and easy to maintain.
- No need to touch existing code to add a new strategy (just register it).
- The trait-based design means you can swap, combine, or test strategies with minimal boilerplate.

---

## 🚀 Next Steps

- Add more advanced strategies or a factory for dynamic creation.
- PRs welcome!