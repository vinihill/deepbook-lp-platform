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
        let amount = coin::value(&coins);
        assert!(amount > 0, EINVALID_AMOUNT);
        let coin_balance = coin::into_balance(coins);
        balance::join(&mut vault.balance, coin_balance);
    }

    /// Withdraws coins from a Vault.
    #[allow(lint(public_entry))]
    public entry fun withdraw(vault: &mut Vault, amount: u64, recipient: address, ctx: &mut TxContext) {
        assert!(balance::value(&vault.balance) >= amount, EINVALID_AMOUNT);
        let withdrawn_balance = balance::split(&mut vault.balance, amount);
        let new_coin = coin::from_balance(withdrawn_balance, ctx);
        transfer::public_transfer(new_coin, recipient);
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


