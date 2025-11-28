export default function Stats() {
  return (
    <section className="py-24 bg-[#1a0033]">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
          <StatCard
            number="264"
            label="Files Created"
            subtitle="Production-ready code"
          />
          <StatCard
            number="52,692"
            label="Lines of Code"
            subtitle="TypeScript & React"
          />
          <StatCard
            number="$23-25M"
            label="ARR Potential"
            subtitle="At 100K users"
          />
          <StatCard
            number="60x"
            label="Faster Build"
            subtitle="Using parallel agents"
          />
        </div>
      </div>
    </section>
  );
}

function StatCard({ number, label, subtitle }: { number: string; label: string; subtitle: string }) {
  return (
    <div className="text-center">
      <div className="text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-blue-400 mb-2">
        {number}
      </div>
      <div className="text-xl font-semibold text-white mb-1">
        {label}
      </div>
      <div className="text-sm text-gray-400">
        {subtitle}
      </div>
    </div>
  );
}
