# Deployment Guide

## Prerequisites

- Docker & Docker Compose
- Sui CLI (for smart contracts)
- Rust toolchain (for backend/frontend)
- Trunk (for frontend builds)
- PostgreSQL (for production)

## Local Development

### 1. Clone Repository

```bash
git clone https://github.com/your-org/deepbook-lp-platform.git
cd deepbook-lp-platform
```

### 2. Start Services

```bash
docker-compose up -d
```

This starts:
- PostgreSQL database
- Backend API server (port 8080)
- AI Agent runner

### 3. Deploy Smart Contracts

```bash
cd contracts/manus_liquidity
sui client publish --gas-budget 100000000
```

Save the package ID and update `.env`:
```
MANUS_PACKAGE_ID=0x...
```

### 4. Build Frontend

```bash
cd frontend
trunk serve --open
```

Frontend available at http://localhost:8080

## Production Deployment

### Smart Contracts (Sui Mainnet)

1. **Test on Testnet**:
```bash
sui client switch --env testnet
sui client publish --gas-budget 100000000
```

2. **Deploy to Mainnet**:
```bash
sui client switch --env mainnet
sui client publish --gas-budget 100000000
```

3. **Verify Deployment**:
```bash
sui client object <PACKAGE_ID>
```

### Backend (Azure TDX VMs)

1. **Build Docker Image**:
```bash
cd backend
docker build -t manus-backend:latest .
```

2. **Push to Registry**:
```bash
docker tag manus-backend:latest your-registry/manus-backend:latest
docker push your-registry/manus-backend:latest
```

3. **Deploy to Kubernetes**:
```bash
kubectl apply -f k8s/deployment.yml
kubectl apply -f k8s/service.yml
```

4. **Verify**:
```bash
kubectl get pods
kubectl logs -f deployment/manus-backend
```

### Frontend (Cloudflare Pages)

1. **Build Production**:
```bash
cd frontend
trunk build --release
```

2. **Deploy**:
```bash
wrangler pages publish dist/
```

Or use GitHub Actions (automatic on push to main).

## Configuration

### Environment Variables

**Backend** (`.env`):
```bash
# Database
MANUS_DATABASE_URL=postgres://user:pass@localhost/manus_liquidity

# Sui Network
MANUS_SUI_NETWORK_URL=https://fullnode.mainnet.sui.io:443
MANUS_PACKAGE_ID=0x...

# Server
MANUS_SERVER_HOST=0.0.0.0
MANUS_SERVER_PORT=8080

# Security
MANUS_SECURITY_PQC_ENABLED=true
MANUS_SECURITY_ZK_PROOFS_ENABLED=true

# Agents
MANUS_AGENTS_ENABLED=true
MANUS_AGENTS_REBALANCE_INTERVAL=300
MANUS_AGENTS_RISK_TOLERANCE=0.5

# Monitoring
RUST_LOG=info
```

### Database Migration

```bash
cd backend
sqlx database create
sqlx migrate run
```

## Monitoring

### Prometheus

Access metrics at: http://localhost:9090

**Key Metrics**:
- `manus_deposits_total` - Total deposits
- `manus_withdrawals_total` - Total withdrawals
- `manus_tvl_usd` - Total value locked
- `manus_agent_decisions_total` - AI decisions

### Grafana

Dashboard at: http://localhost:3000

**Dashboards**:
- Platform Overview
- Vault Performance
- AI Agent Activity
- System Health

## Troubleshooting

### Smart Contract Issues

**Problem**: Transaction fails
```bash
# Check gas budget
sui client gas

# Increase gas budget
sui client publish --gas-budget 200000000
```

### Backend Issues

**Problem**: Database connection fails
```bash
# Check PostgreSQL is running
docker ps | grep postgres

# Check connection string
echo $MANUS_DATABASE_URL
```

**Problem**: Sui RPC errors
```bash
# Test connection
curl https://fullnode.mainnet.sui.io:443

# Switch to backup RPC
export MANUS_SUI_NETWORK_URL=https://sui-mainnet.nodeinfra.com
```

### Frontend Issues

**Problem**: WASM fails to load
```bash
# Clear cache
trunk clean

# Rebuild
trunk build --release
```

## Security Checklist

- [ ] Smart contracts audited
- [ ] PQC keys generated securely
- [ ] TDX attestation verified
- [ ] Database encrypted at rest
- [ ] TLS certificates valid
- [ ] API rate limiting enabled
- [ ] Monitoring alerts configured
- [ ] Backup strategy tested

## Performance Tuning

### Database

```sql
-- Index optimization
CREATE INDEX idx_vaults_strategy ON vaults(strategy);
CREATE INDEX idx_deposits_timestamp ON deposits(timestamp);

-- Connection pooling
ALTER SYSTEM SET max_connections = 200;
```

### Backend

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### Frontend

```bash
# Optimize WASM
wasm-opt -Oz -o optimized.wasm input.wasm

# Compress assets
gzip -9 dist/*.wasm
```

## Scaling

### Horizontal Scaling

```yaml
# k8s/deployment.yml
spec:
  replicas: 5  # Increase replicas
  
  # Auto-scaling
  autoscaling:
    minReplicas: 3
    maxReplicas: 10
    targetCPUUtilizationPercentage: 70
```

### Database Scaling

```bash
# Add read replicas
kubectl apply -f k8s/postgres-replica.yml

# Configure connection pooling
export MANUS_DATABASE_MAX_CONNECTIONS=100
```

## Backup & Recovery

### Database Backup

```bash
# Automated daily backups
0 2 * * * pg_dump manus_liquidity | gzip > backup-$(date +\%Y\%m\%d).sql.gz
```

### Smart Contract State

```bash
# Export on-chain state
sui client object <VAULT_ID> > vault-state.json
```

### Disaster Recovery

1. Restore database from backup
2. Redeploy smart contracts (if needed)
3. Restart backend services
4. Verify state consistency

## Maintenance

### Updates

```bash
# Update dependencies
cargo update
npm update

# Rebuild
docker-compose build
docker-compose up -d
```

### Health Checks

```bash
# Backend health
curl http://localhost:8080/health

# Database health
psql -c "SELECT 1"

# Smart contract health
sui client object <PACKAGE_ID>
```

## Support

For issues:
- GitHub Issues: https://github.com/your-org/deepbook-lp-platform/issues
- Discord: https://discord.gg/manus-ai
- Email: support@manus-ai.io

