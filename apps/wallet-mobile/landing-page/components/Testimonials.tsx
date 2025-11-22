import { Star, Quote } from 'lucide-react';

const testimonials = [
  {
    name: 'Sarah Chen',
    role: 'Crypto Trader',
    avatar: '👩‍💻',
    rating: 5,
    text: 'The advanced trading features and low fees make this my go-to wallet. The AU Bloccard is a game changer!',
  },
  {
    name: 'Michael Rodriguez',
    role: 'Business Owner',
    avatar: '👨‍💼',
    rating: 5,
    text: 'Perfect for my business. The merchant tools and invoicing features save me hours every week.',
  },
  {
    name: 'Emma Thompson',
    role: 'NFT Collector',
    avatar: '👩‍🎨',
    rating: 5,
    text: 'The NFT marketplace is beautiful and easy to use. I love having everything in one app.',
  },
  {
    name: 'David Kim',
    role: 'DeFi Enthusiast',
    avatar: '👨‍💻',
    rating: 5,
    text: 'Lending and borrowing features are incredible. Earning 15% APY on my stablecoins!',
  },
  {
    name: 'Lisa Park',
    role: 'Investor',
    avatar: '👩‍💼',
    rating: 5,
    text: 'Portfolio analytics help me make better investment decisions. The interface is stunning.',
  },
  {
    name: 'James Wilson',
    role: 'DAO Member',
    avatar: '👨‍🔬',
    rating: 5,
    text: 'DAO management tools are top-notch. Multi-sig wallets provide peace of mind.',
  },
];

export default function Testimonials() {
  return (
    <section className="py-24 bg-gradient-to-b from-[#4a0080] to-[#1a0033]">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold text-white mb-4">
            Loved by Thousands
          </h2>
          <p className="text-xl text-gray-300 max-w-3xl mx-auto">
            See what our users are saying about Ëtrid Wallet
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {testimonials.map((testimonial) => (
            <div
              key={testimonial.name}
              className="bg-white/5 backdrop-blur-sm rounded-2xl p-6 border border-white/10 hover:border-purple-500/50 transition-all"
            >
              <Quote className="w-8 h-8 text-purple-400 mb-4" />

              <p className="text-gray-300 mb-6 italic">
                &quot;{testimonial.text}&quot;
              </p>

              <div className="flex items-center gap-4">
                <div className="text-4xl">{testimonial.avatar}</div>
                <div className="flex-1">
                  <div className="font-semibold text-white">
                    {testimonial.name}
                  </div>
                  <div className="text-sm text-gray-400">
                    {testimonial.role}
                  </div>
                </div>
              </div>

              <div className="flex gap-1 mt-4">
                {Array.from({ length: testimonial.rating }).map((_, i) => (
                  <Star key={i} className="w-5 h-5 fill-yellow-400 text-yellow-400" />
                ))}
              </div>
            </div>
          ))}
        </div>

        <div className="text-center mt-12">
          <div className="inline-flex items-center gap-2 text-gray-300">
            <Star className="w-5 h-5 fill-yellow-400 text-yellow-400" />
            <span className="text-2xl font-bold text-white">4.8/5</span>
            <span>from 10,000+ reviews</span>
          </div>
        </div>
      </div>
    </section>
  );
}
