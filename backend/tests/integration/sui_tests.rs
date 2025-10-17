//! Integration tests for Sui blockchain interaction
//!
//! Tests the interaction with Sui network and smart contracts

use manus_backend::sui::{SuiClient, VaultOperation};
use manus_backend::error::Result;

#[tokio::test]
#[ignore] // Requires Sui testnet/devnet access
async fn test_sui_client_connection() -> Result<()> {
    let client = SuiClient::new("https://fullnode.testnet.sui.io:443").await?;
    
    // Verify connection
    assert!(client.is_connected().await?);
    
    Ok(())
}

#[tokio::test]
#[ignore] // Requires Sui testnet/devnet access
async fn test_vault_deposit() -> Result<()> {
    let client = SuiClient::new("https://fullnode.testnet.sui.io:443").await?;
    
    // Test deposit operation
    let deposit_amount = 1_000_000; // 1 SUI
    let operation = VaultOperation::Deposit {
        amount: deposit_amount,
        asset: "SUI".to_string(),
    };
    
    let tx_result = client.execute_vault_operation(operation).await?;
    assert!(tx_result.is_success());
    
    Ok(())
}

#[tokio::test]
#[ignore] // Requires Sui testnet/devnet access
async fn test_vault_withdraw() -> Result<()> {
    let client = SuiClient::new("https://fullnode.testnet.sui.io:443").await?;
    
    // Test withdraw operation
    let withdraw_amount = 500_000; // 0.5 SUI
    let operation = VaultOperation::Withdraw {
        amount: withdraw_amount,
        asset: "SUI".to_string(),
    };
    
    let tx_result = client.execute_vault_operation(operation).await?;
    assert!(tx_result.is_success());
    
    Ok(())
}

#[tokio::test]
#[ignore] // Requires Sui testnet/devnet access
async fn test_get_vault_balance() -> Result<()> {
    let client = SuiClient::new("https://fullnode.testnet.sui.io:443").await?;
    
    let balance = client.get_vault_balance("SUI").await?;
    assert!(balance >= 0);
    
    Ok(())
}

#[tokio::test]
#[ignore] // Requires Sui testnet/devnet access
async fn test_get_pool_info() -> Result<()> {
    let client = SuiClient::new("https://fullnode.testnet.sui.io:443").await?;
    
    let pool_info = client.get_pool_info("SUI", "USDC").await?;
    assert!(pool_info.liquidity > 0);
    assert!(pool_info.fee_rate > 0.0);
    
    Ok(())
}

#[tokio::test]
async fn test_transaction_signing() -> Result<()> {
    // Test transaction signing without sending
    let client = SuiClient::new_offline()?;
    
    let operation = VaultOperation::Deposit {
        amount: 1_000_000,
        asset: "SUI".to_string(),
    };
    
    let signed_tx = client.sign_vault_operation(operation)?;
    assert!(!signed_tx.signature.is_empty());
    
    Ok(())
}

