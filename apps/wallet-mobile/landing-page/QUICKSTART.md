# Quick Start Guide

Get the Ëtrid Mobile Wallet Landing Page running in under 5 minutes.

## Prerequisites

- Node.js 18+ installed
- npm 9+ installed

## Installation (2 minutes)

```bash
# Navigate to project directory
cd /home/user/Etrid/apps/wallet-mobile/landing-page

# Install dependencies
npm install
```

## Development (1 minute)

```bash
# Start development server
npm run dev
```

Open [http://localhost:3001](http://localhost:3001) in your browser.

## Project Structure (Quick Reference)

```
├── app/                  # Pages (Next.js App Router)
│   ├── page.tsx         # Home page
│   ├── features/        # Features page
│   ├── pricing/         # Pricing page
│   └── download/        # Download page
│
├── components/          # Reusable components
│   ├── Hero.tsx        # Hero section
│   ├── Features.tsx    # Features grid
│   ├── Navigation.tsx  # Header
│   └── Footer.tsx      # Footer
│
└── public/             # Static assets
    ├── images/
    ├── screenshots/
    └── manifest.json
```

## Making Changes

### 1. Update Hero Text
Edit `/home/user/Etrid/apps/wallet-mobile/landing-page/components/Hero.tsx`:

```typescript
<h1 className="text-5xl md:text-7xl font-bold text-white mb-6 leading-tight">
  Your New Title
  <span className="block text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-blue-400">
    Your Subtitle
  </span>
</h1>
```

### 2. Update Features
Edit `/home/user/Etrid/apps/wallet-mobile/landing-page/components/Features.tsx`:

```typescript
const features = [
  {
    icon: YourIcon,
    title: 'Feature Name',
    description: 'Feature description',
    color: 'from-purple-400 to-purple-600',
  },
  // ... more features
];
```

### 3. Update Pricing
Edit `/home/user/Etrid/apps/wallet-mobile/landing-page/app/pricing/page.tsx`:

```typescript
const pricingTiers = [
  {
    name: 'Free',
    price: '$0',
    period: 'forever',
    // ... more config
  },
];
```

### 4. Update Colors
Edit `/home/user/Etrid/apps/wallet-mobile/landing-page/tailwind.config.ts`:

```typescript
colors: {
  'etrid-purple': {
    500: '#a855f7', // Your brand color
  },
}
```

### 5. Update SEO
Edit `/home/user/Etrid/apps/wallet-mobile/landing-page/app/layout.tsx`:

```typescript
export const metadata: Metadata = {
  title: 'Your Title',
  description: 'Your description',
  // ... more metadata
};
```

## Common Tasks

### Add a New Page

1. Create folder in `app/`:
   ```bash
   mkdir -p app/new-page
   ```

2. Create `page.tsx`:
   ```typescript
   export default function NewPage() {
     return (
       <main className="pt-16">
         <h1>New Page</h1>
       </main>
     );
   }
   ```

3. Add to navigation in `components/Navigation.tsx`

### Add a New Component

1. Create file in `components/`:
   ```bash
   touch components/NewComponent.tsx
   ```

2. Add component code:
   ```typescript
   export default function NewComponent() {
     return <div>New Component</div>;
   }
   ```

3. Import and use:
   ```typescript
   import NewComponent from '@/components/NewComponent';
   ```

### Update Images

1. Add images to `public/images/`
2. Use in components:
   ```typescript
   <img src="/images/your-image.png" alt="Description" />
   ```

## Building for Production

```bash
# Build the project
npm run build

# Test production build locally
npm start
```

## Deployment

### Vercel (Easiest)

```bash
# Install Vercel CLI
npm install -g vercel

# Deploy
vercel deploy --prod
```

### Other Options
See `DEPLOYMENT.md` for detailed deployment guides for:
- Netlify
- Docker
- AWS
- Self-hosted

## Troubleshooting

### Port 3001 already in use
```bash
# Kill process on port 3001
lsof -ti:3001 | xargs kill -9

# Or use a different port
npm run dev -- -p 3002
```

### Module not found errors
```bash
# Clear cache and reinstall
rm -rf node_modules .next
npm install
```

### Build errors
```bash
# Check TypeScript errors
npm run type-check

# Check linting
npm run lint
```

## Development Tips

### Hot Reload
Changes are automatically reflected in the browser. No need to refresh!

### Component Organization
- Keep components small and focused
- Use props for customization
- Extract repeated code into reusable components

### Styling
- Use Tailwind utility classes
- Consistent spacing (4, 6, 8, 12, 16, 24)
- Follow existing color scheme

### Performance
- Use `next/image` for images
- Lazy load heavy components
- Minimize client-side JavaScript

## Resources

- [Next.js Documentation](https://nextjs.org/docs)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [Lucide Icons](https://lucide.dev/)
- [TypeScript Documentation](https://www.typescriptlang.org/docs/)

## Need Help?

- Check `README.md` for detailed documentation
- Check `DEPLOYMENT.md` for deployment help
- Check `PROJECT_SUMMARY.md` for project overview
- Contact: hello@etrid.com

## Next Steps

1. ✅ Run `npm install`
2. ✅ Run `npm run dev`
3. ✅ View at http://localhost:3001
4. ✅ Make your first edit
5. ✅ Deploy to production

**You're all set! Happy coding!** 🚀
