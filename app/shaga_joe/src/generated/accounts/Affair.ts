/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'
import { AffairState, affairStateBeet } from '../types/AffairState'

/**
 * Arguments used to create {@link Affair}
 * @category Accounts
 * @category generated
 */
export type AffairArgs = {
  authority: web3.PublicKey
  client: web3.PublicKey
  rental: beet.COption<web3.PublicKey>
  coordinates: string
  ipAddress: string
  cpuName: string
  gpuName: string
  totalRamMb: number
  solPerHour: beet.bignum
  affairState: AffairState
  affairTerminationTime: beet.bignum
  activeRentalStartTime: beet.bignum
  dueRentAmount: beet.bignum
}

export const affairDiscriminator = [232, 123, 195, 244, 163, 184, 124, 67]
/**
 * Holds the data for the {@link Affair} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class Affair implements AffairArgs {
  private constructor(
    readonly authority: web3.PublicKey,
    readonly client: web3.PublicKey,
    readonly rental: beet.COption<web3.PublicKey>,
    readonly coordinates: string,
    readonly ipAddress: string,
    readonly cpuName: string,
    readonly gpuName: string,
    readonly totalRamMb: number,
    readonly solPerHour: beet.bignum,
    readonly affairState: AffairState,
    readonly affairTerminationTime: beet.bignum,
    readonly activeRentalStartTime: beet.bignum,
    readonly dueRentAmount: beet.bignum
  ) {}

  /**
   * Creates a {@link Affair} instance from the provided args.
   */
  static fromArgs(args: AffairArgs) {
    return new Affair(
      args.authority,
      args.client,
      args.rental,
      args.coordinates,
      args.ipAddress,
      args.cpuName,
      args.gpuName,
      args.totalRamMb,
      args.solPerHour,
      args.affairState,
      args.affairTerminationTime,
      args.activeRentalStartTime,
      args.dueRentAmount
    )
  }

  /**
   * Deserializes the {@link Affair} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [Affair, number] {
    return Affair.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link Affair} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<Affair> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find Affair account at ${address}`)
    }
    return Affair.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      '9SwYZxTQUYruFSHYeTqrtB5pTtuGJEGksh7ufpNS1YK5'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, affairBeet)
  }

  /**
   * Deserializes the {@link Affair} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [Affair, number] {
    return affairBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link Affair} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return affairBeet.serialize({
      accountDiscriminator: affairDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link Affair} for the provided args.
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   */
  static byteSize(args: AffairArgs) {
    const instance = Affair.fromArgs(args)
    return affairBeet.toFixedFromValue({
      accountDiscriminator: affairDiscriminator,
      ...instance,
    }).byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link Affair} data from rent
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    args: AffairArgs,
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      Affair.byteSize(args),
      commitment
    )
  }

  /**
   * Returns a readable version of {@link Affair} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      authority: this.authority.toBase58(),
      client: this.client.toBase58(),
      rental: this.rental,
      coordinates: this.coordinates,
      ipAddress: this.ipAddress,
      cpuName: this.cpuName,
      gpuName: this.gpuName,
      totalRamMb: this.totalRamMb,
      solPerHour: (() => {
        const x = <{ toNumber: () => number }>this.solPerHour
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      affairState: 'AffairState.' + AffairState[this.affairState],
      affairTerminationTime: (() => {
        const x = <{ toNumber: () => number }>this.affairTerminationTime
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      activeRentalStartTime: (() => {
        const x = <{ toNumber: () => number }>this.activeRentalStartTime
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      dueRentAmount: (() => {
        const x = <{ toNumber: () => number }>this.dueRentAmount
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const affairBeet = new beet.FixableBeetStruct<
  Affair,
  AffairArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['authority', beetSolana.publicKey],
    ['client', beetSolana.publicKey],
    ['rental', beet.coption(beetSolana.publicKey)],
    ['coordinates', beet.utf8String],
    ['ipAddress', beet.utf8String],
    ['cpuName', beet.utf8String],
    ['gpuName', beet.utf8String],
    ['totalRamMb', beet.u32],
    ['solPerHour', beet.u64],
    ['affairState', affairStateBeet],
    ['affairTerminationTime', beet.u64],
    ['activeRentalStartTime', beet.u64],
    ['dueRentAmount', beet.u64],
  ],
  Affair.fromArgs,
  'Affair'
)
