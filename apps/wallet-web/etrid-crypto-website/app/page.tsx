import Hero from "@/components/hero"
import Features from "@/components/features"
import Stats from "@/components/stats"
import Architecture from "@/components/architecture"
import Roadmap from "@/components/roadmap"
import Community from "@/components/community"
import Footer from "@/components/footer"

export default function Home() {
  return (
    <main className="min-h-screen">
      <Hero />
      <Features />
      <Stats />
      <Architecture />
      <Roadmap />
      <Community />
      <Footer />
    </main>
  )
}
