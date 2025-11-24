# Quick Start - Etrid Unified Portal

## Access the Portal

**URL**: http://localhost:3000
**Status**: RUNNING
**Dev Server**: Already started

---

## Quick Commands

```bash
# Navigate to portal
cd /Users/macbook/Desktop/etrid/apps/unified-portal

# Start dev server (if not running)
npm run dev

# Build for production
npm run build

# Run production server
npm start

# Run linter
npm run lint

# Type check
npx tsc --noEmit
```

---

## What's Available

### Routes
- `/` - Dashboard (complete)
- `/lightning` - Placeholder
- `/validator` - Placeholder
- `/watchtower` - Placeholder
- `/wallet` - Placeholder
- `/governance` - Placeholder
- `/monitoring` - Placeholder
- `/masterchef` - Placeholder

### Components
**Location**: `components/`

- `ui/button.tsx` - Button component
- `ui/card.tsx` - Card component
- `ui/badge.tsx` - Badge component
- `layout/portal-header.tsx` - Header
- `layout/portal-nav.tsx` - Navigation
- `layout/status-bar.tsx` - Footer
- `dashboard/stat-card.tsx` - Stats
- `dashboard/service-card.tsx` - Services
- `theme-provider.tsx` - Theme

### Features
- Dark/Light theme toggle (top right)
- Responsive design
- Purple/blue gradients
- Network stats in footer
- 8 service navigation tabs

---

## For Other Agents

### To Migrate a Feature

1. **Navigate to your feature directory**:
   ```bash
   cd /Users/macbook/Desktop/etrid/apps/unified-portal
   ```

2. **Replace the placeholder**:
   ```bash
   # Example: Lightning Network
   # Edit app/lightning/page.tsx
   ```

3. **Use shared components**:
   ```tsx
   import { Button } from "@/components/ui/button"
   import { Card } from "@/components/ui/card"
   ```

4. **Add sub-routes if needed**:
   ```bash
   mkdir -p app/lightning/channels
   # Create app/lightning/channels/page.tsx
   ```

5. **Test locally**:
   ```bash
   npm run dev
   # Visit http://localhost:3000/lightning
   ```

---

## File Structure

```
app/
├── layout.tsx          # Root layout
├── page.tsx            # Dashboard
├── globals.css         # Styles
└── {feature}/
    └── page.tsx        # Feature page

components/
├── ui/                 # Shared UI
├── layout/             # Layout
└── dashboard/          # Dashboard

lib/
└── utils.ts            # Utilities
```

---

## Theme Colors

**Primary**: Purple 600 (`#9333ea`)
**Accent**: Blue 600 (`#2563eb`)

Use in components:
```tsx
className="text-purple-600 dark:text-purple-400"
className="bg-gradient-to-r from-purple-600 to-blue-600"
```

---

## Need Help?

1. Check `README.md` for full documentation
2. Check `AGENT_1_SUMMARY.md` for detailed specs
3. Check `FILE_TREE.md` for file structure
4. Look at existing components for patterns

---

**Agent 1 Built This - Ready for Agents 2-7**
