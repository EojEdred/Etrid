import { Card } from "@/components/ui/card"
import { Droplets, TrendingUp, Shield } from "lucide-react"

export function InfoCards() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
      <Card className="p-6 bg-card/30 backdrop-blur-xl border-border/50">
        <div className="flex items-start gap-4">
          <div className="p-3 rounded-xl bg-accent/10">
            <Droplets className="w-6 h-6 text-accent" />
          </div>
          <div className="flex-1">
            <h4 className="font-semibold mb-2">Liquidity Pool</h4>
            <div className="space-y-1 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">ÉTR in pool:</span>
                <span className="font-medium">10M ÉTR</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">EDSC in pool:</span>
                <span className="font-medium">80M EDSC</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Your share:</span>
                <span className="font-medium text-accent">0.5%</span>
              </div>
            </div>
          </div>
        </div>
      </Card>

      <Card className="p-6 bg-card/30 backdrop-blur-xl border-border/50">
        <div className="flex items-start gap-4">
          <div className="p-3 rounded-xl bg-success/10">
            <TrendingUp className="w-6 h-6 text-success" />
          </div>
          <div className="flex-1">
            <h4 className="font-semibold mb-2">24H Volume</h4>
            <div className="space-y-1 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">ÉTR traded:</span>
                <span className="font-medium">2.4M ÉTR</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">USD value:</span>
                <span className="font-medium">$19.2M</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Transactions:</span>
                <span className="font-medium text-success">1,234</span>
              </div>
            </div>
          </div>
        </div>
      </Card>

      <Card className="p-6 bg-card/30 backdrop-blur-xl border-border/50">
        <div className="flex items-start gap-4">
          <div className="p-3 rounded-xl bg-primary/10">
            <Shield className="w-6 h-6 text-primary" />
          </div>
          <div className="flex-1">
            <h4 className="font-semibold mb-2">EDSC Peg</h4>
            <div className="space-y-1 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Current:</span>
                <span className="font-medium">$1.001</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">24H Range:</span>
                <span className="font-medium">$0.998 - $1.003</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Health:</span>
                <span className="font-medium text-success">Healthy ✅</span>
              </div>
            </div>
          </div>
        </div>
      </Card>
    </div>
  )
}
