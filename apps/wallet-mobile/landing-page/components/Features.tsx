import {
  CreditCard, DollarSign, Users, Image, TrendingUp,
  Coins, Target, Shield, Eye, Briefcase, Store,
  Globe, Building2, BarChart3, Bell, Palette, Gamepad2
} from 'lucide-react';

const features = [
  {
    icon: DollarSign,
    title: 'Fiat On/Off Ramp',
    description: 'Buy crypto with credit card or sell to bank account',
    color: 'from-green-400 to-green-600',
  },
  {
    icon: CreditCard,
    title: 'AU Bloccard',
    description: 'Crypto-secured debit card with 60% LTV spending',
    color: 'from-purple-400 to-purple-600',
  },
  {
    icon: Users,
    title: 'Social Features',
    description: '@username.etrid, contacts, bill splitting, social recovery',
    color: 'from-blue-400 to-blue-600',
  },
  {
    icon: Image,
    title: 'NFT Marketplace',
    description: 'Buy, sell, mint NFTs with auction support',
    color: 'from-pink-400 to-pink-600',
  },
  {
    icon: TrendingUp,
    title: 'Advanced Trading',
    description: 'Professional trading with indicators and DCA bots',
    color: 'from-orange-400 to-orange-600',
  },
  {
    icon: Coins,
    title: 'Lending & Borrowing',
    description: 'Supply to earn 8-15% APY, borrow with collateral',
    color: 'from-yellow-400 to-yellow-600',
  },
  {
    icon: Target,
    title: 'Savings Goals',
    description: 'Gamified savings with 4 auto-save rule types',
    color: 'from-teal-400 to-teal-600',
  },
  {
    icon: Shield,
    title: 'Multi-Sig Wallets',
    description: 'M-of-N approval for shared accounts and DAOs',
    color: 'from-red-400 to-red-600',
  },
  {
    icon: Eye,
    title: 'Privacy Features',
    description: 'Stealth addresses, mixing, Tor routing',
    color: 'from-indigo-400 to-indigo-600',
  },
  {
    icon: Briefcase,
    title: 'Business Accounts',
    description: 'Team management, invoicing, payroll, expenses',
    color: 'from-cyan-400 to-cyan-600',
  },
  {
    icon: Store,
    title: 'Merchant Tools',
    description: 'POS, payment links, product catalog, refunds',
    color: 'from-lime-400 to-lime-600',
  },
  {
    icon: Globe,
    title: 'dApp Browser',
    description: 'Web3 provider with WalletConnect v2',
    color: 'from-violet-400 to-violet-600',
  },
  {
    icon: Building2,
    title: 'DAO Management',
    description: 'Create DAOs, proposals, voting, treasury',
    color: 'from-fuchsia-400 to-fuchsia-600',
  },
  {
    icon: BarChart3,
    title: 'Portfolio Analytics',
    description: 'Risk metrics, tax optimization, performance',
    color: 'from-rose-400 to-rose-600',
  },
  {
    icon: Bell,
    title: 'Smart Notifications',
    description: 'Price, whale, governance, security alerts',
    color: 'from-amber-400 to-amber-600',
  },
  {
    icon: Palette,
    title: 'Themes',
    description: '5 built-in themes + custom creator',
    color: 'from-emerald-400 to-emerald-600',
  },
  {
    icon: Gamepad2,
    title: 'Metaverse',
    description: 'Virtual land, wearables, events integration',
    color: 'from-sky-400 to-sky-600',
  },
];

export default function Features() {
  return (
    <section id="features" className="py-24 bg-gradient-to-b from-[#4a0080] to-[#1a0033]">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold text-white mb-4">
            18 Powerful Features
          </h2>
          <p className="text-xl text-gray-300 max-w-3xl mx-auto">
            Everything you need for crypto finance, all in one beautiful app
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {features.map((feature) => (
            <div
              key={feature.title}
              className="group relative bg-white/5 backdrop-blur-sm rounded-2xl p-6 border border-white/10 hover:border-purple-500/50 transition-all hover:scale-105"
            >
              <div className={`inline-flex p-3 rounded-xl bg-gradient-to-r ${feature.color} mb-4`}>
                <feature.icon className="w-6 h-6 text-white" />
              </div>

              <h3 className="text-xl font-semibold text-white mb-2">
                {feature.title}
              </h3>

              <p className="text-gray-300">
                {feature.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
