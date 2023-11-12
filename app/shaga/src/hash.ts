import { blake3 } from '@noble/hashes/blake3';
import { sha256 } from '@noble/hashes/sha256';
import { bytesToHex as toHex } from '@noble/hashes/utils';
import { HASH_PREFIX } from './constants';

// Function to hash a value using BLAKE3 (NOT IN MAINNET YET)
async function hashValue(value: string): Promise<any> {
  return blake3(HASH_PREFIX + value);
}
// Function to check if a value matches a hashed value (NOT IN MAINNET YET)
async function verifyValue(value: string, hashedValue: string): Promise<boolean> {
  const valueHash = await hashValue(value);
  return valueHash === hashedValue;
}

// Function to hash a value using SHA-256
export async function hashValueSha256(value: string): Promise<any> {
  return sha256(HASH_PREFIX + value);
}

// Function to check if a value matches a hashed value
export async function verifyValueSha256(value: string, hashedValue: string): Promise<boolean> {
  const valueHash = await hashValueSha256(value);
  return valueHash === hashedValue;
}


// async function runMe() {

//   // Example usage:
//   const value = '123';
//   console.time("test hashValue")
//   const hashed = await hashValueSha256(value);
//   console.timeEnd("test hashValue")
//   console.log(`Hashed value is: ${hashed}`);

//   console.time("test verifyValue")
//   const isValid = await verifyValueSha256(value, hashed);
//   console.timeEnd("test verifyValue")
//   console.log(`Is the value valid? ${isValid}`);

// }

// let listOfSupportedHashes = crypto.getHashes();
// console.log('Total supported hashes : ', listOfSupportedHashes);
// runMe()