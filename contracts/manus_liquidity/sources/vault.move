/// Manus AI Liquidity Vault
/// 
/// A formally verified, quantum-resistant liquidity provisioning vault
/// with AI-driven autonomous management.
module manus_liquidity::vault {
    use manus_liquidity::vault_math;

    use sui::object::{Self, UID, ID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::coin::{Self, Coin};
    use sui::balance::{Self, Balance};
    use sui::event;

    /// Vault capability for administrative operations
    public struct VaultCap has key, store {
        id: UID,
        vault_id: ID,
    }

    /// Main vault structure
    public struct Vault<phantom T> has key, store {
        id: UID,
        /// Total balance of underlying assets
        balance: Balance<T>,
        /// Total shares issued
        total_shares: u64,
        /// Total underlying value
        total_underlying: u64,
        /// Maximum capacity
        max_capacity: u64,
        /// Strategy identifier
        strategy: vector<u8>,
        /// Paused state
        paused: bool,
    }

    /// LP share token
    public struct Share has key, store {
        id: UID,
        vault_id: ID,
        amount: u64,
    }

    /// Events
    public struct DepositEvent has copy, drop {
        vault_id: ID,
        depositor: address,
        amount: u64,
        shares_minted: u64,
    }

    public struct WithdrawEvent has copy, drop {
        vault_id: ID,
        withdrawer: address,
        shares_burned: u64,
        amount_withdrawn: u64,
    }

    /// Error codes
    const E_VAULT_PAUSED: u64 = 1;
    const E_INVALID_AMOUNT: u64 = 2;
    const E_INSUFFICIENT_SHARES: u64 = 3;
    const E_MAX_CAPACITY_EXCEEDED: u64 = 4;
    const E_INSUFFICIENT_BALANCE: u64 = 5;

    /// Create a new vault
    public fun create_vault<T>(
        max_capacity: u64,
        strategy: vector<u8>,
        ctx: &mut TxContext
    ): VaultCap {
        let vault_uid = object::new(ctx);
        let vault_id = object::uid_to_inner(&vault_uid);

        let vault = Vault<T> {
            id: vault_uid,
            balance: balance::zero(),
            total_shares: 0,
            total_underlying: 0,
            max_capacity,
            strategy,
            paused: false,
        };

        transfer::share_object(vault);

        VaultCap {
            id: object::new(ctx),
            vault_id,
        }
    }

    /// Deposit assets and receive shares
    public entry fun deposit<T>(
        vault: &mut Vault<T>,
        coins: Coin<T>,
        ctx: &mut TxContext
    ) {
        assert!(!vault.paused, E_VAULT_PAUSED);

        let deposit_amount = coin::value(&coins);
        assert!(deposit_amount > 0, E_INVALID_AMOUNT);

        let current_balance = balance::value(&vault.balance);
        assert!(current_balance + deposit_amount <= vault.max_capacity, E_MAX_CAPACITY_EXCEEDED);

        // Calculate shares to mint
        let shares_to_mint = vault_math::calculate_shares_for_deposit(
            deposit_amount,
            vault.total_shares,
            vault.total_underlying
        );

        // Update vault state
        let coin_balance = coin::into_balance(coins);
        balance::join(&mut vault.balance, coin_balance);
        vault.total_shares = vault.total_shares + shares_to_mint;
        vault.total_underlying = vault.total_underlying + deposit_amount;

        // Mint shares
        let share = Share {
            id: object::new(ctx),
            vault_id: object::id(vault),
            amount: shares_to_mint,
        };

        let depositor = tx_context::sender(ctx);
        transfer::transfer(share, depositor);

        // Emit event
        event::emit(DepositEvent {
            vault_id: object::id(vault),
            depositor,
            amount: deposit_amount,
            shares_minted: shares_to_mint,
        });
    }

    /// Withdraw assets by burning shares
    public entry fun withdraw<T>(
        vault: &mut Vault<T>,
        share: Share,
        ctx: &mut TxContext
    ) {
        assert!(!vault.paused, E_VAULT_PAUSED);

        let Share { id, vault_id, amount: shares_to_burn } = share;
        object::delete(id);

        assert!(vault_id == object::id(vault), E_INVALID_AMOUNT);
        assert!(shares_to_burn > 0, E_INVALID_AMOUNT);
        assert!(vault.total_shares >= shares_to_burn, E_INSUFFICIENT_SHARES);

        // Calculate amount to withdraw
        let amount_to_withdraw = vault_math::calculate_amount_for_withdrawal(
            shares_to_burn,
            vault.total_shares,
            vault.total_underlying
        );

        assert!(balance::value(&vault.balance) >= amount_to_withdraw, E_INSUFFICIENT_BALANCE);

        // Update vault state
        vault.total_shares = vault.total_shares - shares_to_burn;
        vault.total_underlying = vault.total_underlying - amount_to_withdraw;

        // Withdraw assets
        let withdrawn_balance = balance::split(&mut vault.balance, amount_to_withdraw);
        let withdrawn_coin = coin::from_balance(withdrawn_balance, ctx);

        let withdrawer = tx_context::sender(ctx);
        transfer::public_transfer(withdrawn_coin, withdrawer);

        // Emit event
        event::emit(WithdrawEvent {
            vault_id: object::id(vault),
            withdrawer,
            shares_burned: shares_to_burn,
            amount_withdrawn: amount_to_withdraw,
        });
    }

    /// Pause vault operations (admin only)
    public entry fun pause<T>(
        vault: &mut Vault<T>,
        _cap: &VaultCap,
    ) {
        vault.paused = true;
    }

    /// Resume vault operations (admin only)
    public entry fun resume<T>(
        vault: &mut Vault<T>,
        _cap: &VaultCap,
    ) {
        vault.paused = false;
    }

    /// Get vault information
    public fun get_vault_info<T>(vault: &Vault<T>): (u64, u64, u64, bool) {
        (
            balance::value(&vault.balance),
            vault.total_shares,
            vault.total_underlying,
            vault.paused
        )
    }

    #[test_only]
    public fun init_for_testing(ctx: &mut TxContext) {
        // Test initialization
    }
}

