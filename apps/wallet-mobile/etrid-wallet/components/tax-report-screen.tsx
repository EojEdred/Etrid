'use client'

import { ArrowLeft, Download, FileText, DollarSign } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { useTaxReport } from '@/hooks/useTaxReport'
import { TaxLossCard } from '@/components/analytics/TaxLossCard'
import { useState } from 'react'
import { CostBasisMethod } from '@/lib/types/analytics'

interface TaxReportScreenProps {
  onBack: () => void
}

export function TaxReportScreen({ onBack }: TaxReportScreenProps) {
  const currentYear = new Date().getFullYear()
  const [selectedYear, setSelectedYear] = useState(currentYear)
  const {
    capitalGains,
    opportunities,
    loading,
    costBasisMethod,
    setCostBasisMethod,
    harvestLoss,
    exportToTurboTax,
    generateForm8949,
  } = useTaxReport(selectedYear)

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading tax report...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Tax Report</h1>
            <p className="text-sm text-muted-foreground">
              Optimize your tax strategy
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Year & Method Selection */}
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="text-sm font-medium mb-2 block">Tax Year</label>
            <Select
              value={selectedYear.toString()}
              onValueChange={(value) => setSelectedYear(parseInt(value))}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {[currentYear, currentYear - 1, currentYear - 2].map((year) => (
                  <SelectItem key={year} value={year.toString()}>
                    {year}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <label className="text-sm font-medium mb-2 block">Cost Basis</label>
            <Select
              value={costBasisMethod}
              onValueChange={(value) => setCostBasisMethod(value as CostBasisMethod)}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="FIFO">FIFO</SelectItem>
                <SelectItem value="LIFO">LIFO</SelectItem>
                <SelectItem value="HIFO">HIFO</SelectItem>
                <SelectItem value="SpecificID">Specific ID</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>

        {/* Capital Gains Summary */}
        {capitalGains && (
          <Card className="p-6">
            <h3 className="text-lg font-semibold mb-4">Capital Gains Summary</h3>

            <div className="grid grid-cols-2 gap-4 mb-4">
              <div>
                <p className="text-xs text-muted-foreground mb-1">Short-Term Gains</p>
                <p className="text-xl font-bold">
                  ${capitalGains.shortTerm.toLocaleString()}
                </p>
                <p className="text-xs text-muted-foreground">Taxed as income</p>
              </div>
              <div>
                <p className="text-xs text-muted-foreground mb-1">Long-Term Gains</p>
                <p className="text-xl font-bold text-green-500">
                  ${capitalGains.longTerm.toLocaleString()}
                </p>
                <p className="text-xs text-muted-foreground">Lower tax rate</p>
              </div>
            </div>

            <div className="border-t pt-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm">Total Gains</span>
                <span className="font-semibold">
                  ${capitalGains.totalGains.toLocaleString()}
                </span>
              </div>
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm">Total Losses</span>
                <span className="font-semibold text-red-500">
                  -${capitalGains.totalLosses.toLocaleString()}
                </span>
              </div>
              <div className="flex items-center justify-between mb-2 text-lg font-bold">
                <span>Net Gains</span>
                <span className={capitalGains.netGains >= 0 ? 'text-green-500' : 'text-red-500'}>
                  ${capitalGains.netGains.toLocaleString()}
                </span>
              </div>
              <div className="flex items-center justify-between pt-2 border-t">
                <span className="text-sm">Estimated Tax</span>
                <span className="font-semibold text-orange-500">
                  ${capitalGains.estimatedTax.toLocaleString()}
                </span>
              </div>
            </div>
          </Card>
        )}

        {/* Tax Loss Harvesting */}
        {opportunities.length > 0 && (
          <div>
            <h3 className="text-lg font-semibold mb-3">
              Tax Loss Harvesting Opportunities
            </h3>
            <div className="space-y-4">
              {opportunities.map((opp) => (
                <TaxLossCard
                  key={opp.asset}
                  opportunity={opp}
                  onHarvest={harvestLoss}
                />
              ))}
            </div>
          </div>
        )}

        {/* Export Options */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
            <Download className="w-5 h-5" />
            Export Tax Documents
          </h3>
          <div className="space-y-3">
            <Button
              variant="outline"
              className="w-full justify-start"
              onClick={exportToTurboTax}
            >
              <FileText className="w-4 h-4 mr-2" />
              Export to TurboTax (CSV)
            </Button>
            <Button
              variant="outline"
              className="w-full justify-start"
              onClick={generateForm8949}
            >
              <FileText className="w-4 h-4 mr-2" />
              Generate Form 8949 (PDF)
            </Button>
          </div>
        </Card>

        {/* Disclaimer */}
        <Card className="p-4 bg-yellow-500/10 border-yellow-500/20">
          <p className="text-xs text-muted-foreground">
            <strong>Disclaimer:</strong> This is an estimate based on available data.
            Consult a tax professional for accurate tax advice. Cryptocurrency tax laws
            vary by jurisdiction.
          </p>
        </Card>
      </main>
    </div>
  )
}
