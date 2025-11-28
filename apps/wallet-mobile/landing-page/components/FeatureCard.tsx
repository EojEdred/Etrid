import { LucideIcon } from 'lucide-react';

interface FeatureCardProps {
  icon: LucideIcon;
  title: string;
  description: string;
  color?: string;
  className?: string;
}

export default function FeatureCard({
  icon: Icon,
  title,
  description,
  color = 'from-purple-400 to-purple-600',
  className = '',
}: FeatureCardProps) {
  return (
    <div
      className={`group relative bg-white/5 backdrop-blur-sm rounded-2xl p-6 border border-white/10 hover:border-purple-500/50 transition-all hover:scale-105 ${className}`}
    >
      <div className={`inline-flex p-3 rounded-xl bg-gradient-to-r ${color} mb-4`}>
        <Icon className="w-6 h-6 text-white" />
      </div>

      <h3 className="text-xl font-semibold text-white mb-2">
        {title}
      </h3>

      <p className="text-gray-300">
        {description}
      </p>
    </div>
  );
}
