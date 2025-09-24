module deepbook_lp_risk_controls::risk_control {
    use sui::object::{Self, UID};
    use sui::tx_context::TxContext;
    use sui::transfer;

    /// A RiskControl object that defines and enforces risk parameters for a strategy or vault.
    public struct RiskControl has key, store {
        id: UID,
        drawdown_limit: u64, // Percentage, e.g., 1000 for 10%
        circuit_breaker_active: bool,
        timelock_duration: u64, // In seconds
        // Add more risk parameters as needed
    }

    /// Creates a new RiskControl object.
    public fun create_risk_control(
        drawdown_limit: u64,
        timelock_duration: u64,
        ctx: &mut TxContext
    ): RiskControl {
        RiskControl {
            id: object::new(ctx),
            drawdown_limit,
            circuit_breaker_active: false,
            timelock_duration,
        }
    }

    /// Updates the drawdown limit for a RiskControl object.
    public fun update_drawdown_limit(self: &mut RiskControl, new_limit: u64, _ctx: &mut TxContext) {
        self.drawdown_limit = new_limit;
    }

    /// Activates the circuit breaker, pausing strategy execution.
    public fun activate_circuit_breaker(self: &mut RiskControl, _ctx: &mut TxContext) {
        self.circuit_breaker_active = true;
    }

    /// Deactivates the circuit breaker, allowing strategy execution to resume.
    public fun deactivate_circuit_breaker(self: &mut RiskControl, _ctx: &mut TxContext) {
        self.circuit_breaker_active = false;
    }

    /// Checks if the circuit breaker is active.
    public fun is_circuit_breaker_active(self: &RiskControl): bool {
        self.circuit_breaker_active
    }

    /// Transfers ownership of the RiskControl object.
    public fun transfer_risk_control(self: RiskControl, recipient: address) {
        transfer::public_transfer(self, recipient);
    }
}


