# API Specification for DeepBook Liquidity Provisioning Application

## 1. Introduction

This document outlines the preliminary API specifications for the DeepBook Liquidity Provisioning application. These APIs will serve as the communication layer between the web-based frontend and the underlying Sui Move smart contracts, enabling users to interact with vaults, strategies, and performance monitoring features.

## 2. API Design Principles

*   **RESTful Principles (where applicable):** Although interacting with a blockchain, the API will aim for a logical, resource-oriented structure for clarity.
*   **Clear Separation of Concerns:** APIs will be grouped by functionality (e.g., Vaults, Strategies, User Data).
*   **Security:** All interactions will leverage Sui wallet signatures for authentication and authorization.
*   **Efficiency:** Minimize redundant data transfer and optimize for common use cases.

## 3. Core API Endpoints (Preliminary)

### 3.1 Vault Management

**Purpose:** To allow users to deposit and withdraw assets from liquidity vaults.

#### `POST /vaults/deposit`

*   **Description:** Initiates a deposit of assets into a specified vault.
*   **Request Body:**
    ```json
    {
        "user_address": "0x...",
        "vault_id": "0x...",
        "asset_type": "SUI",
        "amount": "1000000000" (in base units)
    }
    ```
*   **Response:**
    ```json
    {
        "transaction_digest": "0x...",
        "status": "success" // or "pending", "failed"
    }
    ```

#### `POST /vaults/withdraw`

*   **Description:** Initiates a withdrawal of assets from a specified vault.
*   **Request Body:**
    ```json
    {
        "user_address": "0x...",
        "position_id": "0x...",
        "amount": "500000000" (in base units)
    }
    ```
*   **Response:**
    ```json
    {
        "transaction_digest": "0x...",
        "status": "success"
    }
    ```

#### `GET /vaults/{vault_id}`

*   **Description:** Retrieves details of a specific vault.
*   **Response:**
    ```json
    {
        "id": "0x...",
        "asset_type": "SUI",
        "total_deposited": "10000000000",
        "strategy_id": "0x...",
        "locked_until": "1735689600" // Unix timestamp
    }
    ```

### 3.2 Strategy Management

**Purpose:** To allow users to select, configure, and manage liquidity provisioning strategies.

#### `GET /strategies`

*   **Description:** Retrieves a list of available predefined strategies.
*   **Response:**
    ```json
    [
        {
            "id": "0x...",
            "name": "Passive AMM Overlay",
            "description": "Provides liquidity across a wide range.",
            "configurable_parameters": [
                {"name": "tick_range", "type": "string", "default": "wide"},
                {"name": "rebalance_frequency", "type": "string", "default": "daily"}
            ]
        },
        // ... more strategies
    ]
    ```

#### `POST /strategies/configure`

*   **Description:** Configures a new or existing strategy for a user.
*   **Request Body:**
    ```json
    {
        "user_address": "0x...",
        "strategy_template_id": "0x...", // ID from /strategies
        "vault_id": "0x...",
        "parameters": {
            "tick_range": "medium",
            "rebalance_frequency": "hourly"
        }
    }
    ```
*   **Response:**
    ```json
    {
        "strategy_instance_id": "0x...",
        "transaction_digest": "0x...",
        "status": "success"
    }
    ```

### 3.3 Performance Monitoring

**Purpose:** To provide users with real-time and historical performance data.

#### `GET /users/{user_address}/positions`

*   **Description:** Retrieves all active liquidity positions for a given user.
*   **Response:**
    ```json
    [
        {
            "id": "0x...",
            "vault_id": "0x...",
            "strategy_id": "0x...",
            "amount_deposited": "1000000000",
            "current_pnl": "50000000", // in base units
            "drawdown": "10000000",
            "liquidity_contribution": "0x..." // reference to DeepBook liquidity object
        },
        // ... more positions
    ]
    ```

#### `GET /strategies/{strategy_id}/performance`

*   **Description:** Retrieves aggregate performance data for a specific strategy.
*   **Response:**
    ```json
    {
        "strategy_id": "0x...",
        "total_pnl": "100000000",
        "average_drawdown": "5000000",
        "total_liquidity_contributed": "0x...",
        "historical_data": [
            {"timestamp": "1735603200", "pnl": "10000000"},
            {"timestamp": "1735689600", "pnl": "15000000"}
        ]
    }
    ```

### 3.4 Risk Management

**Purpose:** To allow users to view and configure risk parameters for their strategies.

#### `GET /strategies/{strategy_id}/risk_controls`

*   **Description:** Retrieves the current risk control settings for a strategy.
*   **Response:**
    ```json
    {
        "id": "0x...",
        "drawdown_limit": "10", // percentage
        "circuit_breaker_active": false,
        "timelock_duration": "86400" // seconds
    }
    ```

#### `POST /strategies/{strategy_id}/risk_controls/update`

*   **Description:** Updates risk control parameters for a strategy.
*   **Request Body:**
    ```json
    {
        "user_address": "0x...",
        "strategy_id": "0x...",
        "drawdown_limit": "15"
    }
    ```
*   **Response:**
    ```json
    {
        "transaction_digest": "0x...",
        "status": "success"
    }
    ```

## 4. Authentication and Authorization

All state-changing operations will require the user to sign a transaction with their Sui wallet. Read-only operations may not require a signature but will be rate-limited.

## 5. Error Handling

API responses will include clear error codes and messages for failed operations.

## 6. Future Considerations

*   **Webhooks:** For real-time notifications on strategy performance or risk events.
*   **Batch Operations:** For more efficient interaction with multiple positions or strategies.
*   **Custom Strategy Upload:** API for deploying custom Move modules as new strategies.


