"use client"

import { User, Mail, MoreVertical } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import type { TeamMember } from "@/lib/types/business"
import { formatDistanceToNow } from "date-fns"

interface TeamMemberCardProps {
  member: TeamMember
  onEdit?: (member: TeamMember) => void
  onRemove?: (member: TeamMember) => void
  onChangeRole?: (member: TeamMember) => void
}

const roleColors = {
  owner: "bg-purple-500/20 text-purple-400 border-purple-500/30",
  admin: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  manager: "bg-green-500/20 text-green-400 border-green-500/30",
  viewer: "bg-gray-500/20 text-gray-400 border-gray-500/30",
}

export function TeamMemberCard({
  member,
  onEdit,
  onRemove,
  onChangeRole,
}: TeamMemberCardProps) {
  const initials = member.name
    .split(" ")
    .map((n) => n[0])
    .join("")
    .toUpperCase()

  return (
    <div className="glass-strong rounded-lg p-4 border border-border">
      <div className="flex items-start justify-between">
        <div className="flex items-start gap-3 flex-1">
          <Avatar className="w-12 h-12">
            {member.avatar_url && <AvatarImage src={member.avatar_url} />}
            <AvatarFallback className="bg-accent/20 text-accent">
              <User className="w-5 h-5" />
            </AvatarFallback>
          </Avatar>

          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <h3 className="font-semibold text-foreground truncate">
                {member.name}
              </h3>
              <span
                className={`text-xs px-2 py-0.5 rounded-full border ${
                  roleColors[member.role]
                }`}
              >
                {member.role}
              </span>
            </div>

            <div className="flex items-center gap-1 text-sm text-muted-foreground mb-2">
              <Mail className="w-3 h-3" />
              <span className="truncate">{member.email}</span>
            </div>

            {member.last_active && (
              <p className="text-xs text-muted-foreground">
                Active {formatDistanceToNow(new Date(member.last_active), { addSuffix: true })}
              </p>
            )}
          </div>
        </div>

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="icon-sm">
              <MoreVertical className="w-4 h-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            {onEdit && (
              <DropdownMenuItem onClick={() => onEdit(member)}>
                Edit Member
              </DropdownMenuItem>
            )}
            {onChangeRole && (
              <DropdownMenuItem onClick={() => onChangeRole(member)}>
                Change Role
              </DropdownMenuItem>
            )}
            {onRemove && member.role !== "owner" && (
              <DropdownMenuItem
                onClick={() => onRemove(member)}
                className="text-destructive"
              >
                Remove Member
              </DropdownMenuItem>
            )}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  )
}
