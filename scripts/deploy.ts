import { execSync } from 'child_process';
import * as path from 'path';

// Load environment variables (e.g. RPC URL, Network Passphrase, Admin Secret)
require('dotenv').config();

const network = process.env.NETWORK || 'testnet';
const sourceAccount = process.env.SOURCE_SECRET || 'YOUR_SECRET_KEY';
const contractDir = path.join(__dirname, '../contracts/vault-contract');
const wasmPath = path.join(contractDir, 'target/wasm32-unknown-unknown/release/validity_vault.wasm');

console.log('Compiling the Vault contract...');
try {
  execSync('cargo build --target wasm32-unknown-unknown --release', { cwd: contractDir, stdio: 'inherit' });
} catch (error: unknown) {
  console.error('Failed to compile contract:', error instanceof Error ? error.message : error);
  process.exit(1);
}

console.log(`Deploying Vault contract to ${network}...`);
// Example Soroban CLI command (this requires `soroban-cli` to be installed globally)
// In a real environment, you would use @stellar/stellar-sdk or CLI bindings.
const deployCmd = `soroban contract deploy --wasm ${wasmPath} --source ${sourceAccount} --network ${network}`;

try {
  const result = execSync(deployCmd, { encoding: 'utf-8' });
  const contractId = result.trim();
  console.log(`✅ Successfully deployed Vault Contract.`);
  console.log(`Contract ID: ${contractId}`);
  
  // TODO: Save contract ID to a config file for frontend/backend to consume
} catch (error: unknown) {
  console.error(
    'Deployment failed. Ensure soroban-cli is installed and configured:',
    error instanceof Error ? error.message : error
  );
}
