import { PublicKey } from '@solana/web3.js';
import {
  initializeShagaAccounts,
  createLender,
  createAffair,
  startRental,
  endRental,
  terminateAffair,
} from './custom';
import shagaAuthorityRaw from '../../test_keypairs/0.json';
import shagaFeePayerRaw from '../../test_keypairs/1.json';
import shagaLenderRaw from '../../test_keypairs/3.json';
import shagaClientRaw from '../../test_keypairs/1.json';
import shagaLenderTwoRaw from '../../test_keypairs/6.json';
import shagaClientTwoRaw from '../../test_keypairs/5.json';
import {
  Connection,
  LAMPORTS_PER_SOL,
  Keypair,
  TransactionInstruction
} from '@solana/web3.js';
import { signAndSendLegacyTransaction, stringToUint8Array } from './utils';
import * as dotenv from "dotenv";
import BN from 'bn.js'

import { Affair, AffairPayload, AffairsList, HashAlgorithm } from './generated';
import { hashValueSha256 } from './hash';
import { HASH_PREFIX } from './constants';
dotenv.config();


const shagaAuthority = Keypair.fromSecretKey(Uint8Array.from(shagaAuthorityRaw));
const shagaFeePayer = Keypair.fromSecretKey(Uint8Array.from(shagaFeePayerRaw));
const shagaLender = Keypair.fromSecretKey(Uint8Array.from(shagaLenderRaw));
const shagaClient = Keypair.fromSecretKey(Uint8Array.from(shagaClientRaw));
const shagaLenderTwo = Keypair.fromSecretKey(Uint8Array.from(shagaLenderTwoRaw));
const shagaClientTwo = Keypair.fromSecretKey(Uint8Array.from(shagaClientTwoRaw));
const connection = new Connection(process.env.RPC_URL || "", "confirmed");

async function main() {
  let instructions: TransactionInstruction[] = [];

  // instructions.push(initializeShagaAccounts(shagaAuthority.publicKey));
  // instructions.push(createLender(shagaLender.publicKey));

  // // Generate some dummy data
  // the shaga coordinates system. built around online maps coordinates system.
  // precision does not have to be accurate to the dot that is why only three decimal points are used.
  // values can be negative. with the format of: ±DD.DDD,±DDD.DDD (lat,long)
  const dummyCoordinates = 'y4xofD32dWBVQJcjJ5Xy3KKW3Q95ofyfdWhoRN.y4xofD32dWBVQJcjJ5Xy3KKW3Q95ofyfdWhoRN,y4xofD32dWBVQJcjJ5Xy3KKW3Q95ofyfdWhoRN.y4xofD32dWBVQJcjJ5Xy3KKW3Q95ofyfdWhoRN';
  const dummyIpAddress = '192.168.1.1';
  const dummyCpuName = 'Intel Core i7-9700K';
  const dummyGpuName = 'NVIDIA GeForce RTX 3070';

  const currentTimeInSeconds = Math.floor(new Date().getTime() / 1000);
  // // console.log(currentTimeInSeconds);
  // // // Add 1 hour to the current time
  const terminationTimeInSeconds = currentTimeInSeconds + 3000;
  // // // Exported dummy data
  const affairPayload: AffairPayload = {
    coordinates: dummyCoordinates,
    ipAddress: dummyIpAddress,
    cpuName: dummyCpuName,
    gpuName: dummyGpuName,
    totalRamMb: 16384, // Assuming 16GB RAM for this dummy data
    solPerHour: 2 * LAMPORTS_PER_SOL, // Assuming a dummy value of 1 SOL per HOUR
    affairTerminationTime: new BN(terminationTimeInSeconds), // Assuming a dummy timestamp value
    hashAlgorithm: HashAlgorithm.Sha256,
    // please use at least 16 characters below to create a password. 
    // the hashValue must include a HASH_PREFIX. please check the function when creating a hash.
    privatePairHash: await hashValueSha256("y4xofD32dWBVQJcjJ5Xy3KKW3Q95ofyfdWhoRN")
  };
  // instructions.push(createAffair(shagaLender.publicKey, affairPayload));

  // const affairKey = new PublicKey("CV27uCUdN8wNkFsAVxEy4Lv6P695725UySTMreRX6tWW")
  const affairKey = new PublicKey("3mU9V6jpJbePx25D3sZ6KwHJDKHXMpyfioFuwz6tc96X")
  // // // constant
  // const affairsListKey = new PublicKey("FN3cFWBo9i5NANaHdPNw6cF789qZPP5poFRum7rCA8Wt")
  // const affairList = await AffairsList.fromAccountAddress(connection, affairsListKey)
  // console.log(affairList.pretty())
  // // await hashValueSha256("y4xofD32dWBVQJcjJ5Xy3KKW3Q95ofyfdWhoRN")
  // const getAffair = await Affair.fromAccountAddress(connection, affairKey) // affairList.activeAffairs[0])
  // console.log(getAffair.pretty())

  // const currentTimeInSeconds = Math.floor(new Date().getTime() / 1000);
  // console.log(currentTimeInSeconds);
  // // Add 1 hour to the current time
  // const terminationTimeInSeconds = currentTimeInSeconds + 1800;
  // instructions.push(await startRental(connection, shagaClientTwo.publicKey, affairKey, terminationTimeInSeconds, "y4xofD32dWBVQJcjJ5Xy3KKW3Q95ofyfdWhoRN"));
  // console.log(shagaClientTwo.publicKey.toBase58())
  // instructions.push(await endRental(shagaClientTwo.publicKey, affairKey))

  instructions.push(await terminateAffair(connection, shagaLender.publicKey, affairKey, false))

  await signAndSendLegacyTransaction(connection,
    [shagaLender],
    shagaLender,
    instructions
  );

  // devnet feature for debugging. not used anymore.
  // close all affairs
  // for (let i = 0; i < affairList.activeAffairs.length; i++) {
  //   let instructions: TransactionInstruction[] = [];

  //   const affairKey = affairList.activeAffairs[i];
  //   const instruction = await terminateAffair(connection, shagaLenderTwo.publicKey, affairKey, true);
  //   instructions.push(instruction);

  //   await signAndSendLegacyTransaction(connection,
  //     [shagaLenderTwo],
  //     // [shagaClient],
  //     shagaLenderTwo,
  //     // shagaClient,
  //     instructions
  //   );
  // }

}

main()