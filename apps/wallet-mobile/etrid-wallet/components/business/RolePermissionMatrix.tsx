"use client"

import { Check, X } from "lucide-react"
import type { Role } from "@/lib/types/business"

interface PermissionDef {
  resource: string
  label: string
  actions: string[]
}

const permissions: PermissionDef[] = [
  { resource: "invoices", label: "Invoices", actions: ["create", "read", "update", "delete"] },
  { resource: "payroll", label: "Payroll", actions: ["create", "read", "update", "delete"] },
  { resource: "expenses", label: "Expenses", actions: ["create", "read", "update", "delete"] },
  { resource: "team", label: "Team Management", actions: ["create", "read", "update", "delete"] },
  { resource: "settings", label: "Settings", actions: ["read", "update"] },
]

const rolePermissions: Record<Role, Record<string, string[]>> = {
  owner: {
    invoices: ["create", "read", "update", "delete"],
    payroll: ["create", "read", "update", "delete"],
    expenses: ["create", "read", "update", "delete"],
    team: ["create", "read", "update", "delete"],
    settings: ["read", "update"],
  },
  admin: {
    invoices: ["create", "read", "update", "delete"],
    payroll: ["create", "read", "update", "delete"],
    expenses: ["create", "read", "update", "delete"],
    team: ["create", "read", "update"],
    settings: ["read"],
  },
  manager: {
    invoices: ["create", "read", "update"],
    payroll: ["read"],
    expenses: ["create", "read", "update"],
    team: ["read"],
    settings: ["read"],
  },
  viewer: {
    invoices: ["read"],
    payroll: ["read"],
    expenses: ["read"],
    team: ["read"],
    settings: ["read"],
  },
}

const roleColors: Record<Role, string> = {
  owner: "bg-purple-500/20 text-purple-400",
  admin: "bg-blue-500/20 text-blue-400",
  manager: "bg-green-500/20 text-green-400",
  viewer: "bg-gray-500/20 text-gray-400",
}

export function RolePermissionMatrix() {
  const roles: Role[] = ["owner", "admin", "manager", "viewer"]

  const hasPermission = (role: Role, resource: string, action: string) => {
    return rolePermissions[role][resource]?.includes(action) || false
  }

  return (
    <div className="glass-strong rounded-lg border border-border overflow-hidden">
      <div className="p-4 border-b border-border">
        <h3 className="text-lg font-semibold text-foreground">
          Role Permissions Matrix
        </h3>
        <p className="text-sm text-muted-foreground mt-1">
          Permissions for each role level
        </p>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="border-b border-border">
              <th className="text-left p-4 text-sm font-semibold text-foreground w-48">
                Resource
              </th>
              {roles.map((role) => (
                <th key={role} className="text-center p-4">
                  <span
                    className={`text-xs px-3 py-1 rounded-full ${roleColors[role]}`}
                  >
                    {role}
                  </span>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {permissions.map((perm, idx) => (
              <tr
                key={perm.resource}
                className={idx !== permissions.length - 1 ? "border-b border-border" : ""}
              >
                <td className="p-4">
                  <div className="font-medium text-foreground">
                    {perm.label}
                  </div>
                  <div className="text-xs text-muted-foreground">
                    {perm.actions.join(", ")}
                  </div>
                </td>
                {roles.map((role) => {
                  const hasFullAccess = perm.actions.every((action) =>
                    hasPermission(role, perm.resource, action)
                  )
                  const hasPartialAccess =
                    !hasFullAccess &&
                    perm.actions.some((action) =>
                      hasPermission(role, perm.resource, action)
                    )

                  return (
                    <td key={`${perm.resource}-${role}`} className="p-4 text-center">
                      {hasFullAccess ? (
                        <div className="flex items-center justify-center">
                          <div className="w-6 h-6 rounded-full bg-green-500/20 flex items-center justify-center">
                            <Check className="w-4 h-4 text-green-400" />
                          </div>
                        </div>
                      ) : hasPartialAccess ? (
                        <div className="flex items-center justify-center">
                          <div className="w-6 h-6 rounded-full bg-yellow-500/20 flex items-center justify-center">
                            <span className="text-xs text-yellow-400">R</span>
                          </div>
                        </div>
                      ) : (
                        <div className="flex items-center justify-center">
                          <div className="w-6 h-6 rounded-full bg-red-500/20 flex items-center justify-center">
                            <X className="w-4 h-4 text-red-400" />
                          </div>
                        </div>
                      )}
                    </td>
                  )
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="p-4 border-t border-border bg-accent/5">
        <div className="flex items-center gap-4 text-xs text-muted-foreground">
          <div className="flex items-center gap-2">
            <Check className="w-4 h-4 text-green-400" />
            <span>Full Access</span>
          </div>
          <div className="flex items-center gap-2">
            <span className="w-4 h-4 rounded-full bg-yellow-500/20 flex items-center justify-center text-yellow-400">
              R
            </span>
            <span>Read Only</span>
          </div>
          <div className="flex items-center gap-2">
            <X className="w-4 h-4 text-red-400" />
            <span>No Access</span>
          </div>
        </div>
      </div>
    </div>
  )
}
