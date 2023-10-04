/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category TerminateVacantAffair
 * @category generated
 */
export const terminateVacantAffairStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */
}>(
  [['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)]],
  'TerminateVacantAffairInstructionArgs'
)
/**
 * Accounts required by the _terminateVacantAffair_ instruction
 *
 * @property [_writable_, **signer**] signer
 * @property [_writable_] affair
 * @property [_writable_] affairsList
 * @property [_writable_] vault
 * @property [] threadAuthority
 * @category Instructions
 * @category TerminateVacantAffair
 * @category generated
 */
export type TerminateVacantAffairInstructionAccounts = {
  signer: web3.PublicKey
  affair: web3.PublicKey
  affairsList: web3.PublicKey
  vault: web3.PublicKey
  threadAuthority: web3.PublicKey
  systemProgram?: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const terminateVacantAffairInstructionDiscriminator = [
  216, 170, 159, 139, 187, 64, 245, 212,
]

/**
 * Creates a _TerminateVacantAffair_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category TerminateVacantAffair
 * @category generated
 */
export function createTerminateVacantAffairInstruction(
  accounts: TerminateVacantAffairInstructionAccounts,
  programId = new web3.PublicKey('9SwYZxTQUYruFSHYeTqrtB5pTtuGJEGksh7ufpNS1YK5')
) {
  const [data] = terminateVacantAffairStruct.serialize({
    instructionDiscriminator: terminateVacantAffairInstructionDiscriminator,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.signer,
      isWritable: true,
      isSigner: true,
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
      pubkey: accounts.vault,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.threadAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
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
