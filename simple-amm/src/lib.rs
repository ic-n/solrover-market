//! SimpleAMM — constant-product AMM (xy=k) for use as a perp price oracle.
//!
//! Price output is in Percolator format: quote per 1 base, scaled by 1e6.
//! So a price of 1.0 USDC/Iron = 1_000_000u64.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmmError {
    ZeroAmount,
    EmptyPool,
    Overflow,
    InsufficientOutput,
    InsufficientLiquidity,
    InsufficientBalance,
}

/// Constant-product AMM.
///
/// Invariant: reserve_base * reserve_quote = k (non-decreasing, fees stay in pool).
pub struct SimpleAMM {
    pub reserve_base: u128,
    pub reserve_quote: u128,
    pub lp_supply: u128,
    pub fee_bps: u16,
}

impl SimpleAMM {
    /// Initialize pool with reserves. `lp_supply` is set to `min(base, quote)`.
    pub fn new(reserve_base: u128, reserve_quote: u128, fee_bps: u16) -> Self {
        assert!(reserve_base > 0 && reserve_quote > 0, "reserves must be non-zero");
        Self {
            reserve_base,
            reserve_quote,
            lp_supply: reserve_base.min(reserve_quote),
            fee_bps,
        }
    }

    /// Current oracle price: quote per 1 base, scaled by 1e6.
    /// This is the value Percolator expects as `oracle_price`.
    pub fn spot_price(&self) -> u64 {
        (self.reserve_quote * 1_000_000 / self.reserve_base) as u64
    }

    /// Sell `amount_in` base, receive quote.
    /// Fee stays in pool (increases k slightly).
    pub fn swap_base_to_quote(&mut self, amount_in: u128) -> Result<u128, AmmError> {
        if amount_in == 0 {
            return Err(AmmError::ZeroAmount);
        }
        if self.reserve_base == 0 || self.reserve_quote == 0 {
            return Err(AmmError::EmptyPool);
        }

        let fee = self.fee_bps as u128;
        let amount_in_after_fee = amount_in
            .checked_mul(10_000 - fee)
            .ok_or(AmmError::Overflow)?
            / 10_000;

        let numerator = self
            .reserve_quote
            .checked_mul(amount_in_after_fee)
            .ok_or(AmmError::Overflow)?;
        let denominator = self
            .reserve_base
            .checked_add(amount_in_after_fee)
            .ok_or(AmmError::Overflow)?;

        let amount_out = numerator / denominator;
        if amount_out == 0 {
            return Err(AmmError::InsufficientOutput);
        }
        if amount_out >= self.reserve_quote {
            return Err(AmmError::InsufficientLiquidity);
        }

        // Add full amount_in to reserves (fee stays in pool)
        self.reserve_base = self
            .reserve_base
            .checked_add(amount_in)
            .ok_or(AmmError::Overflow)?;
        self.reserve_quote -= amount_out;

        Ok(amount_out)
    }

    /// Buy `amount_out`-worth of base by spending quote.
    /// `amount_in` is the quote token input; returns base received.
    pub fn swap_quote_to_base(&mut self, amount_in: u128) -> Result<u128, AmmError> {
        if amount_in == 0 {
            return Err(AmmError::ZeroAmount);
        }
        if self.reserve_base == 0 || self.reserve_quote == 0 {
            return Err(AmmError::EmptyPool);
        }

        let fee = self.fee_bps as u128;
        let amount_in_after_fee = amount_in
            .checked_mul(10_000 - fee)
            .ok_or(AmmError::Overflow)?
            / 10_000;

        let numerator = self
            .reserve_base
            .checked_mul(amount_in_after_fee)
            .ok_or(AmmError::Overflow)?;
        let denominator = self
            .reserve_quote
            .checked_add(amount_in_after_fee)
            .ok_or(AmmError::Overflow)?;

        let amount_out = numerator / denominator;
        if amount_out == 0 {
            return Err(AmmError::InsufficientOutput);
        }
        if amount_out >= self.reserve_base {
            return Err(AmmError::InsufficientLiquidity);
        }

        // Add full amount_in to reserves (fee stays in pool)
        self.reserve_quote = self
            .reserve_quote
            .checked_add(amount_in)
            .ok_or(AmmError::Overflow)?;
        self.reserve_base -= amount_out;

        Ok(amount_out)
    }

    /// Add liquidity. Returns LP tokens minted.
    ///
    /// For the initial deposit, LP tokens = min(base, quote).
    /// For subsequent deposits, tokens are proportional to existing reserves.
    pub fn add_liquidity(&mut self, base: u128, quote: u128) -> Result<u128, AmmError> {
        if base == 0 || quote == 0 {
            return Err(AmmError::ZeroAmount);
        }

        let lp_out = if self.lp_supply == 0 {
            base.min(quote)
        } else {
            let lp_b = base
                .checked_mul(self.lp_supply)
                .ok_or(AmmError::Overflow)?
                / self.reserve_base;
            let lp_q = quote
                .checked_mul(self.lp_supply)
                .ok_or(AmmError::Overflow)?
                / self.reserve_quote;
            lp_b.min(lp_q)
        };

        if lp_out == 0 {
            return Err(AmmError::InsufficientOutput);
        }

        self.reserve_base = self
            .reserve_base
            .checked_add(base)
            .ok_or(AmmError::Overflow)?;
        self.reserve_quote = self
            .reserve_quote
            .checked_add(quote)
            .ok_or(AmmError::Overflow)?;
        self.lp_supply = self
            .lp_supply
            .checked_add(lp_out)
            .ok_or(AmmError::Overflow)?;

        Ok(lp_out)
    }

    /// Burn `lp_amount` LP tokens, receive proportional (base, quote).
    pub fn remove_liquidity(&mut self, lp_amount: u128) -> Result<(u128, u128), AmmError> {
        if lp_amount == 0 {
            return Err(AmmError::ZeroAmount);
        }
        if lp_amount > self.lp_supply {
            return Err(AmmError::InsufficientBalance);
        }

        let base_out = self.reserve_base * lp_amount / self.lp_supply;
        let quote_out = self.reserve_quote * lp_amount / self.lp_supply;

        if base_out == 0 || quote_out == 0 {
            return Err(AmmError::InsufficientOutput);
        }

        self.reserve_base -= base_out;
        self.reserve_quote -= quote_out;
        self.lp_supply -= lp_amount;

        Ok((base_out, quote_out))
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn amm() -> SimpleAMM {
        // 1M Iron / 1M USDC, 2% fee — standard test params
        SimpleAMM::new(1_000_000, 1_000_000, 200)
    }

    #[test]
    fn test_spot_price_initial() {
        let amm = amm();
        // 1M quote / 1M base * 1e6 = 1_000_000
        assert_eq!(amm.spot_price(), 1_000_000);
    }

    #[test]
    fn test_spot_price_matches_reserves() {
        let amm = SimpleAMM::new(2_000_000, 3_000_000, 0);
        // 3M/2M * 1e6 = 1_500_000
        assert_eq!(amm.spot_price(), 1_500_000);
    }

    #[test]
    fn test_swap_quote_to_base_moves_price_up() {
        let mut amm = amm();
        let price_before = amm.spot_price();
        let _base_out = amm.swap_quote_to_base(100_000).unwrap();
        let price_after = amm.spot_price();
        // Buying base with quote drives price up (more quote per base)
        assert!(price_after > price_before, "price should rise after buying base");
    }

    #[test]
    fn test_swap_base_to_quote_moves_price_down() {
        let mut amm = amm();
        let price_before = amm.spot_price();
        let _quote_out = amm.swap_base_to_quote(100_000).unwrap();
        let price_after = amm.spot_price();
        // Selling base for quote drives price down
        assert!(price_after < price_before, "price should fall after selling base");
    }

    #[test]
    fn test_fee_reduces_output() {
        let mut amm_fee = SimpleAMM::new(1_000_000, 1_000_000, 200); // 2% fee
        let mut amm_no_fee = SimpleAMM::new(1_000_000, 1_000_000, 0); // no fee

        let out_fee = amm_fee.swap_quote_to_base(10_000).unwrap();
        let out_no_fee = amm_no_fee.swap_quote_to_base(10_000).unwrap();

        assert!(out_fee < out_no_fee, "fee should reduce output");
    }

    #[test]
    fn test_k_non_decreasing_after_swap() {
        let mut amm = amm();
        let k_before = amm.reserve_base * amm.reserve_quote;
        amm.swap_quote_to_base(50_000).unwrap();
        let k_after = amm.reserve_base * amm.reserve_quote;
        assert!(k_after >= k_before, "k must not decrease (fee stays in pool)");
    }

    #[test]
    fn test_add_remove_liquidity_roundtrip() {
        let mut amm = SimpleAMM::new(1_000_000, 1_000_000, 0);
        let initial_lp = amm.lp_supply;
        let lp_minted = amm.add_liquidity(100_000, 100_000).unwrap();
        let (base_back, quote_back) = amm.remove_liquidity(lp_minted).unwrap();
        // Should get back approximately what was put in (rounding down)
        assert!(base_back >= 99_000, "base back: {base_back}");
        assert!(quote_back >= 99_000, "quote back: {quote_back}");
        assert_eq!(amm.lp_supply, initial_lp);
    }

    #[test]
    fn test_zero_amount_errors() {
        let mut amm = amm();
        assert!(matches!(amm.swap_quote_to_base(0), Err(AmmError::ZeroAmount)));
        assert!(matches!(amm.swap_base_to_quote(0), Err(AmmError::ZeroAmount)));
        assert!(matches!(amm.add_liquidity(0, 100), Err(AmmError::ZeroAmount)));
        assert!(matches!(amm.remove_liquidity(0), Err(AmmError::ZeroAmount)));
    }
}
