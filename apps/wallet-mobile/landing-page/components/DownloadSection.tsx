import { Apple, Chrome, Smartphone } from 'lucide-react';

export default function DownloadSection() {
  return (
    <section className="py-24 bg-gradient-to-b from-[#1a0033] to-[#4a0080]">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <h2 className="text-4xl md:text-5xl font-bold text-white mb-6">
          Get Started Today
        </h2>
        <p className="text-xl text-gray-300 mb-12">
          Available on Web, iOS, and Android
        </p>

        <div className="grid md:grid-cols-3 gap-6">
          <DownloadCard
            icon={<Chrome className="w-8 h-8 text-white" />}
            title="Web App"
            description="Use in any browser"
            buttonText="Launch App"
            href="https://wallet.etrid.com"
            badge="PWA"
          />

          <DownloadCard
            icon={<Apple className="w-8 h-8 text-white" />}
            title="iOS App"
            description="iPhone & iPad"
            buttonText="App Store"
            href="#"
            badge="Coming Soon"
          />

          <DownloadCard
            icon={<Smartphone className="w-8 h-8 text-white" />}
            title="Android App"
            description="All Android devices"
            buttonText="Google Play"
            href="#"
            badge="Coming Soon"
          />
        </div>

        <div className="mt-12 text-sm text-gray-400">
          Supports Chrome, Safari, Firefox, Edge • iOS 13+ • Android 8+
        </div>
      </div>
    </section>
  );
}

interface DownloadCardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  buttonText: string;
  href: string;
  badge?: string;
}

function DownloadCard({ icon, title, description, buttonText, href, badge }: DownloadCardProps) {
  return (
    <div className="bg-white/5 backdrop-blur-sm rounded-2xl p-8 border border-white/10">
      <div className="inline-flex p-4 rounded-2xl bg-purple-500/20 mb-4">
        {icon}
      </div>

      {badge && (
        <div className="inline-block px-3 py-1 rounded-full bg-purple-500/20 text-purple-300 text-xs font-semibold mb-4">
          {badge}
        </div>
      )}

      <h3 className="text-2xl font-bold text-white mb-2">{title}</h3>
      <p className="text-gray-300 mb-6">{description}</p>

      <a
        href={href}
        className="block w-full py-3 px-6 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold hover:scale-105 transition-transform"
      >
        {buttonText}
      </a>
    </div>
  );
}
