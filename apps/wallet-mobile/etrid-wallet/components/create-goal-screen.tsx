/**
 * CreateGoalScreen - Multi-step goal creation
 */

'use client'

import { useState } from 'react'
import { ArrowLeft, ChevronRight } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { useSavingsGoals } from '@/hooks/use-savings-goals'

interface CreateGoalScreenProps {
  onBack: () => void
}

const categories = [
  { name: 'Emergency Fund', icon: '🛡️' },
  { name: 'Vacation', icon: '✈️' },
  { name: 'House', icon: '🏠' },
  { name: 'Car', icon: '🚗' },
  { name: 'Education', icon: '🎓' },
  { name: 'Wedding', icon: '💒' },
  { name: 'Retirement', icon: '🏖️' },
  { name: 'Electronics', icon: '💻' },
  { name: 'Other', icon: '🎯' },
]

export function CreateGoalScreen({ onBack }: CreateGoalScreenProps) {
  const [step, setStep] = useState(1)
  const [goalData, setGoalData] = useState({
    name: '',
    category: 'Emergency Fund',
    icon: '🛡️',
    targetAmount: '',
    targetDate: '',
    initialContribution: '',
  })

  const { createGoal, loading } = useSavingsGoals()

  const handleCreate = async () => {
    try {
      await createGoal({
        name: goalData.name,
        category: goalData.category,
        icon: goalData.icon,
        targetAmount: parseFloat(goalData.targetAmount),
        targetDate: goalData.targetDate ? new Date(goalData.targetDate).getTime() : undefined,
        initialContribution: goalData.initialContribution ? parseFloat(goalData.initialContribution) : 0,
      })
      onBack()
    } catch (err) {
      console.error(err)
    }
  }

  const canProceed = () => {
    switch (step) {
      case 1:
        return goalData.name.length > 0 && goalData.category.length > 0
      case 2:
        return parseFloat(goalData.targetAmount) > 0
      case 3:
        return true
      case 4:
        return true
      default:
        return false
    }
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">Create Goal</h1>
            <p className="text-sm text-muted-foreground">Step {step} of 4</p>
          </div>
        </div>

        {/* Progress */}
        <div className="flex gap-2">
          {[1, 2, 3, 4].map((s) => (
            <div
              key={s}
              className={`h-1 flex-1 rounded-full ${
                s <= step ? 'bg-accent' : 'bg-muted'
              }`}
            />
          ))}
        </div>
      </header>

      <main className="px-6 space-y-6">
        {step === 1 && (
          <>
            <div className="space-y-3">
              <Label>Goal Name</Label>
              <Input
                placeholder="e.g., Dream Vacation"
                value={goalData.name}
                onChange={(e) => setGoalData({ ...goalData, name: e.target.value })}
                className="glass border-border h-14 text-lg"
              />
            </div>

            <div className="space-y-3">
              <Label>Category</Label>
              <Select
                value={goalData.category}
                onValueChange={(value) => {
                  const cat = categories.find(c => c.name === value)
                  setGoalData({
                    ...goalData,
                    category: value,
                    icon: cat?.icon || '🎯',
                  })
                }}
              >
                <SelectTrigger className="glass border-border h-14">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {categories.map((cat) => (
                    <SelectItem key={cat.name} value={cat.name}>
                      <span className="flex items-center gap-2">
                        <span>{cat.icon}</span>
                        <span>{cat.name}</span>
                      </span>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="glass-strong rounded-2xl p-6 text-center">
              <div className="text-6xl mb-2">{goalData.icon}</div>
              <p className="font-semibold">{goalData.name || 'Your Goal'}</p>
              <p className="text-sm text-muted-foreground">{goalData.category}</p>
            </div>
          </>
        )}

        {step === 2 && (
          <>
            <div className="space-y-3">
              <Label>Target Amount (ÉTR)</Label>
              <div className="glass rounded-2xl p-4">
                <Input
                  type="number"
                  placeholder="0.00"
                  value={goalData.targetAmount}
                  onChange={(e) => setGoalData({ ...goalData, targetAmount: e.target.value })}
                  className="text-3xl font-bold bg-transparent border-0 p-0 h-auto focus-visible:ring-0 text-center"
                />
                <p className="text-center text-sm text-muted-foreground mt-2">
                  ≈ ${(parseFloat(goalData.targetAmount || '0') * 8).toLocaleString()}
                </p>
              </div>
            </div>

            <div className="grid grid-cols-3 gap-3">
              {[1000, 5000, 10000].map((amount) => (
                <Button
                  key={amount}
                  variant="outline"
                  onClick={() => setGoalData({ ...goalData, targetAmount: amount.toString() })}
                  className="glass border-border"
                >
                  {amount.toLocaleString()}
                </Button>
              ))}
            </div>
          </>
        )}

        {step === 3 && (
          <>
            <div className="space-y-3">
              <Label>Target Date (Optional)</Label>
              <Input
                type="date"
                value={goalData.targetDate}
                onChange={(e) => setGoalData({ ...goalData, targetDate: e.target.value })}
                className="glass border-border h-14"
              />
              <p className="text-xs text-muted-foreground">
                Leave blank if you don't have a specific deadline
              </p>
            </div>

            {goalData.targetDate && (
              <div className="glass-strong rounded-xl p-4">
                <p className="text-sm text-muted-foreground mb-2">Time to Goal</p>
                <p className="text-2xl font-bold">
                  {Math.ceil(
                    (new Date(goalData.targetDate).getTime() - Date.now()) /
                      (1000 * 60 * 60 * 24)
                  )}{' '}
                  days
                </p>
              </div>
            )}
          </>
        )}

        {step === 4 && (
          <>
            <div className="space-y-3">
              <Label>Initial Contribution (Optional)</Label>
              <div className="glass rounded-2xl p-4">
                <Input
                  type="number"
                  placeholder="0.00"
                  value={goalData.initialContribution}
                  onChange={(e) => setGoalData({ ...goalData, initialContribution: e.target.value })}
                  className="text-2xl font-bold bg-transparent border-0 p-0 h-auto focus-visible:ring-0"
                />
              </div>
            </div>

            <div className="glass-strong rounded-2xl p-6 space-y-4">
              <h3 className="font-semibold text-center">Goal Summary</h3>
              <div className="text-center">
                <div className="text-4xl mb-2">{goalData.icon}</div>
                <p className="font-semibold text-lg">{goalData.name}</p>
                <p className="text-sm text-muted-foreground">{goalData.category}</p>
              </div>
              <div className="space-y-2 text-sm">
                <div className="flex items-center justify-between">
                  <span className="text-muted-foreground">Target</span>
                  <span className="font-semibold">{goalData.targetAmount} ÉTR</span>
                </div>
                {goalData.targetDate && (
                  <div className="flex items-center justify-between">
                    <span className="text-muted-foreground">Due Date</span>
                    <span className="font-semibold">
                      {new Date(goalData.targetDate).toLocaleDateString()}
                    </span>
                  </div>
                )}
                {goalData.initialContribution && (
                  <div className="flex items-center justify-between">
                    <span className="text-muted-foreground">Starting With</span>
                    <span className="font-semibold">{goalData.initialContribution} ÉTR</span>
                  </div>
                )}
              </div>
            </div>
          </>
        )}

        <div className="flex gap-3">
          {step > 1 && (
            <Button variant="outline" onClick={() => setStep(step - 1)} className="flex-1">
              Back
            </Button>
          )}
          <Button
            onClick={() => {
              if (step < 4) {
                setStep(step + 1)
              } else {
                handleCreate()
              }
            }}
            disabled={!canProceed() || (step === 4 && loading)}
            className="flex-1"
            style={{ background: '#00d9ff', color: '#000' }}
          >
            {step < 4 ? (
              <>
                Next <ChevronRight className="w-4 h-4 ml-2" />
              </>
            ) : loading ? (
              'Creating...'
            ) : (
              'Create Goal'
            )}
          </Button>
        </div>
      </main>
    </div>
  )
}
