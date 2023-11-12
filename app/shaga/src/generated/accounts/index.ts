export * from './Affair'
export * from './AffairsList'
export * from './Escrow'
export * from './Lender'
export * from './Rental'
export * from './ShagaState'

import { AffairsList } from './AffairsList'
import { Affair } from './Affair'
import { Escrow } from './Escrow'
import { Lender } from './Lender'
import { Rental } from './Rental'
import { ShagaState } from './ShagaState'

export const accountProviders = {
  AffairsList,
  Affair,
  Escrow,
  Lender,
  Rental,
  ShagaState,
}
