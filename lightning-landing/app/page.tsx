'use client';

import Hero from '@/components/Hero';
import Features from '@/components/Features';
import HowItWorks from '@/components/HowItWorks';
import SupportedChains from '@/components/SupportedChains';
import Statistics from '@/components/Statistics';
import UseCases from '@/components/UseCases';
import Demo from '@/components/Demo';
import Developer from '@/components/Developer';
import Roadmap from '@/components/Roadmap';
import Footer from '@/components/Footer';

export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-b from-gray-900 via-purple-900 to-gray-900">
      <Hero />
      <Features />
      <HowItWorks />
      <SupportedChains />
      <Statistics />
      <UseCases />
      <Demo />
      <Developer />
      <Roadmap />
      <Footer />
    </main>
  );
}
