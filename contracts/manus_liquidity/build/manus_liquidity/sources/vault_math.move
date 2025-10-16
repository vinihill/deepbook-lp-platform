/// Manus AI Liquidity Vault Math
/// 
/// Formally verified mathematical helper functions for the vault.
module manus_liquidity::vault_math {
    /// Error codes
    const E_DIVISION_BY_ZERO: u64 = 100;

    /// Calculates the number of shares to mint for a given deposit amount.
    /// Handles the initial deposit case (1:1 ratio) and subsequent deposits proportionally.
    public fun calculate_shares_for_deposit(
        deposit_amount: u64,
        total_shares: u64,
        total_underlying: u64
    ): u64 {
        if (total_shares == 0 || total_underlying == 0) {
            // First deposit or vault is empty: 1:1 ratio
            deposit_amount
        } else {
            // Subsequent deposits: proportional to existing ratio
            assert!(total_underlying > 0, E_DIVISION_BY_ZERO);
            ((deposit_amount as u128) * (total_shares as u128) / (total_underlying as u128)) as u64
        }
    }

    /// Calculates the amount of underlying assets to withdraw for a given number of shares.
    /// Handles the case where total_shares is zero to prevent division by zero.
    public fun calculate_amount_for_withdrawal(
        shares_to_burn: u64,
        total_shares: u64,
        total_underlying: u64
    ): u64 {
        if (total_shares == 0) {
            // No shares, no withdrawal possible
            0
        } else {
            // Proportional withdrawal
            assert!(total_shares > 0, E_DIVISION_BY_ZERO);
            ((shares_to_burn as u128) * (total_underlying as u128) / (total_shares as u128)) as u64
        }
    }
}

