"use client"

import { ArrowLeft, Plus, Link as LinkIcon, Copy } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { usePaymentLinks } from "@/hooks/merchant/use-payment-links"
import { PaymentLinkCard } from "@/components/merchant/PaymentLinkCard"
import type { PaymentLinkStatus } from "@/lib/types/merchant"
import { useState } from "react"

interface PaymentLinksScreenProps {
  onBack: () => void
  onCreate: () => void
}

export function PaymentLinksScreen({ onBack, onCreate }: PaymentLinksScreenProps) {
  const [filter, setFilter] = useState<PaymentLinkStatus | undefined>(undefined)
  const { links, loading, deactivateLink, deleteLink, getPaymentUrl } =
    usePaymentLinks(filter)

  const handleCopy = async (link: any) => {
    const url = getPaymentUrl(link.link_code)
    try {
      await navigator.clipboard.writeText(url)
      alert("Link copied to clipboard!")
    } catch (error) {
      console.error("Failed to copy link:", error)
    }
  }

  const handleShare = async (link: any) => {
    const url = getPaymentUrl(link.link_code)
    if (navigator.share) {
      try {
        await navigator.share({
          title: link.description,
          text: `Pay via this link: ${link.description}`,
          url,
        })
      } catch (error) {
        console.error("Failed to share:", error)
      }
    } else {
      handleCopy(link)
    }
  }

  const handleDeactivate = async (link: any) => {
    if (confirm("Deactivate this payment link?")) {
      try {
        await deactivateLink(link.id)
      } catch (error) {
        console.error("Failed to deactivate link:", error)
      }
    }
  }

  const handleDelete = async (link: any) => {
    if (confirm("Delete this payment link?")) {
      try {
        await deleteLink(link.id)
      } catch (error) {
        console.error("Failed to delete link:", error)
      }
    }
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="glass-strong border-b border-border sticky top-0 z-10">
        <div className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <h1 className="text-xl font-bold text-foreground">Payment Links</h1>
          </div>
          <Button size="sm" onClick={onCreate}>
            <Plus className="w-4 h-4" />
            Create
          </Button>
        </div>
      </header>

      <main className="px-4 py-6">
        <Tabs
          value={filter || "all"}
          onValueChange={(val) =>
            setFilter(val === "all" ? undefined : (val as PaymentLinkStatus))
          }
        >
          <TabsList className="w-full grid grid-cols-4 mb-6">
            <TabsTrigger value="all">All</TabsTrigger>
            <TabsTrigger value="active">Active</TabsTrigger>
            <TabsTrigger value="paid">Paid</TabsTrigger>
            <TabsTrigger value="expired">Expired</TabsTrigger>
          </TabsList>

          <div className="space-y-4">
            {loading ? (
              <p className="text-center text-muted-foreground">Loading...</p>
            ) : links.length === 0 ? (
              <div className="glass-strong rounded-lg p-8 border border-border text-center">
                <LinkIcon className="w-12 h-12 text-muted-foreground mx-auto mb-3" />
                <p className="text-foreground font-medium mb-1">
                  No payment links found
                </p>
                <p className="text-sm text-muted-foreground mb-4">
                  Create a payment link to accept payments
                </p>
                <Button onClick={onCreate}>
                  <Plus className="w-4 h-4" />
                  Create Link
                </Button>
              </div>
            ) : (
              links.map((link) => (
                <PaymentLinkCard
                  key={link.id}
                  link={link}
                  onCopy={handleCopy}
                  onShare={handleShare}
                  onDeactivate={handleDeactivate}
                  onDelete={handleDelete}
                  getUrl={getPaymentUrl}
                />
              ))
            )}
          </div>
        </Tabs>
      </main>
    </div>
  )
}
