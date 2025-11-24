'use client'

import { NetworkNode } from '@/lib/hooks/useNetworkStats'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

interface NodeExplorerProps {
  nodes: NetworkNode[]
}

export function NodeExplorer({ nodes }: NodeExplorerProps) {
  return (
    <div className="rounded-lg border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Node Name</TableHead>
            <TableHead>Type</TableHead>
            <TableHead>Location</TableHead>
            <TableHead>Status</TableHead>
            <TableHead>Version</TableHead>
            <TableHead className="text-right">Block Height</TableHead>
            <TableHead className="text-right">Peers</TableHead>
            <TableHead className="text-right">Uptime</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {nodes.length === 0 ? (
            <TableRow>
              <TableCell colSpan={8} className="text-center py-8 text-muted-foreground">
                No nodes detected. Waiting for blockchain to start...
              </TableCell>
            </TableRow>
          ) : (
            nodes.map((node, index) => (
              <TableRow key={index} className="hover:bg-muted/50">
                <TableCell className="font-medium">{node.name}</TableCell>
                <TableCell>
                  <span className={`inline-flex px-2 py-1 rounded-md text-xs font-semibold ${getTypeBadgeClass(node.type)}`}>
                    {node.type.replace('-', ' ').toUpperCase()}
                  </span>
                </TableCell>
                <TableCell>{node.location}</TableCell>
                <TableCell>
                  <span className={`inline-flex items-center gap-1.5 px-2 py-1 rounded-md text-xs font-semibold ${getStatusBadgeClass(node.status)}`}>
                    <span className="w-1.5 h-1.5 rounded-full bg-current" />
                    {node.status.toUpperCase()}
                  </span>
                </TableCell>
                <TableCell className="font-mono text-sm">{node.version}</TableCell>
                <TableCell className="text-right font-mono">{node.block.toLocaleString()}</TableCell>
                <TableCell className="text-right">{node.peers}</TableCell>
                <TableCell className="text-right">{node.uptime}</TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </div>
  )
}

function getTypeBadgeClass(type: string): string {
  switch (type) {
    case 'bootstrap':
      return 'bg-green-100 text-green-700 dark:bg-green-950 dark:text-green-400'
    case 'validator':
      return 'bg-orange-100 text-orange-700 dark:bg-orange-950 dark:text-orange-400'
    case 'full-node':
      return 'bg-blue-100 text-blue-700 dark:bg-blue-950 dark:text-blue-400'
    default:
      return 'bg-gray-100 text-gray-700 dark:bg-gray-950 dark:text-gray-400'
  }
}

function getStatusBadgeClass(status: string): string {
  switch (status) {
    case 'online':
      return 'bg-green-100 text-green-700 dark:bg-green-950 dark:text-green-400'
    case 'offline':
      return 'bg-red-100 text-red-700 dark:bg-red-950 dark:text-red-400'
    case 'syncing':
      return 'bg-yellow-100 text-yellow-700 dark:bg-yellow-950 dark:text-yellow-400'
    default:
      return 'bg-gray-100 text-gray-700 dark:bg-gray-950 dark:text-gray-400'
  }
}
