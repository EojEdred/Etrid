"use client"

import { ArrowLeft, QrCode, Link as LinkIcon, TrendingUp, TrendingDown } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useMerchant } from "@/hooks/merchant/use-merchant"
import { SalesChart } from "@/components/merchant/SalesChart"
import { format } from "date-fns"

interface MerchantDashboardScreenProps {
  onBack: () => void
  onNewSale: () => void
  onPaymentLink: () => void
  onNavigate: (screen: string) => void
}

export function MerchantDashboardScreen({
  onBack,
  onNewSale,
  onPaymentLink,
  onNavigate,
}: MerchantDashboardScreenProps) {
  const { account, stats, recentSales, loading } = useMerchant()

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <p className="text-muted-foreground">Loading...</p>
      </div>
    )
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="glass-strong border-b border-border sticky top-0 z-10">
        <div className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <div>
              <h1 className="text-xl font-bold text-foreground">Merchant Dashboard</h1>
              {account && (
                <p className="text-sm text-muted-foreground">{account.business_name}</p>
              )}
            </div>
          </div>
        </div>
      </header>

      <main className="px-4 py-6 space-y-6">
        {/* Stats Overview */}
        <div className="grid grid-cols-2 gap-4">
          <div className="glass-strong rounded-lg p-4 border border-border">
            <p className="text-sm text-muted-foreground mb-2">Sales Today</p>
            <p className="text-2xl font-bold text-foreground">
              ${stats?.total_sales.toFixed(2) || "0.00"}
            </p>
            <div className="flex items-center gap-1 mt-1">
              {(stats?.sales_change || 0) >= 0 ? (
                <>
                  <TrendingUp className="w-3 h-3 text-green-400" />
                  <span className="text-xs text-green-400">
                    +{stats?.sales_change.toFixed(1)}%
                  </span>
                </>
              ) : (
                <>
                  <TrendingDown className="w-3 h-3 text-red-400" />
                  <span className="text-xs text-red-400">
                    {stats?.sales_change.toFixed(1)}%
                  </span>
                </>
              )}
            </div>
          </div>

          <div className="glass-strong rounded-lg p-4 border border-border">
            <p className="text-sm text-muted-foreground mb-2">Transactions</p>
            <p className="text-2xl font-bold text-foreground">
              {stats?.transaction_count || 0}
            </p>
            <div className="flex items-center gap-1 mt-1">
              {(stats?.transaction_change || 0) >= 0 ? (
                <>
                  <TrendingUp className="w-3 h-3 text-green-400" />
                  <span className="text-xs text-green-400">
                    +{stats?.transaction_change.toFixed(1)}%
                  </span>
                </>
              ) : (
                <>
                  <TrendingDown className="w-3 h-3 text-red-400" />
                  <span className="text-xs text-red-400">
                    {stats?.transaction_change.toFixed(1)}%
                  </span>
                </>
              )}
            </div>
          </div>
        </div>

        {/* Quick Actions */}
        <div>
          <h3 className="text-lg font-semibold text-foreground mb-3">Quick Actions</h3>
          <div className="grid grid-cols-2 gap-3">
            <Button
              variant="default"
              className="h-20 flex-col gap-2"
              onClick={onNewSale}
            >
              <QrCode className="w-6 h-6" />
              <span>New Sale</span>
            </Button>

            <Button
              variant="outline"
              className="h-20 flex-col gap-2"
              onClick={onPaymentLink}
            >
              <LinkIcon className="w-6 h-6" />
              <span>Payment Link</span>
            </Button>
          </div>
        </div>

        {/* Sales Chart */}
        {stats && <SalesChart stats={stats} />}

        {/* Top Products */}
        {stats && stats.top_products.length > 0 && (
          <div className="glass-strong rounded-lg p-4 border border-border">
            <h3 className="text-lg font-semibold text-foreground mb-4">
              Top Products
            </h3>
            <div className="space-y-3">
              {stats.top_products.map((product, index) => (
                <div key={product.product_id} className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="w-6 h-6 rounded-full bg-accent/20 flex items-center justify-center">
                      <span className="text-xs font-bold text-accent">{index + 1}</span>
                    </div>
                    <div>
                      <p className="text-sm font-medium text-foreground">
                        {product.product_name}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        {product.quantity_sold} sold
                      </p>
                    </div>
                  </div>
                  <p className="text-sm font-semibold text-foreground">
                    ${product.revenue.toFixed(2)}
                  </p>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Recent Sales */}
        <div>
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-lg font-semibold text-foreground">Recent Sales</h3>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => onNavigate("sales")}
            >
              View All
            </Button>
          </div>

          <div className="space-y-2">
            {recentSales.slice(0, 5).map((sale) => (
              <div
                key={sale.id}
                className="glass-strong rounded-lg p-3 border border-border flex items-center justify-between"
              >
                <div>
                  <p className="text-sm font-medium text-foreground">
                    Sale #{sale.sale_number}
                  </p>
                  <p className="text-xs text-muted-foreground">
                    {format(new Date(sale.created_at), "MMM d, h:mm a")}
                  </p>
                </div>
                <p className="text-sm font-semibold text-foreground">
                  ${sale.total.toFixed(2)}
                </p>
              </div>
            ))}
          </div>
        </div>
      </main>
    </div>
  )
}
