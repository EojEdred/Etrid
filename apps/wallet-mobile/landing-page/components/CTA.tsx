import { ArrowRight, Download } from 'lucide-react';
import Link from 'next/link';

interface CTAProps {
  title?: string;
  description?: string;
  primaryButtonText?: string;
  primaryButtonHref?: string;
  secondaryButtonText?: string;
  secondaryButtonHref?: string;
  className?: string;
}

export default function CTA({
  title = 'Ready to Get Started?',
  description = 'Join thousands of users managing their crypto with Ëtrid Wallet.',
  primaryButtonText = 'Get Started Free',
  primaryButtonHref = '/download',
  secondaryButtonText = 'View Features',
  secondaryButtonHref = '/features',
  className = '',
}: CTAProps) {
  return (
    <section className={`py-24 bg-gradient-to-b from-[#1a0033] to-[#4a0080] ${className}`}>
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <h2 className="text-4xl md:text-5xl font-bold text-white mb-6">
          {title}
        </h2>
        <p className="text-xl text-gray-300 mb-8 max-w-2xl mx-auto">
          {description}
        </p>

        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <Link
            href={primaryButtonHref}
            className="group px-8 py-4 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold flex items-center justify-center gap-2 hover:scale-105 transition-transform"
          >
            <Download className="w-5 h-5" />
            {primaryButtonText}
            <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
          </Link>

          {secondaryButtonText && secondaryButtonHref && (
            <Link
              href={secondaryButtonHref}
              className="px-8 py-4 bg-white/10 backdrop-blur-sm text-white rounded-xl font-semibold border border-white/20 hover:bg-white/20 transition-colors"
            >
              {secondaryButtonText}
            </Link>
          )}
        </div>

        <div className="mt-8 text-sm text-gray-400">
          No credit card required • Free forever • Cancel anytime
        </div>
      </div>
    </section>
  );
}
