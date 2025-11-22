import { Metadata } from 'next';
import { Check, X } from 'lucide-react';

export const metadata: Metadata = {
  title: 'Pricing | Ëtrid Mobile Wallet',
  description: 'Free for basic features. Premium tiers for advanced trading, business accounts, and white-label solutions.',
};

const pricingTiers = [
  {
    name: 'Free',
    price: '$0',
    period: 'forever',
    description: 'Perfect for individuals getting started',
    features: [
      { name: 'Wallet & storage', included: true },
      { name: 'Fiat on/off ramp', included: true },
      { name: 'Basic trading (0.3% fee)', included: true },
      { name: 'NFT marketplace', included: true },
      { name: 'Social features', included: true },
      { name: 'dApp browser', included: true },
      { name: 'AU Bloccard', included: false },
      { name: 'Advanced trading', included: false },
      { name: 'Lending & borrowing', included: false },
      { name: 'Priority support', included: false },
    ],
    cta: 'Get Started',
    highlighted: false,
  },
  {
    name: 'Pro',
    price: '$19',
    period: 'per month',
    description: 'For active traders and DeFi users',
    features: [
      { name: 'Everything in Free', included: true },
      { name: 'AU Bloccard (crypto debit card)', included: true },
      { name: 'Advanced trading (0.1% fee)', included: true },
      { name: 'Lending & borrowing', included: true },
      { name: 'DCA bots', included: true },
      { name: 'Portfolio analytics', included: true },
      { name: 'Priority support', included: true },
      { name: 'Multi-sig wallets (up to 5)', included: true },
      { name: 'Business accounts', included: false },
      { name: 'White-label', included: false },
    ],
    cta: 'Start Free Trial',
    highlighted: true,
  },
  {
    name: 'Business',
    price: '$199',
    period: 'per month',
    description: 'For teams and businesses',
    features: [
      { name: 'Everything in Pro', included: true },
      { name: 'Business accounts', included: true },
      { name: 'Team management (up to 10 users)', included: true },
      { name: 'Merchant tools & POS', included: true },
      { name: 'Invoicing & payroll', included: true },
      { name: 'Advanced analytics', included: true },
      { name: 'Dedicated account manager', included: true },
      { name: 'Custom integrations', included: true },
      { name: 'API access', included: true },
      { name: 'White-label (add-on)', included: false },
    ],
    cta: 'Contact Sales',
    highlighted: false,
  },
];

export default function PricingPage() {
  return (
    <main className="pt-16">
      {/* Hero Section */}
      <section className="py-24 bg-gradient-to-b from-[#1a0033] via-[#2d0055] to-[#4a0080]">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h1 className="text-5xl md:text-7xl font-bold text-white mb-6">
            Simple, Transparent
            <span className="block text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-blue-400 mt-2">
              Pricing
            </span>
          </h1>
          <p className="text-xl text-gray-300 max-w-3xl mx-auto">
            Start free, upgrade when you need more. No hidden fees, cancel anytime.
          </p>
        </div>
      </section>

      {/* Pricing Cards */}
      <section className="py-24 bg-[#1a0033]">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid md:grid-cols-3 gap-8">
            {pricingTiers.map((tier) => (
              <div
                key={tier.name}
                className={`relative bg-white/5 backdrop-blur-sm rounded-3xl p-8 border ${
                  tier.highlighted
                    ? 'border-purple-500 scale-105 shadow-2xl shadow-purple-500/20'
                    : 'border-white/10'
                }`}
              >
                {tier.highlighted && (
                  <div className="absolute -top-4 left-1/2 -translate-x-1/2 px-4 py-1 bg-gradient-to-r from-purple-500 to-blue-500 text-white text-sm font-semibold rounded-full">
                    Most Popular
                  </div>
                )}

                <div className="text-center mb-8">
                  <h3 className="text-2xl font-bold text-white mb-2">{tier.name}</h3>
                  <div className="mb-4">
                    <span className="text-5xl font-bold text-white">{tier.price}</span>
                    <span className="text-gray-400 ml-2">/{tier.period}</span>
                  </div>
                  <p className="text-gray-300">{tier.description}</p>
                </div>

                <ul className="space-y-4 mb-8">
                  {tier.features.map((feature) => (
                    <li key={feature.name} className="flex items-start gap-3">
                      {feature.included ? (
                        <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                      ) : (
                        <X className="w-5 h-5 text-gray-600 mt-0.5 flex-shrink-0" />
                      )}
                      <span className={feature.included ? 'text-gray-300' : 'text-gray-600'}>
                        {feature.name}
                      </span>
                    </li>
                  ))}
                </ul>

                <a
                  href={tier.name === 'Business' ? '/contact' : '/download'}
                  className={`block w-full py-4 px-6 rounded-xl font-semibold text-center transition-all ${
                    tier.highlighted
                      ? 'bg-gradient-to-r from-purple-500 to-blue-500 text-white hover:scale-105'
                      : 'bg-white/10 text-white hover:bg-white/20'
                  }`}
                >
                  {tier.cta}
                </a>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Fee Breakdown */}
      <section className="py-24 bg-gradient-to-b from-[#1a0033] to-[#4a0080]">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-4xl font-bold text-white text-center mb-12">
            Fee Breakdown
          </h2>
          <div className="bg-white/5 backdrop-blur-sm rounded-2xl p-8 border border-white/10">
            <div className="space-y-6">
              <div className="flex justify-between items-center">
                <span className="text-gray-300">Trading (Free tier)</span>
                <span className="text-white font-semibold">0.3%</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-300">Trading (Pro tier)</span>
                <span className="text-white font-semibold">0.1%</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-300">Fiat on-ramp</span>
                <span className="text-white font-semibold">1.5%</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-300">Crypto transfers</span>
                <span className="text-white font-semibold">Network fees only</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-300">AU Bloccard transactions</span>
                <span className="text-white font-semibold">Free</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-300">NFT marketplace</span>
                <span className="text-white font-semibold">2.5%</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* FAQ */}
      <section className="py-24 bg-[#1a0033]">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-4xl font-bold text-white text-center mb-12">
            Frequently Asked Questions
          </h2>
          <div className="space-y-6">
            {[
              {
                question: 'Can I switch plans anytime?',
                answer: 'Yes! You can upgrade or downgrade your plan at any time. Changes take effect immediately.',
              },
              {
                question: 'Is there a free trial?',
                answer: 'Yes, Pro and Business plans come with a 14-day free trial. No credit card required.',
              },
              {
                question: 'What payment methods do you accept?',
                answer: 'We accept credit cards, bank transfers, and crypto payments.',
              },
              {
                question: 'Do you offer refunds?',
                answer: 'Yes, we offer a 30-day money-back guarantee on all paid plans.',
              },
            ].map((faq) => (
              <div key={faq.question} className="bg-white/5 backdrop-blur-sm rounded-xl p-6 border border-white/10">
                <h3 className="text-xl font-semibold text-white mb-3">{faq.question}</h3>
                <p className="text-gray-300">{faq.answer}</p>
              </div>
            ))}
          </div>
        </div>
      </section>
    </main>
  );
}
