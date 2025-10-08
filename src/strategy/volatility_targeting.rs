use crate::types::{Candle, Signal};
use return_quant::Returns;
use sharp_ratio::std_deviation;
use super::Strategy;
use std::any::Any;

pub struct VolatilityTargeting {
    pub target_volatility: f64,      // Target daily volatility (e.g., 0.01 for 1%)
    pub lookback_period: usize,      // Period for calculating realized volatility (e.g., 20 days)
    pub base_signal_strategy: Box<dyn Strategy>, // Underlying strategy for signals
    pub returns_calculator: Returns,
    pub returns_history: Vec<f64>,
    pub position_multiplier: f64,    // Current position size multiplier
    pub min_multiplier: f64,         // Minimum position multiplier (e.g., 0.1)
    pub max_multiplier: f64,         // Maximum position multiplier (e.g., 5.0)
}

impl VolatilityTargeting {
    pub fn new(
        target_volatility: f64,
        lookback_period: usize,
        base_signal_strategy: Box<dyn Strategy>,
        min_multiplier: f64,
        max_multiplier: f64,
    ) -> Self {
        Self {
            target_volatility,
            lookback_period,
            base_signal_strategy,
            returns_calculator: Returns::new(),
            returns_history: Vec::new(),
            position_multiplier: 1.0,
            min_multiplier,
            max_multiplier,
        }
    }

    /// Calculate the realized volatility from recent returns
    fn calculate_realized_volatility(&self) -> f64 {
        if self.returns_history.len() < 2 {
            return self.target_volatility; // Default to target if not enough data
        }
        
        // Use the most recent returns up to lookback_period
        let start_idx = if self.returns_history.len() > self.lookback_period {
            self.returns_history.len() - self.lookback_period
        } else {
            0
        };
        
        let recent_returns = &self.returns_history[start_idx..];
        std_deviation(recent_returns)
    }

    /// Update position multiplier based on volatility targeting formula
    fn update_position_multiplier(&mut self) {
        let realized_vol = self.calculate_realized_volatility();
        
        if realized_vol > 0.0 {
            // Position Size Multiplier = σ_target / σ_realized
            let raw_multiplier = self.target_volatility / realized_vol;
            
            // Clamp the multiplier to reasonable bounds
            self.position_multiplier = raw_multiplier.max(self.min_multiplier).min(self.max_multiplier);
        } else {
            // If no volatility, use neutral multiplier
            self.position_multiplier = 1.0;
        }
    }

    /// Get the current position size multiplier
    pub fn get_position_multiplier(&self) -> f64 {
        self.position_multiplier
    }

    /// Get the current realized volatility
    pub fn get_realized_volatility(&self) -> f64 {
        self.calculate_realized_volatility()
    }
}

impl Strategy for VolatilityTargeting {
    fn next(&mut self, candle: &Candle) -> Signal {
        // Calculate return for this candle
        if let Some(return_val) = self.returns_calculator.next(candle.close) {
            self.returns_history.push(return_val);
            
            // Keep only the data we need (a bit more than lookback for efficiency)
            if self.returns_history.len() > self.lookback_period * 2 {
                self.returns_history.drain(0..self.lookback_period);
            }
        }
        
        // Update position multiplier based on current volatility
        self.update_position_multiplier();
        
        // Get signal from underlying strategy
        // The position sizing will be applied at the portfolio/execution level
        self.base_signal_strategy.next(candle)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::ema_cross::EmaCross;

    #[test]
    fn test_volatility_targeting_initialization() {
        let base_strategy = Box::new(EmaCross::new(5, 20));
        let vol_targeting = VolatilityTargeting::new(
            0.01,  // 1% target volatility
            20,    // 20-day lookback
            base_strategy,
            0.1,   // min multiplier
            5.0,   // max multiplier
        );
        
        assert_eq!(vol_targeting.target_volatility, 0.01);
        assert_eq!(vol_targeting.lookback_period, 20);
        assert_eq!(vol_targeting.position_multiplier, 1.0);
    }

    #[test]
    fn test_position_multiplier_calculation() {
        let base_strategy = Box::new(EmaCross::new(5, 20));
        let mut vol_targeting = VolatilityTargeting::new(0.01, 20, base_strategy, 0.1, 5.0);
        
        // Simulate some returns with known volatility
        vol_targeting.returns_history = vec![0.005, -0.005, 0.005, -0.005]; // Low volatility
        vol_targeting.update_position_multiplier();
        
        // With low realized volatility, multiplier should be > 1
        assert!(vol_targeting.get_position_multiplier() > 1.0);
        
        // Simulate high volatility returns
        vol_targeting.returns_history = vec![0.05, -0.05, 0.05, -0.05]; // High volatility
        vol_targeting.update_position_multiplier();
        
        // With high realized volatility, multiplier should be < 1
        assert!(vol_targeting.get_position_multiplier() < 1.0);
    }

    #[test]
    fn test_multiplier_bounds() {
        let base_strategy = Box::new(EmaCross::new(5, 20));
        let mut vol_targeting = VolatilityTargeting::new(0.01, 20, base_strategy, 0.5, 2.0);
        
        // Test extreme low volatility (should hit max bound)
        vol_targeting.returns_history = vec![0.001; 20]; // Very low volatility
        vol_targeting.update_position_multiplier();
        assert!(vol_targeting.get_position_multiplier() <= 2.0);
        
        // Test extreme high volatility (should hit min bound)  
        vol_targeting.returns_history = vec![0.1, -0.1, 0.1, -0.1, 0.1, -0.1]; // Very high volatility
        vol_targeting.update_position_multiplier();
        assert!(vol_targeting.get_position_multiplier() >= 0.5);
    }
}