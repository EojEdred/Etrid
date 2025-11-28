'use client';

import { FaGithub, FaDiscord, FaTwitter, FaTelegram } from 'react-icons/fa';

export default function Footer() {
  return (
    <footer className="bg-gray-900 border-t border-purple-500/20">
      <div className="section-container">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8 mb-8">
          <div>
            <h3 className="text-xl font-bold mb-4 gradient-text">ÉTRID Lightning</h3>
            <p className="text-gray-400">Lightning-fast payments across 14 blockchains</p>
          </div>
          <div>
            <h4 className="font-bold mb-4">Resources</h4>
            <ul className="space-y-2 text-gray-400">
              <li><a href="#" className="hover:text-purple-400">Documentation</a></li>
              <li><a href="#" className="hover:text-purple-400">API Reference</a></li>
              <li><a href="#" className="hover:text-purple-400">GitHub</a></li>
            </ul>
          </div>
          <div>
            <h4 className="font-bold mb-4">Community</h4>
            <ul className="space-y-2 text-gray-400">
              <li><a href="#" className="hover:text-purple-400">Discord</a></li>
              <li><a href="#" className="hover:text-purple-400">Twitter</a></li>
              <li><a href="#" className="hover:text-purple-400">Telegram</a></li>
            </ul>
          </div>
          <div>
            <h4 className="font-bold mb-4">Newsletter</h4>
            <input type="email" placeholder="your@email.com" className="w-full p-3 bg-gray-800 rounded mb-2" />
            <button className="btn-primary w-full">Subscribe</button>
          </div>
        </div>

        <div className="border-t border-gray-800 pt-8 flex flex-col md:flex-row justify-between items-center">
          <p className="text-gray-400 mb-4 md:mb-0">© 2025 ÉTRID. All rights reserved.</p>
          <div className="flex gap-6">
            <a href="#" className="text-2xl text-gray-400 hover:text-purple-400 transition"><FaGithub /></a>
            <a href="#" className="text-2xl text-gray-400 hover:text-purple-400 transition"><FaDiscord /></a>
            <a href="#" className="text-2xl text-gray-400 hover:text-purple-400 transition"><FaTwitter /></a>
            <a href="#" className="text-2xl text-gray-400 hover:text-purple-400 transition"><FaTelegram /></a>
          </div>
        </div>
      </div>
    </footer>
  );
}
