/**
 * Governance proposal (Snapshot)
 */
export interface Proposal {
  id: string
  title: string
  body: string
  choices: string[]
  start: Date
  end: Date
  snapshot: number
  state: 'active' | 'closed' | 'pending'
  author: string
  scores: number[]
  scoresTotal: number
  votes: number
  space?: string
}

/**
 * Vote on a proposal
 */
export interface Vote {
  id: string
  voter: string
  proposalId: string
  choice: number | number[]
  votingPower: number
  timestamp: Date
  reason?: string
}

/**
 * Director information (9 directors)
 */
export interface Director {
  address: string
  name: string
  title: string
  startDate: Date
  endDate?: Date
  votingPower: number
  proposalsCreated: number
  votesParticipated: number
  status: 'active' | 'inactive'
}

/**
 * Consensus Day event
 */
export interface ConsensusDay {
  id: string
  date: Date
  year: number
  proposals: string[]
  participants: number
  status: 'upcoming' | 'active' | 'completed'
  results?: ConsensusDayResult[]
}

/**
 * Consensus Day result
 */
export interface ConsensusDayResult {
  proposalId: string
  passed: boolean
  votesFor: number
  votesAgainst: number
  votesAbstain: number
  turnout: number
}

/**
 * Governance space (Snapshot space)
 */
export interface GovernanceSpace {
  id: string
  name: string
  about?: string
  network: string
  symbol: string
  members: string[]
  admins: string[]
  proposalCount: number
  voterCount: number
}
