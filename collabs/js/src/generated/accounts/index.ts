export * from './ContributorAccount'
export * from './XpPoolAccount'

import { XpPoolAccount } from './XpPoolAccount'
import { ContributorAccount } from './ContributorAccount'

export const accountProviders = { XpPoolAccount, ContributorAccount }
