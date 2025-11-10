# Etrid Website - 3D Animation & UI Integration Guide

This guide covers integrating animated 3D graphics and UI elements into the Etrid website using multiple tools and libraries.

## Tools & Resources

### 1. Unicorn Studio (3D WebGL Animations)
**Installed**: `unicornstudio-react` ✅

**What it is**: No-code WebGL tool for creating interactive 3D scenes and animations
- Lightweight (36kb gzipped)
- Perfect for hero sections, backgrounds, and interactive elements
- Export and embed directly into React/Next.js

**How to use**:
1. Create your 3D animation at https://unicorn.studio
2. Export and get your `projectId`
3. Use the React component:

```tsx
import UnicornScene from "unicornstudio-react/next";

export default function Hero() {
  return (
    <div className="relative">
      <UnicornScene
        projectId="YOUR_PROJECT_ID"
        width="100vw"
        height="100vh"
        className="absolute inset-0 z-0"
        lazyLoad={true}
        fps={60}
        altText="Animated 3D background"
      />
      <div className="relative z-10">
        {/* Your content here */}
      </div>
    </div>
  );
}
```

**Best use cases for Etrid**:
- Hero section animated blockchain network visualization
- Floating 3D blockchain cubes/nodes
- Interactive particle systems
- Animated governance voting visualization
- 3D layer architecture diagrams

---

### 2. Uiverse.io (UI Component Library)
**Website**: https://uiverse.io

**What it is**: Largest open-source UI library with 3000+ animated components
- Copy-paste ready code
- CSS and Tailwind variants
- Buttons, cards, loaders, forms

**How to use**:
1. Visit https://uiverse.io/tags/react or https://uiverse.io/tags/reactjs
2. Browse components (glowing buttons, animated cards, etc.)
3. Copy the React/Tailwind code
4. Paste directly into your components

**Best use cases for Etrid**:
- Animated "Launch App" button with glow effects
- Glassmorphic cards for Features section
- Loading spinners for blockchain transactions
- Animated progress bars for staking
- Hover effects for governance proposals

**Example Integration**:
```tsx
// From uiverse.io - Glowing Button
export function GlowButton({ children }: { children: React.ReactNode }) {
  return (
    <button className="relative inline-flex h-12 overflow-hidden rounded-full p-[1px] focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2 focus:ring-offset-slate-50">
      <span className="absolute inset-[-1000%] animate-[spin_2s_linear_infinite] bg-[conic-gradient(from_90deg_at_50%_50%,#E2CBFF_0%,#393BB2_50%,#E2CBFF_100%)]" />
      <span className="inline-flex h-full w-full cursor-pointer items-center justify-center rounded-full bg-slate-950 px-8 py-1 text-sm font-medium text-white backdrop-blur-3xl">
        {children}
      </span>
    </button>
  );
}
```

---

### 3. Jitter.video (Motion Design)
**Website**: https://jitter.video

**What it is**: Motion design tool for creating professional animations
- Export as Lottie, GIF, or 4K video
- Figma meets After Effects
- Used by Microsoft, Dropbox, Anthropic, Lyft

**How to use**:
1. Create animations at https://jitter.video
2. Export as Lottie JSON
3. Install Lottie player: `npm install lottie-react`
4. Use in React:

```tsx
import Lottie from 'lottie-react';
import animationData from './animation.json';

export function AnimatedIcon() {
  return (
    <Lottie
      animationData={animationData}
      loop={true}
      style={{ width: 200, height: 200 }}
    />
  );
}
```

**Best use cases for Etrid**:
- Animated logo reveals
- Feature icons with micro-interactions
- Explainer animations for consensus mechanism
- Animated infographics for tokenomics
- Loading states and transitions

---

### 4. Space Type Generator
**Website**: https://spacetypegenerator.com

**What it is**: Kinetic typography generator for animated text
- Built with p5.js/Processing
- Creates space-themed animated typography
- Web-based, no software needed

**How to use**:
1. Create animated typography at https://spacetypegenerator.com
2. Export as video/GIF
3. OR recreate effect using React libraries like:
   - `react-type-animation` (typewriter effects)
   - `framer-motion` (text animations)

**Example with react-type-animation**:
```bash
npm install react-type-animation
```

```tsx
import { TypeAnimation } from 'react-type-animation';

export function AnimatedTitle() {
  return (
    <TypeAnimation
      sequence={[
        'ËTRID',
        1000,
        'Decentralized Democracy',
        1000,
        'On-Chain Governance',
        1000,
      ]}
      wrapper="h1"
      speed={50}
      className="text-7xl font-bold"
      repeat={Infinity}
    />
  );
}
```

**Best use cases for Etrid**:
- Animated hero title
- Dynamic taglines
- Feature headings with typewriter effect
- Stats counters with animated numbers

---

## Implementation Recommendations

### Priority 1: Hero Section
**Replace current particle animation with Unicorn Studio 3D scene**

Create a 3D blockchain network visualization:
- Floating geometric nodes
- Connecting lines showing network activity
- Subtle rotation and movement
- Responsive to mouse movement

### Priority 2: Features Section
**Add Uiverse.io animated cards**

- Glowing borders on hover
- Subtle scale animations
- Glassmorphic backgrounds
- Animated icons from Jitter

### Priority 3: Architecture Section
**Use Unicorn Studio for 3D layer visualization**

Create an interactive 3D representation of:
- Layer 1: Flare Chain (base)
- Layer 2: Partition Burst Chains (middle)
- Layer 3: State Channels (top)

Users can rotate and explore the architecture

### Priority 4: Stats Section
**Add animated counters and progress bars**

Use Uiverse.io components for:
- Animated number counters
- Progress bars for network stats
- Glowing metric cards

### Priority 5: Community Section
**Jitter animations for social icons**

Create animated hover states for:
- Discord icon
- Twitter icon
- Telegram icon
- GitHub icon

---

## Next Steps

1. **Create your first Unicorn Studio scene**
   - Visit https://unicorn.studio
   - Sign up for free account
   - Create a simple 3D scene
   - Export and get project ID

2. **Browse Uiverse.io**
   - Find 3-5 components you like
   - Save the code snippets
   - Test in development

3. **Optional: Create Jitter animations**
   - Sign up at https://jitter.video
   - Create icon animations
   - Export as Lottie files

4. **Integration**
   - Start with hero section
   - Test performance
   - Add more scenes gradually

---

## Performance Tips

- Use `lazyLoad={true}` for Unicorn Studio scenes
- Load heavy 3D animations only above the fold
- Use placeholders while loading
- Optimize FPS (30-60 depending on complexity)
- Test on mobile devices
- Consider reducing quality on slower connections

---

## Support & Documentation

- **Unicorn Studio Docs**: https://www.unicorn.studio/docs/
- **Uiverse.io Gallery**: https://uiverse.io/tags/react
- **Jitter Docs**: https://jitter.video/
- **React Type Animation**: https://react-type-animation.netlify.app/

---

**Created**: November 2025
**For**: Etrid Cryptocurrency Website
**Developer**: Eoj
