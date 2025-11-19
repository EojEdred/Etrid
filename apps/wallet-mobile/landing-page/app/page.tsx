import Hero from '@/components/Hero';
import Features from '@/components/Features';
import Stats from '@/components/Stats';
import DownloadSection from '@/components/DownloadSection';

export default function HomePage() {
  return (
    <main>
      <Hero />
      <Features />
      <Stats />
      <DownloadSection />
    </main>
  );
}
