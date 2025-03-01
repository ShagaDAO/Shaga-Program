/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  RentalTerminationAuthority,
  rentalTerminationAuthorityBeet,
} from '../types/RentalTerminationAuthority'

/**
 * @category Instructions
 * @category EndRental
 * @category generated
 */
export type EndRentalInstructionArgs = {
  terminationBy: RentalTerminationAuthority
}
/**
 * @category Instructions
 * @category EndRental
 * @category generated
 */
export const endRentalStruct = new beet.BeetArgsStruct<
  EndRentalInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['terminationBy', rentalTerminationAuthorityBeet],
  ],
  'EndRentalInstructionArgs'
)
/**
 * Accounts required by the _endRental_ instruction
 *
 * @property [_writable_, **signer**] signer
 * @property [_writable_] client
 * @property [_writable_] shagaState
 * @property [] threadAuthority
 * @property [_writable_] lender
 * @property [_writable_] affair
 * @property [_writable_] affairsList
 * @property [_writable_] escrow
 * @property [_writable_] rental
 * @property [_writable_] rentalClockworkThread
 * @property [] clockworkProgram
 * @category Instructions
 * @category EndRental
 * @category generated
 */
export type EndRentalInstructionAccounts = {
  signer: web3.PublicKey
  client: web3.PublicKey
  shagaState: web3.PublicKey
  threadAuthority: web3.PublicKey
  lender: web3.PublicKey
  affair: web3.PublicKey
  affairsList: web3.PublicKey
  escrow: web3.PublicKey
  rental: web3.PublicKey
  rentalClockworkThread: web3.PublicKey
  systemProgram?: web3.PublicKey
  clockworkProgram: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const endRentalInstructionDiscriminator = [
  80, 139, 137, 253, 17, 175, 149, 20,
]

/**
 * Creates a _EndRental_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category EndRental
 * @category generated
 */
export function createEndRentalInstruction(
  accounts: EndRentalInstructionAccounts,
  args: EndRentalInstructionArgs,
  programId = new web3.PublicKey('HQeckNoXMczA5AtgKKWmLzQPT4Wcm6YBjeHCrRp2XLF1')
) {
  const [data] = endRentalStruct.serialize({
    instructionDiscriminator: endRentalInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.signer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.client,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.shagaState,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.threadAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.lender,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.affair,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.affairsList,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.escrow,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.rental,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.rentalClockworkThread,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.clockworkProgram,
      isWritable: false,
      isSigner: false,
    },
  ]

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc)
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
