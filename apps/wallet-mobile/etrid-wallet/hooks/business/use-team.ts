"use client"

import { useState, useEffect } from 'react'
import { teamService } from '@/lib/services/TeamService'
import type { TeamMember, TeamMemberInput, Role } from '@/lib/types/business'

export function useTeam() {
  const [members, setMembers] = useState<TeamMember[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchMembers = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await teamService.getMembers()
      setMembers(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const addMember = async (member: TeamMemberInput) => {
    try {
      setError(null)
      const newMember = await teamService.addMember(member)
      setMembers([...members, newMember])
      return newMember
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const removeMember = async (memberId: string) => {
    try {
      setError(null)
      await teamService.removeMember(memberId)
      setMembers(members.filter((m) => m.id !== memberId))
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const updateRole = async (memberId: string, role: Role) => {
    try {
      setError(null)
      await teamService.updateRole(memberId, role)
      setMembers(
        members.map((m) => (m.id === memberId ? { ...m, role } : m))
      )
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  useEffect(() => {
    fetchMembers()
  }, [])

  return {
    members,
    loading,
    error,
    refetch: fetchMembers,
    addMember,
    removeMember,
    updateRole,
  }
}
