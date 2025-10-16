#!/bin/bash

# Define paths
CONTRACT_DIR="$(dirname "$0")"
SOURCES_DIR="$CONTRACT_DIR/sources"

VAULT_MOVE="$SOURCES_DIR/vault.move"
VAULT_MATH_MOVE="$SOURCES_DIR/vault_math.move"
VAULT_MATH_VERIFY="$SOURCES_DIR/vault_math.move.verify"

# Create temporary files for original content
TMP_VAULT_MOVE=$(mktemp)
TMP_VAULT_MATH_MOVE=$(mktemp)
TMP_VAULT_MATH_VERIFY=$(mktemp)

# Save original content
cp "$VAULT_MOVE" "$TMP_VAULT_MOVE"
cp "$VAULT_MATH_MOVE" "$TMP_VAULT_MATH_MOVE"
cp "$VAULT_MATH_VERIFY" "$TMP_VAULT_MATH_VERIFY"

# Copy the verification file content into the main Move file for sui-prover
# This is a common workaround for sui-prover to verify functions and their specs in the same file
cat "$VAULT_MATH_VERIFY" >> "$VAULT_MATH_MOVE"

# Run sui-prover
echo "Running sui-prover for deepbook_lp_vaults..."
sui move prove "$CONTRACT_DIR"

# Restore original files
mv "$TMP_VAULT_MOVE" "$VAULT_MOVE"
mv "$TMP_VAULT_MATH_MOVE" "$VAULT_MATH_MOVE"
mv "$TMP_VAULT_MATH_VERIFY" "$VAULT_MATH_VERIFY"

echo "Verification complete. Original files restored."

