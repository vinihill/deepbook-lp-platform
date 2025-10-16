#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Running formal verification for manus_liquidity contracts..."

# Path to the sui-prover executable (assuming it's built in release mode)
SUI_PROVER_PATH="/home/ubuntu/sui-prover/target/release/sui-prover"

# Check if sui-prover exists
if [ ! -f "$SUI_PROVER_PATH" ]; then
    echo "Error: sui-prover not found at $SUI_PROVER_PATH. Please ensure it is built."
    exit 1
fi

# Navigate to the package directory
cd /home/ubuntu/deepbook-lp-platform/contracts/manus_liquidity

# Run sui-prover for the current package
echo "\n--- Verifying manus_liquidity package ---"
"$SUI_PROVER_PATH" --path .

echo "\nFormal verification completed."

