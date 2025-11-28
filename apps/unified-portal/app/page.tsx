import { StatCard } from "@/components/dashboard/stat-card"
import { ServiceCard } from "@/components/dashboard/service-card"
import {
  Shield,
  Eye,
  Wallet,
  Vote,
  Activity,
  Coins,
  Zap,
  Users,
  Box,
  TrendingUp,
} from "lucide-react"

export default function DashboardPage() {
  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent mb-2">
          Etrid Control Center
        </h1>
        <p className="text-zinc-600 dark:text-zinc-400">
          Unified dashboard for all Etrid Protocol operations
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <StatCard
          title="Active Validators"
          value="21"
          status="healthy"
          icon={Shield}
        />
        <StatCard
          title="Current Block"
          value="#12,450"
          status="syncing"
          icon={Box}
        />
        <StatCard
          title="Network Peers"
          value="21"
          status="healthy"
          icon={Users}
        />
        <StatCard
          title="Network TPS"
          value="1,250"
          status="healthy"
          icon={TrendingUp}
        />
      </div>

      <div>
        <h2 className="text-2xl font-bold mb-6">Services</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <ServiceCard
            name="Lightning Network"
            href="/lightning"
            description="Manage Lightning channels and payments"
            icon={Zap}
          />
          <ServiceCard
            name="Validator Dashboard"
            href="/validator"
            description="Monitor validator performance and rewards"
            icon={Shield}
          />
          <ServiceCard
            name="Watchtower Monitor"
            href="/watchtower"
            description="Channel monitoring and fraud detection"
            icon={Eye}
          />
          <ServiceCard
            name="Wallet & DeFi"
            href="/wallet"
            description="Swap, stake, and manage assets"
            icon={Wallet}
          />
          <ServiceCard
            name="Governance"
            href="/governance"
            description="Participate in protocol governance"
            icon={Vote}
          />
          <ServiceCard
            name="Network Monitoring"
            href="/monitoring"
            description="Real-time network telemetry and metrics"
            icon={Activity}
          />
          <ServiceCard
            name="MasterChef"
            href="/masterchef"
            description="Liquidity pool rewards and farming"
            icon={Coins}
          />
        </div>
      </div>
    </div>
  )
}
