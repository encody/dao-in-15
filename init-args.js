const ONE_DAY = 1_000_000_000 * 60 * 60 * 24;

// 1e24, calculated like this because JS numbers don't work that large
const ONE_NEAR = BigInt(1e12) ** 2n;

const nullAccountId = '0'.repeat(64);

console.log(JSON.stringify({
  // owner_id: nullAccountId, // replace with your account ID
  owner_id: 'hatchet.testnet', 
}));
