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
 * @category InitializeLender
 * @category generated
 */
export const initializeLenderStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */
}>(
  [['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)]],
  'InitializeLenderInstructionArgs'
)
/**
 * Accounts required by the _initializeLender_ instruction
 *
 * @property [_writable_, **signer**] payer
 * @property [_writable_] lender
 * @category Instructions
 * @category InitializeLender
 * @category generated
 */
export type InitializeLenderInstructionAccounts = {
  payer: web3.PublicKey
  lender: web3.PublicKey
  systemProgram?: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const initializeLenderInstructionDiscriminator = [
  116, 27, 248, 201, 199, 38, 118, 15,
]

/**
 * Creates a _InitializeLender_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category InitializeLender
 * @category generated
 */
export function createInitializeLenderInstruction(
  accounts: InitializeLenderInstructionAccounts,
  programId = new web3.PublicKey('HQeckNoXMczA5AtgKKWmLzQPT4Wcm6YBjeHCrRp2XLF1')
) {
  const [data] = initializeLenderStruct.serialize({
    instructionDiscriminator: initializeLenderInstructionDiscriminator,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.lender,
      isWritable: true,
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
