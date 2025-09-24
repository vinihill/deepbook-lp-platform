module deepbook_lp_registry::registry {
    use sui::object::{Self, UID, ID};
    use sui::tx_context::TxContext;
    use sui::table::{Self, Table};
    use sui::transfer;

    use deepbook_lp_vaults::vault::{Vault, VaultCapability};
    use deepbook_lp_strategies::strategy::Strategy;
    use deepbook_lp_risk_controls::risk_control::RiskControl;

    /// A Registry object that stores references to deployed Vaults, Strategies, and RiskControls.
    public struct Registry has key, store {
        id: UID,
        vaults: Table<ID, ID>,
        strategies: Table<ID, ID>,
        risk_controls: Table<ID, ID>,
    }

    /// Initializes the Registry module, creating a shared Registry object.
    fun init(ctx: &mut TxContext) {
        let registry = Registry {
            id: object::new(ctx),
            vaults: table::new(ctx),
            strategies: table::new(ctx),
            risk_controls: table::new(ctx),
        };
        transfer::share_object(registry);
    }

    /// Registers a new Vault with the Registry.
    public fun register_vault(self: &mut Registry, vault: Vault, _ctx: &mut TxContext) {
        let vault_id = object::id(&vault);
        table::add(&mut self.vaults, vault_id, vault_id);
        transfer::public_share_object(vault);
    }

    /// Registers a new Strategy with the Registry.
    public fun register_strategy(self: &mut Registry, strategy: Strategy, _ctx: &mut TxContext) {
        let strategy_id = object::id(&strategy);
        table::add(&mut self.strategies, strategy_id, strategy_id);
        transfer::public_share_object(strategy);
    }

    /// Registers a new RiskControl with the Registry.
    public fun register_risk_control(self: &mut Registry, risk_control: RiskControl, _ctx: &mut TxContext) {
        let risk_control_id = object::id(&risk_control);
        table::add(&mut self.risk_controls, risk_control_id, risk_control_id);
        transfer::public_share_object(risk_control);
    }

    /// Retrieves a Vault by its ID.
    public fun get_vault_id(self: &Registry, vault_id: ID): ID {
        *table::borrow(&self.vaults, vault_id)
    }

    /// Retrieves a Strategy by its ID.
    public fun get_strategy_id(self: &Registry, strategy_id: ID): ID {
        *table::borrow(&self.strategies, strategy_id)
    }

    /// Retrieves a RiskControl by its ID.
    public fun get_risk_control_id(self: &Registry, risk_control_id: ID): ID {
        *table::borrow(&self.risk_controls, risk_control_id)
    }

    // TODO: Add functions for unregistering and updating entries
}


