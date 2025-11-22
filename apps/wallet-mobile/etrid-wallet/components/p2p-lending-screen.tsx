/**
 * P2PLendingScreen - Peer-to-peer lending marketplace
 */

'use client'

import { useState } from 'react'
import { Users, Filter } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { BottomNav } from '@/components/bottom-nav'
import { LoanOffer } from '@/components/lending/loan-offer'
import { useP2PLending } from '@/hooks/use-p2p-lending'

interface P2PLendingScreenProps {
  onBack: () => void
  activeTab: string
  onTabChange: (tab: string) => void
}

export function P2PLendingScreen({ onBack, activeTab, onTabChange }: P2PLendingScreenProps) {
  const { offers, myOffers, myLoans, loading, acceptOffer } = useP2PLending()

  return (
    <div className="min-h-screen pb-24">
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="w-12 h-12 rounded-full bg-primary/20 flex items-center justify-center">
            <Users className="w-6 h-6 text-primary" />
          </div>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">P2P Lending</h1>
            <p className="text-sm text-muted-foreground">Peer-to-peer loan marketplace</p>
          </div>
          <Button variant="ghost" size="icon">
            <Filter className="w-5 h-5" />
          </Button>
        </div>
      </header>

      <main className="px-6 space-y-6">
        <Tabs defaultValue="offers" className="w-full">
          <TabsList className="grid w-full grid-cols-3 glass">
            <TabsTrigger value="offers">Offers</TabsTrigger>
            <TabsTrigger value="my-offers">My Offers</TabsTrigger>
            <TabsTrigger value="my-loans">My Loans</TabsTrigger>
          </TabsList>

          <TabsContent value="offers" className="space-y-4 mt-6">
            {offers.map((offer) => (
              <LoanOffer
                key={offer.id}
                offer={offer}
                onAccept={() => {
                  // In real app, show collateral selection dialog
                  acceptOffer(offer.id, [
                    { asset: 'ÉTR', amount: (offer.amount * offer.minCollateralRatio) / 100 },
                  ])
                }}
              />
            ))}
          </TabsContent>

          <TabsContent value="my-offers" className="space-y-4 mt-6">
            {myOffers.length === 0 ? (
              <div className="glass rounded-2xl p-8 text-center">
                <p className="text-muted-foreground">No active offers</p>
              </div>
            ) : (
              myOffers.map((offer) => (
                <LoanOffer key={offer.id} offer={offer} />
              ))
            )}
          </TabsContent>

          <TabsContent value="my-loans" className="space-y-4 mt-6">
            <div className="space-y-4">
              {myLoans.asLender.length === 0 && myLoans.asBorrower.length === 0 ? (
                <div className="glass rounded-2xl p-8 text-center">
                  <p className="text-muted-foreground">No active loans</p>
                </div>
              ) : (
                <>
                  {myLoans.asLender.length > 0 && (
                    <div>
                      <h3 className="font-semibold mb-3">As Lender</h3>
                      {myLoans.asLender.map((loan) => (
                        <div key={loan.id} className="glass rounded-2xl p-5 mb-3">
                          <div className="flex items-center justify-between mb-3">
                            <div>
                              <p className="font-semibold">{loan.amount} {loan.asset}</p>
                              <p className="text-sm text-muted-foreground">
                                {loan.apy}% APY • {loan.duration} days
                              </p>
                            </div>
                            <span className="text-xs px-2 py-1 rounded-full bg-success/20 text-success capitalize">
                              {loan.status}
                            </span>
                          </div>
                          <div className="glass-strong rounded-xl p-3 text-sm">
                            <div className="flex items-center justify-between">
                              <span className="text-muted-foreground">Interest Earned</span>
                              <span className="font-semibold text-success">
                                +{loan.interestAccrued.toFixed(4)} {loan.asset}
                              </span>
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}

                  {myLoans.asBorrower.length > 0 && (
                    <div>
                      <h3 className="font-semibold mb-3">As Borrower</h3>
                      {myLoans.asBorrower.map((loan) => (
                        <div key={loan.id} className="glass rounded-2xl p-5 mb-3">
                          <div className="flex items-center justify-between mb-3">
                            <div>
                              <p className="font-semibold">{loan.amount} {loan.asset}</p>
                              <p className="text-sm text-muted-foreground">
                                {loan.apy}% APY • {loan.duration} days
                              </p>
                            </div>
                            <span className="text-xs px-2 py-1 rounded-full bg-warning/20 text-warning capitalize">
                              {loan.status}
                            </span>
                          </div>
                          <div className="glass-strong rounded-xl p-3 space-y-2 text-sm">
                            <div className="flex items-center justify-between">
                              <span className="text-muted-foreground">Owed</span>
                              <span className="font-semibold text-destructive">
                                {(loan.amount + loan.interestAccrued - loan.amountRepaid).toFixed(4)} {loan.asset}
                              </span>
                            </div>
                            <div className="flex items-center justify-between">
                              <span className="text-muted-foreground">Due Date</span>
                              <span>{new Date(loan.dueDate).toLocaleDateString()}</span>
                            </div>
                          </div>
                          <Button className="w-full mt-3" variant="outline">
                            Repay Loan
                          </Button>
                        </div>
                      ))}
                    </div>
                  )}
                </>
              )}
            </div>
          </TabsContent>
        </Tabs>
      </main>

      <BottomNav activeTab={activeTab} onTabChange={onTabChange} />
    </div>
  )
}
