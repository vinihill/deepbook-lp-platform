# DeepBook Liquidity Provisioning Platform: Deployment Guide

**Version:** 1.0.0  
**Date:** September 2025  
**Author:** Manus AI  

## Table of Contents

1. [Quick Start Guide](#quick-start-guide)
2. [Prerequisites](#prerequisites)
3. [Environment Setup](#environment-setup)
4. [Smart Contract Deployment](#smart-contract-deployment)
5. [Frontend Deployment](#frontend-deployment)
6. [Testing and Validation](#testing-and-validation)
7. [Production Deployment](#production-deployment)
8. [Monitoring and Maintenance](#monitoring-and-maintenance)
9. [Troubleshooting](#troubleshooting)

---

## Quick Start Guide

This quick start guide provides the essential steps to deploy the DeepBook Liquidity Provisioning Platform in a development environment. For production deployments, please refer to the detailed sections below.

### 1. Install Prerequisites

```bash
# Install Sui CLI
curl -sSfL https://raw.githubusercontent.com/MystenLabs/suiup/main/install.sh | sh
export PATH="$HOME/.local/bin:$PATH"
suiup install sui@devnet -y

# Install Node.js (if not already installed)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installations
sui --version
node --version
npm --version
```

### 2. Clone and Setup Project

```bash
# Clone the project repository
git clone <repository-url>
cd deepbook-liquidity-provisioning

# Install dependencies
cd deepbook-lp-frontend
npm install
cd ..

cd integration_tests
npm install
cd ..
```

### 3. Deploy Smart Contracts

```bash
# Deploy contracts in order
cd contracts

# Deploy vault contracts
cd deepbook_lp_vaults
sui move build
VAULT_PACKAGE_ID=$(sui client publish --gas-budget 100000000 --json | jq -r '.objectChanges[] | select(.type == "published") | .packageId')
echo "Vault Package ID: $VAULT_PACKAGE_ID"
cd ..

# Deploy other contracts (strategy, risk control, accounting, registry)
# ... (repeat for each contract)
```

### 4. Configure and Start Frontend

```bash
cd deepbook-lp-frontend

# Create environment configuration
cat > .env.local << EOF
VITE_SUI_NETWORK=devnet
VITE_SUI_RPC_URL=https://fullnode.devnet.sui.io:443
VITE_VAULT_PACKAGE_ID=$VAULT_PACKAGE_ID
# ... (add other package IDs)
EOF

# Start development server
npm run dev -- --host
```

### 5. Run Tests

```bash
cd integration_tests
npm run test:all
```

---

## Prerequisites

### System Requirements

- **Operating System:** Linux (Ubuntu 20.04+), macOS (10.15+), or Windows 10+ with WSL2
- **Memory:** Minimum 8GB RAM, recommended 16GB+
- **Storage:** Minimum 20GB free space
- **Network:** Stable internet connection for blockchain interactions

### Required Software

#### Sui CLI and Tools

The Sui CLI is essential for smart contract compilation and deployment. Install using the official suiup installer:

```bash
# Install suiup
curl -sSfL https://raw.githubusercontent.com/MystenLabs/suiup/main/install.sh | sh

# Add to PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Install Sui CLI for devnet
suiup install sui@devnet -y

# Verify installation
sui --version
```

#### Node.js and npm

Node.js version 18 or higher is required for the frontend application and testing framework:

```bash
# Install Node.js 18.x
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installation
node --version  # Should be v18.x.x or higher
npm --version   # Should be 8.x.x or higher
```

#### Development Tools

Additional tools for development and deployment:

```bash
# Install Git
sudo apt-get install git

# Install jq for JSON parsing
sudo apt-get install jq

# Install curl and wget
sudo apt-get install curl wget

# Install build essentials
sudo apt-get install build-essential
```

### Wallet Setup

#### Create Sui Wallet

```bash
# Create a new wallet
sui client new-address ed25519

# Request test tokens from faucet
sui client faucet

# Check balance
sui client balance
```

#### Configure Network

```bash
# Add devnet configuration
sui client new-env --alias devnet --rpc https://fullnode.devnet.sui.io:443

# Switch to devnet
sui client switch --env devnet

# Verify configuration
sui client envs
```

---

## Environment Setup

### Development Environment

Create a comprehensive development environment that supports all project components:

```bash
# Create project directory structure
mkdir -p deepbook-lp-platform/{contracts,frontend,tests,docs,scripts}
cd deepbook-lp-platform

# Initialize Git repository
git init
git remote add origin <your-repository-url>
```

### Environment Variables

Create environment configuration files for different deployment targets:

#### Development Environment (.env.development)

```bash
cat > .env.development << EOF
# Network Configuration
SUI_NETWORK=devnet
SUI_RPC_URL=https://fullnode.devnet.sui.io:443
SUI_FAUCET_URL=https://faucet.devnet.sui.io/gas

# Contract Addresses (to be filled after deployment)
VAULT_PACKAGE_ID=
STRATEGY_PACKAGE_ID=
RISK_CONTROL_PACKAGE_ID=
ACCOUNTING_PACKAGE_ID=
REGISTRY_PACKAGE_ID=

# Frontend Configuration
FRONTEND_PORT=5173
FRONTEND_HOST=0.0.0.0

# Testing Configuration
TEST_TIMEOUT=30000
TEST_PARALLEL=false
EOF
```

#### Production Environment (.env.production)

```bash
cat > .env.production << EOF
# Network Configuration
SUI_NETWORK=mainnet
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443

# Contract Addresses (production values)
VAULT_PACKAGE_ID=
STRATEGY_PACKAGE_ID=
RISK_CONTROL_PACKAGE_ID=
ACCOUNTING_PACKAGE_ID=
REGISTRY_PACKAGE_ID=

# Frontend Configuration
FRONTEND_URL=https://your-production-domain.com

# Monitoring Configuration
MONITORING_ENABLED=true
ALERT_WEBHOOK_URL=https://your-monitoring-service.com/webhook
EOF
```

### Directory Structure

Organize the project with a clear directory structure:

```
deepbook-lp-platform/
├── contracts/
│   ├── deepbook_lp_vaults/
│   ├── deepbook_lp_strategies/
│   ├── deepbook_lp_risk_controls/
│   ├── deepbook_lp_accounting/
│   └── deepbook_lp_registry/
├── frontend/
│   ├── src/
│   ├── public/
│   ├── package.json
│   └── vite.config.js
├── tests/
│   ├── unit/
│   ├── integration/
│   └── e2e/
├── scripts/
│   ├── deploy.sh
│   ├── test.sh
│   └── monitor.sh
├── docs/
│   ├── api/
│   ├── user-guide/
│   └── technical/
└── README.md
```

---

## Smart Contract Deployment

### Compilation and Validation

Before deployment, compile and validate all smart contracts:

```bash
# Navigate to contracts directory
cd contracts

# Compile all contracts
for contract in deepbook_lp_*; do
    echo "Compiling $contract..."
    cd $contract
    sui move build
    if [ $? -eq 0 ]; then
        echo "✅ $contract compiled successfully"
    else
        echo "❌ $contract compilation failed"
        exit 1
    fi
    cd ..
done
```

### Deployment Script

Create an automated deployment script:

```bash
cat > scripts/deploy_contracts.sh << 'EOF'
#!/bin/bash

set -e

# Load environment variables
source .env.development

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting smart contract deployment...${NC}"

# Check Sui CLI is available
if ! command -v sui &> /dev/null; then
    echo -e "${RED}Sui CLI not found. Please install it first.${NC}"
    exit 1
fi

# Check wallet balance
BALANCE=$(sui client balance --json | jq -r '.totalBalance')
if [ "$BALANCE" -lt 1000000000 ]; then
    echo -e "${RED}Insufficient balance. Please request tokens from faucet.${NC}"
    exit 1
fi

cd contracts

# Deploy contracts in dependency order
declare -A PACKAGE_IDS

# 1. Deploy Vault Contract
echo -e "${YELLOW}Deploying Vault Contract...${NC}"
cd deepbook_lp_vaults
VAULT_RESULT=$(sui client publish --gas-budget 100000000 --json)
VAULT_PACKAGE_ID=$(echo $VAULT_RESULT | jq -r '.objectChanges[] | select(.type == "published") | .packageId')
PACKAGE_IDS["VAULT"]=$VAULT_PACKAGE_ID
echo -e "${GREEN}✅ Vault Contract deployed: $VAULT_PACKAGE_ID${NC}"
cd ..

# 2. Deploy Strategy Contract
echo -e "${YELLOW}Deploying Strategy Contract...${NC}"
cd deepbook_lp_strategies
STRATEGY_RESULT=$(sui client publish --gas-budget 100000000 --json)
STRATEGY_PACKAGE_ID=$(echo $STRATEGY_RESULT | jq -r '.objectChanges[] | select(.type == "published") | .packageId')
PACKAGE_IDS["STRATEGY"]=$STRATEGY_PACKAGE_ID
echo -e "${GREEN}✅ Strategy Contract deployed: $STRATEGY_PACKAGE_ID${NC}"
cd ..

# 3. Deploy Risk Control Contract
echo -e "${YELLOW}Deploying Risk Control Contract...${NC}"
cd deepbook_lp_risk_controls
RISK_RESULT=$(sui client publish --gas-budget 100000000 --json)
RISK_PACKAGE_ID=$(echo $RISK_RESULT | jq -r '.objectChanges[] | select(.type == "published") | .packageId')
PACKAGE_IDS["RISK_CONTROL"]=$RISK_PACKAGE_ID
echo -e "${GREEN}✅ Risk Control Contract deployed: $RISK_PACKAGE_ID${NC}"
cd ..

# 4. Deploy Accounting Contract
echo -e "${YELLOW}Deploying Accounting Contract...${NC}"
cd deepbook_lp_accounting
ACCOUNTING_RESULT=$(sui client publish --gas-budget 100000000 --json)
ACCOUNTING_PACKAGE_ID=$(echo $ACCOUNTING_RESULT | jq -r '.objectChanges[] | select(.type == "published") | .packageId')
PACKAGE_IDS["ACCOUNTING"]=$ACCOUNTING_PACKAGE_ID
echo -e "${GREEN}✅ Accounting Contract deployed: $ACCOUNTING_PACKAGE_ID${NC}"
cd ..

# 5. Deploy Registry Contract
echo -e "${YELLOW}Deploying Registry Contract...${NC}"
cd deepbook_lp_registry
REGISTRY_RESULT=$(sui client publish --gas-budget 100000000 --json)
REGISTRY_PACKAGE_ID=$(echo $REGISTRY_RESULT | jq -r '.objectChanges[] | select(.type == "published") | .packageId')
PACKAGE_IDS["REGISTRY"]=$REGISTRY_PACKAGE_ID
echo -e "${GREEN}✅ Registry Contract deployed: $REGISTRY_PACKAGE_ID${NC}"
cd ..

# Save deployment configuration
cd ..
cat > deployment_config.json << EOL
{
  "network": "$SUI_NETWORK",
  "deployment_time": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "contracts": {
    "vault": "$VAULT_PACKAGE_ID",
    "strategy": "$STRATEGY_PACKAGE_ID",
    "risk_control": "$RISK_PACKAGE_ID",
    "accounting": "$ACCOUNTING_PACKAGE_ID",
    "registry": "$REGISTRY_PACKAGE_ID"
  }
}
EOL

echo -e "${GREEN}✅ All contracts deployed successfully!${NC}"
echo -e "${YELLOW}Deployment configuration saved to deployment_config.json${NC}"

# Update environment file
sed -i "s/VAULT_PACKAGE_ID=.*/VAULT_PACKAGE_ID=$VAULT_PACKAGE_ID/" .env.development
sed -i "s/STRATEGY_PACKAGE_ID=.*/STRATEGY_PACKAGE_ID=$STRATEGY_PACKAGE_ID/" .env.development
sed -i "s/RISK_CONTROL_PACKAGE_ID=.*/RISK_CONTROL_PACKAGE_ID=$RISK_PACKAGE_ID/" .env.development
sed -i "s/ACCOUNTING_PACKAGE_ID=.*/ACCOUNTING_PACKAGE_ID=$ACCOUNTING_PACKAGE_ID/" .env.development
sed -i "s/REGISTRY_PACKAGE_ID=.*/REGISTRY_PACKAGE_ID=$REGISTRY_PACKAGE_ID/" .env.development

echo -e "${GREEN}✅ Environment file updated with contract addresses${NC}"
EOF

chmod +x scripts/deploy_contracts.sh
```

### Execute Deployment

```bash
# Run the deployment script
./scripts/deploy_contracts.sh
```

### Verify Deployment

```bash
# Verify contracts are deployed and accessible
sui client object <VAULT_PACKAGE_ID>
sui client object <STRATEGY_PACKAGE_ID>
# ... verify other contracts
```

---

## Frontend Deployment

### Development Deployment

```bash
cd deepbook-lp-frontend

# Install dependencies
npm install

# Create development configuration
cat > .env.local << EOF
VITE_SUI_NETWORK=devnet
VITE_SUI_RPC_URL=https://fullnode.devnet.sui.io:443
VITE_VAULT_PACKAGE_ID=$(grep VAULT_PACKAGE_ID ../.env.development | cut -d'=' -f2)
VITE_STRATEGY_PACKAGE_ID=$(grep STRATEGY_PACKAGE_ID ../.env.development | cut -d'=' -f2)
VITE_RISK_CONTROL_PACKAGE_ID=$(grep RISK_CONTROL_PACKAGE_ID ../.env.development | cut -d'=' -f2)
VITE_ACCOUNTING_PACKAGE_ID=$(grep ACCOUNTING_PACKAGE_ID ../.env.development | cut -d'=' -f2)
VITE_REGISTRY_PACKAGE_ID=$(grep REGISTRY_PACKAGE_ID ../.env.development | cut -d'=' -f2)
EOF

# Start development server
npm run dev -- --host
```

### Production Build

```bash
# Create production configuration
cat > .env.production << EOF
VITE_SUI_NETWORK=mainnet
VITE_SUI_RPC_URL=https://fullnode.mainnet.sui.io:443
VITE_VAULT_PACKAGE_ID=<production_vault_package_id>
VITE_STRATEGY_PACKAGE_ID=<production_strategy_package_id>
VITE_RISK_CONTROL_PACKAGE_ID=<production_risk_control_package_id>
VITE_ACCOUNTING_PACKAGE_ID=<production_accounting_package_id>
VITE_REGISTRY_PACKAGE_ID=<production_registry_package_id>
EOF

# Build for production
npm run build

# Test production build locally
npm run preview
```

### Static Deployment

Deploy the built application to a static hosting service:

```bash
# Deploy to Vercel
npx vercel --prod

# Deploy to Netlify
npx netlify deploy --prod --dir=dist

# Deploy to AWS S3
aws s3 sync dist/ s3://your-bucket-name --delete

# Deploy using the service deployment tool
cd ..
# Use the service_deploy_frontend tool if available
```

---

## Testing and Validation

### Unit Tests

```bash
cd integration_tests

# Run smart contract tests
npm run test

# Run frontend tests
npm run test:frontend

# Run API tests
npm run test:api
```

### Integration Tests

```bash
# Run full integration test suite
npm run test:all

# Run tests with coverage
npm run test:coverage

# Run tests in watch mode for development
npm run test:watch
```

### End-to-End Testing

```bash
# Install E2E testing dependencies
npm install --save-dev playwright

# Run E2E tests
npx playwright test

# Run E2E tests with UI
npx playwright test --ui
```

### Performance Testing

```bash
# Install performance testing tools
npm install --save-dev lighthouse

# Run performance audit
npx lighthouse http://localhost:5173 --output=html --output-path=./performance-report.html

# Run load testing
npx artillery run load-test-config.yml
```

---

## Production Deployment

### Pre-Production Checklist

- [ ] All tests passing
- [ ] Security audit completed
- [ ] Performance benchmarks met
- [ ] Documentation updated
- [ ] Monitoring configured
- [ ] Backup procedures tested
- [ ] Rollback plan prepared

### Production Environment Setup

```bash
# Create production environment
cat > .env.production << EOF
# Production configuration
SUI_NETWORK=mainnet
SUI_RPC_URL=https://fullnode.mainnet.sui.io:443

# Production contract addresses
VAULT_PACKAGE_ID=<production_vault_id>
STRATEGY_PACKAGE_ID=<production_strategy_id>
RISK_CONTROL_PACKAGE_ID=<production_risk_control_id>
ACCOUNTING_PACKAGE_ID=<production_accounting_id>
REGISTRY_PACKAGE_ID=<production_registry_id>

# Production settings
NODE_ENV=production
LOG_LEVEL=warn
MONITORING_ENABLED=true
EOF
```

### Deployment Pipeline

Create an automated deployment pipeline:

```bash
cat > scripts/deploy_production.sh << 'EOF'
#!/bin/bash

set -e

echo "🚀 Starting production deployment..."

# 1. Run all tests
echo "📋 Running test suite..."
cd integration_tests
npm run test:all
cd ..

# 2. Build frontend
echo "🏗️ Building frontend..."
cd deepbook-lp-frontend
npm run build
cd ..

# 3. Deploy contracts to mainnet
echo "📦 Deploying contracts to mainnet..."
# Switch to mainnet
sui client switch --env mainnet
./scripts/deploy_contracts.sh

# 4. Deploy frontend
echo "🌐 Deploying frontend..."
cd deepbook-lp-frontend
npm run deploy:production
cd ..

# 5. Run smoke tests
echo "🧪 Running smoke tests..."
npm run test:smoke

echo "✅ Production deployment completed successfully!"
EOF

chmod +x scripts/deploy_production.sh
```

---

## Monitoring and Maintenance

### Health Monitoring

Set up comprehensive monitoring for the deployed system:

```bash
cat > scripts/monitor.sh << 'EOF'
#!/bin/bash

# Monitor contract health
check_contract_health() {
    local package_id=$1
    local contract_name=$2
    
    echo "Checking $contract_name health..."
    
    # Check if contract is accessible
    if sui client object $package_id > /dev/null 2>&1; then
        echo "✅ $contract_name is accessible"
    else
        echo "❌ $contract_name is not accessible"
        # Send alert
        curl -X POST $ALERT_WEBHOOK_URL -d "{\"text\":\"$contract_name is not accessible\"}"
    fi
}

# Monitor frontend health
check_frontend_health() {
    local url=$1
    
    echo "Checking frontend health..."
    
    if curl -f $url > /dev/null 2>&1; then
        echo "✅ Frontend is accessible"
    else
        echo "❌ Frontend is not accessible"
        # Send alert
        curl -X POST $ALERT_WEBHOOK_URL -d "{\"text\":\"Frontend is not accessible\"}"
    fi
}

# Load configuration
source .env.production

# Run health checks
check_contract_health $VAULT_PACKAGE_ID "Vault Contract"
check_contract_health $STRATEGY_PACKAGE_ID "Strategy Contract"
check_contract_health $RISK_CONTROL_PACKAGE_ID "Risk Control Contract"
check_contract_health $ACCOUNTING_PACKAGE_ID "Accounting Contract"
check_contract_health $REGISTRY_PACKAGE_ID "Registry Contract"
check_frontend_health $FRONTEND_URL
EOF

chmod +x scripts/monitor.sh
```

### Automated Monitoring

Set up cron jobs for regular monitoring:

```bash
# Add to crontab
crontab -e

# Add these lines:
# Check system health every 5 minutes
*/5 * * * * /path/to/project/scripts/monitor.sh

# Generate daily reports
0 9 * * * /path/to/project/scripts/daily_report.sh

# Weekly maintenance
0 2 * * 0 /path/to/project/scripts/weekly_maintenance.sh
```

---

## Troubleshooting

### Common Issues

#### Contract Deployment Failures

**Issue:** Contract compilation fails
```bash
# Solution: Check Move.toml dependencies
sui move build --verbose

# Update dependencies if needed
sui move update
```

**Issue:** Insufficient gas for deployment
```bash
# Solution: Request more test tokens or increase gas budget
sui client faucet
sui client publish --gas-budget 200000000
```

#### Frontend Issues

**Issue:** Environment variables not loading
```bash
# Solution: Check .env file format and restart dev server
cat .env.local
npm run dev
```

**Issue:** Wallet connection fails
```bash
# Solution: Check wallet extension and network configuration
# Ensure wallet is connected to correct network
```

#### Network Issues

**Issue:** RPC endpoint not responding
```bash
# Solution: Try alternative RPC endpoints
sui client new-env --alias backup --rpc https://backup-rpc-url.com
sui client switch --env backup
```

### Debug Mode

Enable debug mode for detailed logging:

```bash
# Set debug environment variables
export DEBUG=true
export LOG_LEVEL=debug

# Run with verbose output
npm run dev -- --debug
```

### Support Resources

- **Documentation:** [Project Documentation](./Technical_Documentation.md)
- **Community:** [Discord/Telegram Channel]
- **Issues:** [GitHub Issues](https://github.com/your-repo/issues)
- **Support:** [Support Email](mailto:support@yourproject.com)

---

*This deployment guide is maintained by the development team. For the latest updates and additional resources, please refer to the project repository.*

