import type {
  TeamMemberInput,
  TeamMember,
  Role,
  Activity,
} from '@/lib/types/business'

export class TeamService {
  private baseUrl = '/api/team'

  async addMember(member: TeamMemberInput): Promise<TeamMember> {
    try {
      const response = await fetch(`${this.baseUrl}/members`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(member),
      })

      if (!response.ok) {
        throw new Error('Failed to add team member')
      }

      return await response.json()
    } catch (error) {
      console.error('Error adding team member:', error)
      throw error
    }
  }

  async removeMember(memberId: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/members/${memberId}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error('Failed to remove team member')
      }
    } catch (error) {
      console.error('Error removing team member:', error)
      throw error
    }
  }

  async updateRole(memberId: string, role: Role): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/members/${memberId}/role`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ role }),
      })

      if (!response.ok) {
        throw new Error('Failed to update member role')
      }
    } catch (error) {
      console.error('Error updating member role:', error)
      throw error
    }
  }

  async getMembers(): Promise<TeamMember[]> {
    try {
      const response = await fetch(`${this.baseUrl}/members`)

      if (!response.ok) {
        throw new Error('Failed to fetch team members')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching team members:', error)
      throw error
    }
  }

  async getMemberActivity(memberId: string): Promise<Activity[]> {
    try {
      const response = await fetch(
        `${this.baseUrl}/members/${memberId}/activity`
      )

      if (!response.ok) {
        throw new Error('Failed to fetch member activity')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching member activity:', error)
      throw error
    }
  }

  async updatePermissions(
    memberId: string,
    permissions: TeamMember['permissions']
  ): Promise<void> {
    try {
      const response = await fetch(
        `${this.baseUrl}/members/${memberId}/permissions`,
        {
          method: 'PATCH',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ permissions }),
        }
      )

      if (!response.ok) {
        throw new Error('Failed to update member permissions')
      }
    } catch (error) {
      console.error('Error updating member permissions:', error)
      throw error
    }
  }
}

export const teamService = new TeamService()
