"use client"

import { useState, useEffect } from "react"

export interface SnapshotProposal {
  id: string
  title: string
  body: string
  choices: string[]
  start: number
  end: number
  snapshot: string
  state: "active" | "closed" | "pending"
  author: string
  space: {
    id: string
    name: string
  }
  scores: number[]
  scores_total: number
  votes: number
}

interface UseSnapshotProposalsOptions {
  spaceId?: string
  state?: "active" | "closed" | "pending" | "all"
  first?: number
}

const SNAPSHOT_HUB_URL = "https://hub.snapshot.org/graphql"

export function useSnapshotProposals(options: UseSnapshotProposalsOptions = {}) {
  const { spaceId = "etrid.eth", state = "active", first = 20 } = options

  const [proposals, setProposals] = useState<SnapshotProposal[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchProposals = async () => {
      setLoading(true)
      setError(null)

      try {
        const query = `
          query Proposals($space: String!, $state: String!, $first: Int!) {
            proposals(
              first: $first
              where: { space_in: [$space], state: $state }
              orderBy: "created"
              orderDirection: desc
            ) {
              id
              title
              body
              choices
              start
              end
              snapshot
              state
              author
              space {
                id
                name
              }
              scores
              scores_total
              votes
            }
          }
        `

        const variables = {
          space: spaceId,
          state: state === "all" ? "" : state,
          first
        }

        const response = await fetch(SNAPSHOT_HUB_URL, {
          method: "POST",
          headers: {
            "Content-Type": "application/json"
          },
          body: JSON.stringify({
            query,
            variables
          })
        })

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }

        const data = await response.json()

        if (data.errors) {
          throw new Error(data.errors[0].message)
        }

        setProposals(data.data.proposals || [])
      } catch (err) {
        console.error("Error fetching Snapshot proposals:", err)
        setError(err instanceof Error ? err.message : "Failed to fetch proposals")
      } finally {
        setLoading(false)
      }
    }

    fetchProposals()
  }, [spaceId, state, first])

  return { proposals, loading, error }
}

export function useSnapshotProposal(proposalId: string) {
  const [proposal, setProposal] = useState<SnapshotProposal | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchProposal = async () => {
      setLoading(true)
      setError(null)

      try {
        const query = `
          query Proposal($id: String!) {
            proposal(id: $id) {
              id
              title
              body
              choices
              start
              end
              snapshot
              state
              author
              space {
                id
                name
              }
              scores
              scores_total
              votes
            }
          }
        `

        const response = await fetch(SNAPSHOT_HUB_URL, {
          method: "POST",
          headers: {
            "Content-Type": "application/json"
          },
          body: JSON.stringify({
            query,
            variables: { id: proposalId }
          })
        })

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }

        const data = await response.json()

        if (data.errors) {
          throw new Error(data.errors[0].message)
        }

        setProposal(data.data.proposal)
      } catch (err) {
        console.error("Error fetching Snapshot proposal:", err)
        setError(err instanceof Error ? err.message : "Failed to fetch proposal")
      } finally {
        setLoading(false)
      }
    }

    if (proposalId) {
      fetchProposal()
    }
  }, [proposalId])

  return { proposal, loading, error }
}

export interface SnapshotVote {
  id: string
  voter: string
  created: number
  choice: number | number[]
  vp: number
  proposal: {
    id: string
  }
}

export function useSnapshotVotes(proposalId: string, first: number = 1000) {
  const [votes, setVotes] = useState<SnapshotVote[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchVotes = async () => {
      setLoading(true)
      setError(null)

      try {
        const query = `
          query Votes($proposal: String!, $first: Int!) {
            votes(
              first: $first
              where: { proposal: $proposal }
              orderBy: "vp"
              orderDirection: desc
            ) {
              id
              voter
              created
              choice
              vp
              proposal {
                id
              }
            }
          }
        `

        const response = await fetch(SNAPSHOT_HUB_URL, {
          method: "POST",
          headers: {
            "Content-Type": "application/json"
          },
          body: JSON.stringify({
            query,
            variables: { proposal: proposalId, first }
          })
        })

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }

        const data = await response.json()

        if (data.errors) {
          throw new Error(data.errors[0].message)
        }

        setVotes(data.data.votes || [])
      } catch (err) {
        console.error("Error fetching Snapshot votes:", err)
        setError(err instanceof Error ? err.message : "Failed to fetch votes")
      } finally {
        setLoading(false)
      }
    }

    if (proposalId) {
      fetchVotes()
    }
  }, [proposalId, first])

  return { votes, loading, error }
}

export interface SnapshotSpace {
  id: string
  name: string
  about: string
  network: string
  symbol: string
  members: string[]
  admins: string[]
  proposalsCount: number
  followersCount: number
  avatar: string
}

export function useSnapshotSpace(spaceId: string = "etrid.eth") {
  const [space, setSpace] = useState<SnapshotSpace | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchSpace = async () => {
      setLoading(true)
      setError(null)

      try {
        const query = `
          query Space($id: String!) {
            space(id: $id) {
              id
              name
              about
              network
              symbol
              members
              admins
              proposalsCount
              followersCount
              avatar
            }
          }
        `

        const response = await fetch(SNAPSHOT_HUB_URL, {
          method: "POST",
          headers: {
            "Content-Type": "application/json"
          },
          body: JSON.stringify({
            query,
            variables: { id: spaceId }
          })
        })

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }

        const data = await response.json()

        if (data.errors) {
          throw new Error(data.errors[0].message)
        }

        setSpace(data.data.space)
      } catch (err) {
        console.error("Error fetching Snapshot space:", err)
        setError(err instanceof Error ? err.message : "Failed to fetch space")
      } finally {
        setLoading(false)
      }
    }

    if (spaceId) {
      fetchSpace()
    }
  }, [spaceId])

  return { space, loading, error }
}
