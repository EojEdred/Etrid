"use client"

import { ArrowLeft, Plus, UserPlus } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useTeam } from "@/hooks/business/use-team"
import { TeamMemberCard } from "@/components/business/TeamMemberCard"
import { RolePermissionMatrix } from "@/components/business/RolePermissionMatrix"
import { useState } from "react"

interface TeamManagementScreenProps {
  onBack: () => void
}

export function TeamManagementScreen({ onBack }: TeamManagementScreenProps) {
  const { members, loading, removeMember, updateRole } = useTeam()
  const [activeTab, setActiveTab] = useState("members")

  const handleRemove = async (member: any) => {
    if (confirm(`Remove ${member.name} from the team?`)) {
      try {
        await removeMember(member.id)
      } catch (error) {
        console.error("Failed to remove member:", error)
      }
    }
  }

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="glass-strong border-b border-border sticky top-0 z-10">
        <div className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <h1 className="text-xl font-bold text-foreground">Team Management</h1>
          </div>
          <Button size="sm">
            <UserPlus className="w-4 h-4" />
            Add Member
          </Button>
        </div>
      </header>

      <main className="px-4 py-6">
        <Tabs value={activeTab} onValueChange={setActiveTab}>
          <TabsList className="w-full grid grid-cols-2 mb-6">
            <TabsTrigger value="members">Team Members</TabsTrigger>
            <TabsTrigger value="permissions">Permissions</TabsTrigger>
          </TabsList>

          <TabsContent value="members" className="space-y-4">
            {loading ? (
              <p className="text-center text-muted-foreground">Loading...</p>
            ) : members.length === 0 ? (
              <div className="glass-strong rounded-lg p-8 border border-border text-center">
                <UserPlus className="w-12 h-12 text-muted-foreground mx-auto mb-3" />
                <p className="text-foreground font-medium mb-1">No team members yet</p>
                <p className="text-sm text-muted-foreground mb-4">
                  Add your first team member to get started
                </p>
                <Button>
                  <Plus className="w-4 h-4" />
                  Add Member
                </Button>
              </div>
            ) : (
              members.map((member) => (
                <TeamMemberCard
                  key={member.id}
                  member={member}
                  onRemove={handleRemove}
                  onChangeRole={(m) => {
                    // Handle role change
                  }}
                />
              ))
            )}
          </TabsContent>

          <TabsContent value="permissions">
            <RolePermissionMatrix />
          </TabsContent>
        </Tabs>
      </main>
    </div>
  )
}
