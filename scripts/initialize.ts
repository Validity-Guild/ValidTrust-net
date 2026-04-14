import { execSync } from 'child_process';

require('dotenv').config();

const network = process.env.NETWORK || 'testnet';
const sourceAccount = process.env.SOURCE_SECRET || 'YOUR_SECRET_KEY';
const contractId = process.env.VAULT_CONTRACT_ID;
const tokenAddress = process.env.TOKEN_CONTRACT_ID;
const adminAddress = process.env.ADMIN_ADDRESS;

if (!contractId || !tokenAddress || !adminAddress) {
  console.error('Missing required environment variables (VAULT_CONTRACT_ID, TOKEN_CONTRACT_ID, ADMIN_ADDRESS)');
  process.exit(1);
}

console.log(`Initializing Vault contract ${contractId}...`);

// Example Soroban CLI command to invoke the initialize function
const invokeCmd = `soroban contract invoke --id ${contractId} --source ${sourceAccount} --network ${network} -- initialize --admin ${adminAddress} --token ${tokenAddress}`;

try {
  const result = execSync(invokeCmd, { encoding: 'utf-8' });
  console.log(`✅ Successfully initialized Vault Contract.`);
  console.log(result);
} catch (error) {
  console.error('Initialization failed:', error);
}
