module deepbook_lp_vaults::vault {
    use sui::coin::{Self, Coin};
    use sui::transfer;
    use sui::tx_context::TxContext;
    use sui::object::{Self, UID, ID};
    use sui::balance::{Self, Balance};

    /// A capability to manage a specific Vault.
    public struct VaultCapability has key, store {
        id: UID,
        vault_id: ID,
    }

    /// A Vault object that holds deposited assets.
    public struct Vault has key, store {
        id: UID,
        asset_type: u8, // Placeholder for asset type (e.g., 0 for SUI, 1 for USDC)
        balance: Balance<sui::sui::SUI>,
        total_shares: u64,
        total_underlying: u64,
    }

    /// Error codes
    const EINVALID_AMOUNT: u64 = 0;

    /// Initializes the Vault module. Only callable once.
    #[allow(unused_variable)]
    fun init(_ctx: &mut TxContext) {
        // In a more complex system, you might create a global registry here
    }

    /// Creates a new Vault for a specific asset type.
    public fun create_vault(asset_type: u8, ctx: &mut TxContext): VaultCapability {
        let vault = Vault {
            id: object::new(ctx),
            asset_type,
            balance: balance::zero(),
            total_shares: 0,
            total_underlying: 0,
        };
        let vault_id = object::id(&vault);
        transfer::share_object(vault);
        VaultCapability {
            id: object::new(ctx),
            vault_id,
        }
    }

    /// Deposits coins into a Vault.
    #[allow(lint(public_entry))]
    public entry fun deposit(vault: &mut Vault, coins: Coin<sui::sui::SUI>, _ctx: &mut TxContext) {
        use deepbook_lp_vaults::vault_math;

        let deposit_amount = coin::value(&coins);
        assert!(deposit_amount > 0, EINVALID_AMOUNT);

        let shares_to_mint = if (vault.total_shares == 0 || vault.total_underlying == 0) {
            deposit_amount // First deposit, 1 share = 1 underlying
        } else {
            vault_math::calculate_shares_for_deposit(deposit_amount, vault.total_shares, vault.total_underlying)
        };

        let coin_balance = coin::into_balance(coins);
        balance::join(&mut vault.balance, coin_balance);
        vault.total_shares = vault.total_shares + shares_to_mint;
        vault.total_underlying = vault.total_underlying + deposit_amount;
    }

    /// Withdraws coins from a Vault.
    #[allow(lint(public_entry))]
    public entry fun withdraw(vault: &mut Vault, shares_to_burn: u64, recipient: address, ctx: &mut TxContext) {
        use deepbook_lp_vaults::vault_math;

        assert!(shares_to_burn > 0, EINVALID_AMOUNT);
        assert!(vault.total_shares >= shares_to_burn, EINVALID_AMOUNT);

        let amount_to_withdraw = vault_math::calculate_amount_for_withdrawal(shares_to_burn, vault.total_shares, vault.total_underlying);

        assert!(balance::value(&vault.balance) >= amount_to_withdraw, EINVALID_AMOUNT);

        let withdrawn_balance = balance::split(&mut vault.balance, amount_to_withdraw);
        let new_coin = coin::from_balance(withdrawn_balance, ctx);
        transfer::public_transfer(new_coin, recipient);

        vault.total_shares = vault.total_shares - shares_to_burn;
        vault.total_underlying = vault.total_underlying - amount_to_withdraw;
    }

    /// Get the total deposited amount in the vault.
    public fun get_balance(vault: &Vault): u64 {
        balance::value(&vault.balance)
    }

    // TODO: Add functions for managing strategy allocation within the vault
    // TODO: Add functions for integrating with DeepBook

    /// Get the vault ID from a VaultCapability
    public fun vault_id(cap: &VaultCapability): ID {
        cap.vault_id
    }
}


