# Ëtrid Mobile Wallet Landing Page

A stunning, conversion-focused marketing landing page for the Ëtrid Mobile Wallet, showcasing all 18 features and driving app installs.

## Features

- **Modern Design**: Sleek, professional design with purple/blue gradient theme
- **Responsive**: Fully responsive for mobile, tablet, and desktop
- **SEO Optimized**: Complete metadata, sitemap, and robots.txt
- **Performance**: Optimized with Next.js 14 App Router
- **PWA Ready**: Progressive Web App support with manifest
- **18 Feature Showcase**: Complete feature grid with detailed descriptions

## Pages

- **Home** (`/`): Hero, features grid, stats, download section
- **Features** (`/features`): Detailed feature showcase with deep dives
- **Pricing** (`/pricing`): Three pricing tiers with feature comparison
- **Download** (`/download`): Download options for Web, iOS, Android

## Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **Icons**: Lucide React
- **Deployment**: Vercel/Netlify ready

## Getting Started

### Prerequisites

- Node.js 18 or later
- npm 9 or later

### Installation

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

The app will be available at `http://localhost:3001`

## Project Structure

```
landing-page/
├── app/
│   ├── page.tsx              # Home page
│   ├── features/page.tsx     # Features showcase
│   ├── pricing/page.tsx      # Pricing tiers
│   ├── download/page.tsx     # Download page
│   ├── layout.tsx            # Root layout
│   ├── globals.css           # Global styles
│   └── sitemap.ts            # Sitemap generation
├── components/
│   ├── Hero.tsx              # Hero section
│   ├── Features.tsx          # Features grid
│   ├── Stats.tsx             # Statistics section
│   ├── DownloadSection.tsx   # Download CTA
│   ├── Navigation.tsx        # Header navigation
│   └── Footer.tsx            # Footer
├── public/
│   ├── images/               # Image assets
│   ├── screenshots/          # App screenshots
│   ├── manifest.json         # PWA manifest
│   └── robots.txt            # SEO robots
├── package.json              # Dependencies
├── tailwind.config.ts        # Tailwind config
├── next.config.js            # Next.js config
└── tsconfig.json             # TypeScript config
```

## Key Features

### Hero Section
- Animated gradient background
- Interactive phone mockup
- Social proof metrics
- Dual CTAs

### Features Grid
- 18 feature cards
- Color-coded icons
- Hover effects
- Responsive layout

### Stats Section
- Development metrics
- Revenue potential
- Build statistics
- Gradient text effects

### Download Section
- Platform options (Web/iOS/Android)
- Feature highlights
- Coming soon badges
- Direct launch links

## Deployment

### Vercel (Recommended)

```bash
npm install -g vercel
vercel deploy
```

### Netlify

```bash
npm run build
# Deploy the .next folder
```

### Docker

```bash
docker build -t etrid-landing .
docker run -p 3001:3001 etrid-landing
```

## Environment Variables

Create a `.env.local` file:

```env
NEXT_PUBLIC_SITE_URL=https://wallet.etrid.com
```

## Performance Optimizations

- ✅ Next.js 14 App Router for optimal performance
- ✅ Image optimization with next/image
- ✅ Lazy loading components
- ✅ Code splitting
- ✅ Minification and compression
- ✅ Security headers
- ✅ PWA support

## SEO Features

- ✅ Complete metadata for all pages
- ✅ OpenGraph tags
- ✅ Twitter Card tags
- ✅ Dynamic sitemap
- ✅ Robots.txt
- ✅ Structured data ready

## Browser Support

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)
- Mobile browsers

## License

Copyright © 2025 Ëtrid. All rights reserved.

## Support

For questions or issues, contact: hello@etrid.com
