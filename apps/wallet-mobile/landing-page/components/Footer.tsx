import { Twitter, Github, Mail } from 'lucide-react';

export default function Footer() {
  return (
    <footer className="bg-[#0d001a] border-t border-white/10">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="grid md:grid-cols-4 gap-8 mb-8">
          {/* Company */}
          <div>
            <h3 className="font-semibold text-white mb-4">Company</h3>
            <div className="space-y-2">
              <a href="/about" className="block text-gray-400 hover:text-white">About</a>
              <a href="/careers" className="block text-gray-400 hover:text-white">Careers</a>
              <a href="/blog" className="block text-gray-400 hover:text-white">Blog</a>
            </div>
          </div>

          {/* Product */}
          <div>
            <h3 className="font-semibold text-white mb-4">Product</h3>
            <div className="space-y-2">
              <a href="#features" className="block text-gray-400 hover:text-white">Features</a>
              <a href="/pricing" className="block text-gray-400 hover:text-white">Pricing</a>
              <a href="/security" className="block text-gray-400 hover:text-white">Security</a>
            </div>
          </div>

          {/* Resources */}
          <div>
            <h3 className="font-semibold text-white mb-4">Resources</h3>
            <div className="space-y-2">
              <a href="/docs" className="block text-gray-400 hover:text-white">Documentation</a>
              <a href="/support" className="block text-gray-400 hover:text-white">Support</a>
              <a href="/api" className="block text-gray-400 hover:text-white">API</a>
            </div>
          </div>

          {/* Legal */}
          <div>
            <h3 className="font-semibold text-white mb-4">Legal</h3>
            <div className="space-y-2">
              <a href="/privacy" className="block text-gray-400 hover:text-white">Privacy</a>
              <a href="/terms" className="block text-gray-400 hover:text-white">Terms</a>
              <a href="/cookies" className="block text-gray-400 hover:text-white">Cookies</a>
            </div>
          </div>
        </div>

        <div className="border-t border-white/10 pt-8 flex flex-col md:flex-row justify-between items-center">
          <div className="text-gray-400 text-sm mb-4 md:mb-0">
            © 2025 Ëtrid. All rights reserved.
          </div>

          <div className="flex gap-6">
            <a href="https://twitter.com/etrid" className="text-gray-400 hover:text-white">
              <Twitter className="w-5 h-5" />
            </a>
            <a href="https://github.com/etrid" className="text-gray-400 hover:text-white">
              <Github className="w-5 h-5" />
            </a>
            <a href="mailto:hello@etrid.com" className="text-gray-400 hover:text-white">
              <Mail className="w-5 h-5" />
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}
