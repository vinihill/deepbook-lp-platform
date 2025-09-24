module deepbook_lp_accounting::accounting {
    use sui::object::{Self, UID};
    use sui::tx_context::TxContext;
    use sui::transfer;

    /// A Position object that tracks a user's individual allocation within a Vault and Strategy.
    public struct Position has key, store {
        id: UID,
        user_address: address,
        vault_id: UID,
        strategy_id: UID,
        amount_deposited: u64,
        entry_timestamp: u64,
        pnl_snapshot: u64, // Snapshot of P&L at entry or last rebalance
        // Add more fields for detailed performance tracking (e.g., fees earned, impermanent loss)
    }

    /// Creates a new Position object for a user.
    public fun create_position(
        user: address,
        vault_id: UID,
        strategy_id: UID,
        amount: u64,
        ctx: &mut TxContext
    ): Position {
        Position {
            id: object::new(ctx),
            user_address: user,
            vault_id,
            strategy_id,
            amount_deposited: amount,
            entry_timestamp: sui::tx_context::epoch_timestamp_ms(ctx),
            pnl_snapshot: 0, // Initial P&L is 0
        }
    }

    /// Updates the amount deposited in a Position.
    public fun update_deposited_amount(self: &mut Position, new_amount: u64, _ctx: &mut TxContext) {
        self.amount_deposited = new_amount;
    }

    /// Updates the P&L snapshot for a Position.
    public fun update_pnl_snapshot(self: &mut Position, new_pnl: u64, _ctx: &mut TxContext) {
        self.pnl_snapshot = new_pnl;
    }

    /// Transfers ownership of the Position object.
    public fun transfer_position(self: Position, recipient: address) {
        transfer::public_transfer(self, recipient);
    }
}


