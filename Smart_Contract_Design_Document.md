# Smart Contract Design Document for DeepBook Liquidity Provisioning

## 1. Introduction

This document details the architectural design of the Move smart contracts for the DeepBook Liquidity Provisioning application on the Sui blockchain. The design prioritizes modularity, security, auditability, and upgradeability, aligning with the requirements outlined in the Request for Proposal (RFP).

## 2. Core Principles

*   **Modularity:** Separation of concerns into distinct Move packages/modules for vaults, strategies, risk controls, and user accounting.
*   **Security:** Leveraging Move's safety features, rigorous access control, and internal checks.
*   **Auditability:** Clean, well-structured code that facilitates external security audits.
*   **Upgradeability:** Designing contracts to allow for future enhancements and bug fixes without requiring a complete redeployment of the entire system.
*   **Composability:** Enabling seamless interaction with DeepBook and other Sui ecosystem protocols.

## 3. High-Level Architecture

The smart contract architecture will consist of several interconnected Move packages, each responsible for a specific set of functionalities:

*   **`deepbook_lp_vaults`:** Manages user deposits, withdrawals, and capital allocation to various strategies. It will hold the underlying assets (SUI, stablecoins, DEEP).
*   **`deepbook_lp_strategies`:** Contains the logic for different liquidity provisioning strategies (e.g., passive AMM overlays, active market making). Strategies will interact with DeepBook's CLOB.
*   **`deepbook_lp_risk_controls`:** Implements safety mechanisms such as timelocks, circuit breakers, and drawdown limits.
*   **`deepbook_lp_accounting`:** Handles user-specific accounting, performance tracking (P&L, drawdowns), and liquidity contributions.
*   **`deepbook_lp_registry`:** A central registry to manage and discover deployed vaults and strategies.

## 4. Data Models (On-Chain Objects)

Key on-chain objects will be defined to represent the state of the liquidity provisioning system:

### 4.1 `Vault` Object

Represents a pool of assets deposited by users for a specific strategy or set of strategies. Each `Vault` will have a unique ID and manage a particular asset type (e.g., SUI, USDC).

**Fields:**
*   `id: UID`
*   `asset_type: Type` (e.g., `sui::sui::SUI`, `sui::coin::Coin<USDC>`) 
*   `total_deposited: u64`
*   `strategy_id: UID` (reference to the associated strategy)
*   `owner: address` (the module that controls the vault, e.g., `deepbook_lp_registry`)
*   `locked_until: u64` (for timelocks)

### 4.2 `Strategy` Object

Defines the parameters and logic for a specific liquidity provisioning strategy. Strategies will be deployed as immutable objects or upgradeable packages.

**Fields:**
*   `id: UID`
*   `name: String`
*   `description: String`
*   `parameters: Vec<u8>` (serialized strategy-specific parameters like tick range, rebalancing frequency)
*   `logic_package_id: ObjectID` (reference to the Move package containing the strategy's execution logic)
*   `risk_control_id: UID` (reference to associated risk control object)

### 4.3 `Position` Object

Represents a user's individual allocation within a `Vault` and `Strategy`.

**Fields:**
*   `id: UID`
*   `user_address: address`
*   `vault_id: UID`
*   `strategy_id: UID`
*   `amount_deposited: u64`
*   `entry_timestamp: u64`
*   `pnl_snapshot: u64` (snapshot of P&L at entry or last rebalance)

### 4.4 `RiskControl` Object

Encapsulates the parameters and state for risk management associated with a strategy or vault.

**Fields:**
*   `id: UID`
*   `drawdown_limit: u64` (percentage)
*   `circuit_breaker_active: bool`
*   `timelock_duration: u64` (in seconds)
*   `monitoring_data: Vec<u8>` (serialized data for real-time monitoring)

## 5. Module Interactions and Control Flow

### 5.1 Deposit Flow

1.  User calls `deepbook_lp_vaults::deposit(vault_id, amount, strategy_id)`.
2.  `deepbook_lp_vaults` verifies `vault_id` and `strategy_id` via `deepbook_lp_registry`.
3.  `deepbook_lp_vaults` transfers assets from user to the vault's treasury.
4.  `deepbook_lp_accounting` updates user's `Position` object.
5.  `deepbook_lp_strategies` is notified to potentially adjust liquidity on DeepBook based on new capital.

### 5.2 Strategy Execution Flow

1.  Automated trigger (e.g., off-chain keeper, time-based event) or user action calls `deepbook_lp_strategies::execute_strategy(strategy_id)`.
2.  `deepbook_lp_strategies` retrieves strategy parameters and current market data (potentially via oracles or DeepBook queries).
3.  `deepbook_lp_strategies` interacts with DeepBook's CLOB to place/cancel orders.
4.  `deepbook_lp_accounting` updates P&L and liquidity contribution metrics.
5.  `deepbook_lp_risk_controls` checks for any breaches (e.g., drawdown limits, circuit breakers) and takes appropriate action (e.g., pause strategy, notify).

### 5.3 Withdrawal Flow

1.  User calls `deepbook_lp_vaults::withdraw(position_id, amount)`.
2.  `deepbook_lp_vaults` verifies `position_id` and checks `deepbook_lp_risk_controls` for any active timelocks or circuit breakers.
3.  If conditions met, `deepbook_lp_vaults` transfers assets from vault treasury to user.
4.  `deepbook_lp_accounting` updates user's `Position` object.

## 6. Access Control and Ownership

*   **Vaults:** Owned by the `deepbook_lp_registry` or a designated admin multisig for management and upgrades.
*   **Strategies:** Deployed by authorized entities (e.g., the protocol itself, or whitelisted developers).
*   **Positions:** Owned by individual users.
*   **Risk Controls:** Owned by the `deepbook_lp_registry` or associated strategy modules.

Move's ownership model will be leveraged to enforce strict access control, ensuring that only authorized entities can modify or interact with specific objects.

## 7. Upgradeability Considerations

Sui Move supports module upgradeability. The design will consider:

*   **Versioned Modules:** Each module will be versioned to manage upgrades.
*   **Data Migration:** Strategies for migrating on-chain data when object structures change.
*   **Proxy Patterns:** Potentially using proxy contracts for critical components to enable seamless upgrades without changing user entry points.

## 8. Integration with DeepBook

Strategies will directly interact with DeepBook's public interfaces for placing and managing orders. This will involve understanding DeepBook's specific Move module APIs for order creation, cancellation, and querying market data.

## 9. Testing Strategy

*   **Unit Tests:** Extensive unit tests for each function within every Move module.
*   **Integration Tests:** Testing interactions between `deepbook_lp_vaults`, `deepbook_lp_strategies`, `deepbook_lp_risk_controls`, and `deepbook_lp_accounting`.
*   **DeepBook Mocking:** Developing mock DeepBook modules for isolated testing of strategies.
*   **Fuzzing and Property-Based Testing:** Utilizing advanced testing techniques to uncover edge cases and vulnerabilities.

## 10. Future Considerations

*   **External Oracle Integration:** For more sophisticated strategies requiring off-chain data.
*   **Governance Module:** For decentralized control over protocol parameters and upgrades.
*   **Flash Loan Protection:** Implementing mechanisms to prevent manipulation via flash loans.


