"use client"

import { Input } from "@/components/ui/input"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Search } from "lucide-react"

interface FilterBarProps {
  selectedFilter: string
  onFilterChange: (filter: string) => void
  sortBy: string
  onSortChange: (sort: string) => void
  searchQuery: string
  onSearchChange: (query: string) => void
}

const filters = [
  { value: "all", label: "All Proposals" },
  { value: "fiscal", label: "Fiscal Policy" },
  { value: "protocol", label: "Protocol Upgrades" },
  { value: "treasury", label: "Treasury" },
  { value: "voted", label: "Voted" },
  { value: "not-voted", label: "Not Voted" },
]

export default function FilterBar({
  selectedFilter,
  onFilterChange,
  sortBy,
  onSortChange,
  searchQuery,
  onSearchChange,
}: FilterBarProps) {
  return (
    <div className="mb-6 space-y-4">
      <div className="flex flex-wrap gap-2">
        {filters.map((filter) => (
          <button
            key={filter.value}
            onClick={() => onFilterChange(filter.value)}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
              selectedFilter === filter.value
                ? "bg-accent text-accent-foreground"
                : "bg-card hover:bg-accent/10 text-muted-foreground"
            }`}
          >
            {filter.label}
          </button>
        ))}
      </div>

      <div className="flex flex-col sm:flex-row gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <Input
            placeholder="Search proposals..."
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            className="pl-10"
          />
        </div>

        <Select value={sortBy} onValueChange={onSortChange}>
          <SelectTrigger className="w-full sm:w-48">
            <SelectValue placeholder="Sort by" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="most-votes">Most votes</SelectItem>
            <SelectItem value="ending-soon">Ending soon</SelectItem>
            <SelectItem value="recently-added">Recently added</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
  )
}
