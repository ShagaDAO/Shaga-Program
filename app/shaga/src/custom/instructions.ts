import { Connection } from '@solana/web3.js';
import { PublicKey } from '@solana/web3.js';
import { Affair, AffairPayload, RentalTerminationAuthority, ShagaState, createCreateAffairInstruction, createEndRentalInstruction, createInitializeInstruction, createInitializeLenderInstruction, createStartRentalInstruction, createTerminateAffairInstruction, createTerminateVacantAffairInstruction, createUpdateShagaStateInstruction } from '../generated';
import {
  findAffairList,
  findShagaState,
  findThreadAuthority,
  findAffairThreadId,
  findRentalThreadId,
  findClockworkThreadAccount,
  findRentAccount,
  findRentEscrow,
  findLender,
  findAffair,
} from '../pda';
import { CLOCKWORK_PROGRAM_ID } from '../constants';

export function initializeShagaAccounts(payer: PublicKey) {

  const [affairsList] = findAffairList();
  const [shagaState] = findShagaState();
  const [threadAuthority] = findThreadAuthority();

  const initializeAccountsIx = createInitializeInstruction(
    {
      payer: payer,
      affairsList,
      shagaState,
      threadAuthority,
    },
    {
      shagaAuthority: new PublicKey("BNw1V3jc8cGmMfdbgGMzpworE46MQKEfsCQ4QrzQm7G6"),
      feeDestination: new PublicKey("BNw1V3jc8cGmMfdbgGMzpworE46MQKEfsCQ4QrzQm7G6"),
      feeBasisPoints: 100,
      isPaused: false
    }
  )
  return initializeAccountsIx
}

export function createLender(payer: PublicKey) {

  const [lender] = findLender(payer);

  const createLenderIx = createInitializeLenderInstruction(
    {
      payer: payer,
      lender,
    }
  )
  return createLenderIx
}

export function createAffair(authority: PublicKey, affairPayload: AffairPayload) {
  const [affair] = findAffair(authority);
  console.log('affair', affair.toBase58())
  const [lender] = findLender(authority);
  const [affairsList] = findAffairList();
  const [shagaState] = findShagaState();
  const [threadAuthority] = findThreadAuthority();
  const [threadId] = findAffairThreadId(threadAuthority, affair);
  const [affairClockworkThread] = findClockworkThreadAccount(threadAuthority, threadId);

  const createAffairIx = createCreateAffairInstruction(
    {
      authority,
      lender,
      affair,
      affairsList,
      shagaState,
      threadAuthority,
      affairClockworkThread,
      clockworkProgram: CLOCKWORK_PROGRAM_ID
    },
    {
      payload: affairPayload
    }
  )
  return createAffairIx

}

export async function startRental(connection: Connection,
  client: PublicKey,
  affair: PublicKey,
  rentalTerminationTime: number,
  privatePairHashCode?: string) {
  const affairData = await Affair.fromAccountAddress(connection, affair);
  const [lender] = findLender(affairData.authority);
  const [affairsList] = findAffairList();
  const [shagaState] = findShagaState();
  const shagaStateData = await ShagaState.fromAccountAddress(connection, shagaState);
  const [threadAuthority] = findThreadAuthority();
  const [escrow] = findRentEscrow(lender, client);
  const [rental] = findRentAccount(lender, client);
  const [threadId] = findRentalThreadId(threadAuthority, rental);
  const [rentalClockworkThread] = findClockworkThreadAccount(threadAuthority, threadId);
  const startRentalIx = createStartRentalInstruction(
    {
      client,
      lender,
      affair,
      affairsList,
      shagaState,
      feeDestination: shagaStateData.feeDestination,
      escrow,
      rental,
      threadAuthority,
      rentalClockworkThread,
      clockworkProgram: CLOCKWORK_PROGRAM_ID
    },
    { // time in UTC timestamp since epoch
      rentalTerminationTime,
      // this code is no does not include the HASH_PREFIX. only the 16+ character code.
      privatePairHashCode
    }
  )
  return startRentalIx
}
export async function endRental(client: PublicKey, affair: PublicKey) {
  const connection = new Connection(process.env.RPC_URL || "");
  const affairData = await Affair.fromAccountAddress(connection, affair);
  const [lender] = findLender(affairData.authority);
  const [affairsList] = findAffairList();
  const [shagaState] = findShagaState();
  const [threadAuthority] = findThreadAuthority();
  const [escrow] = findRentEscrow(lender, client);
  const [rental] = findRentAccount(lender, client);
  const [threadId] = findRentalThreadId(threadAuthority, rental);
  const [rentalClockworkThread] = findClockworkThreadAccount(threadAuthority, threadId);

  const endRentalIx = createEndRentalInstruction(
    {
      signer: client,
      client,
      lender,
      affair,
      affairsList,
      shagaState,
      escrow,
      rental,
      threadAuthority,
      rentalClockworkThread,
      clockworkProgram: CLOCKWORK_PROGRAM_ID
    },
    {
      terminationBy: RentalTerminationAuthority.Client
    }
  )
  return endRentalIx

}

export async function terminateAffair(connection: Connection, authority: PublicKey, affair: PublicKey, vacant?: boolean) {
  let [lender] = findLender(authority);
  const [affairsList] = findAffairList();
  const [shagaState] = findShagaState();
  const [threadAuthority] = findThreadAuthority();
  const [threadId] = findAffairThreadId(threadAuthority, affair);
  const [affairClockworkThread] = findClockworkThreadAccount(threadAuthority, threadId);
  if (vacant) {
    const terminateAffairIx = createTerminateVacantAffairInstruction(
      {
        signer: authority,
        authority,
        lender,
        affair,
        affairsList,
        shagaState,
        threadAuthority,
        affairClockworkThread,
        clockworkProgram: CLOCKWORK_PROGRAM_ID
      }
    )
    return terminateAffairIx
  }

  const affairData = await Affair.fromAccountAddress(connection, affair);
  [lender] = findLender(affairData.authority);
  const [escrow] = findRentEscrow(lender, affairData.client);
  const [rental] = findRentAccount(lender, affairData.client);
  const [threadIdRental] = findRentalThreadId(threadAuthority, rental);
  const [rentalClockworkThread] = findClockworkThreadAccount(threadAuthority, threadIdRental);

  const terminateAffairIx = createTerminateAffairInstruction(
    {
      authority,
      client: affairData.client,
      lender,
      affair,
      affairsList,
      shagaState,
      escrow,
      rental,
      threadAuthority,
      affairClockworkThread,
      rentalClockworkThread,
      clockworkProgram: CLOCKWORK_PROGRAM_ID
    }
  )
  return terminateAffairIx
}

type UpdateShagaStateType = {
  payer: PublicKey
  newShagaAuthority?: PublicKey
  feeDestination?: PublicKey
  feeBasisPoints?: number
  isPaused?: boolean

}

export function updateShagaState({
  payer,
  newShagaAuthority,
  feeDestination,
  feeBasisPoints,
  isPaused,
}: UpdateShagaStateType) {

  const [shagaState] = findShagaState();

  const initializeAccountsIx = createUpdateShagaStateInstruction(
    {
      shagaAuthority: payer,
      shagaState,
    },
    {
      newShagaAuthority,
      feeDestination,
      feeBasisPoints,
      isPaused
    }
  )
  return initializeAccountsIx
}