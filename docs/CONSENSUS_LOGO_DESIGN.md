# Consensus Logo Design Specification

**Brand**: Consensus (Ëtrid Governance System)
**Date**: October 24, 2025
**Style**: Matches Ëtrid branding aesthetic

---

## 🎨 Logo Concept: "Consensus Ë"

### Primary Design Elements

**Core Symbol**: The letter **"Ë"** (E with two dots/diaeresis)
- Represents: Consensus, agreement, collective decision
- Style: Neon glow effect matching Ëtrid logo
- Typography: Modern, geometric, technical

**Visual Metaphor**: The two dots above the E represent:
- **Dual quorum** (citizen + security/validator quorum)
- **Balance** between community and technical governance
- **Two eyes** watching/verifying (transparency)
- **Binary choice** (yes/no voting)

---

## 🌈 Color Palette

### Primary Gradient (Matches Ëtrid Brand)

**Option 1: Purple-to-Cyan (Governance Focus)**
```
Left (Purple): #B83FE0 → #9333EA
Right (Cyan): #4FE2C9 → #06B6D4
```
**Meaning**: Bridges community (warm purple) and technical (cool cyan)

**Option 2: Blue-to-Teal (Trust & Stability)**
```
Left (Blue): #3B82F6 → #2563EB
Right (Teal): #14B8A6 → #0D9488
```
**Meaning**: Trust, transparency, democratic stability

**Option 3: Gold-to-Cyan (Premium Governance)**
```
Left (Gold): #F59E0B → #D97706
Right (Cyan): #4FE2C9 → #06B6D4
```
**Meaning**: Value, importance, critical system

**Recommended**: **Option 1 (Purple-to-Cyan)** - Best brand consistency with Ëtrid

---

## 🎭 Design Variations

### Variation 1: Microchip Style (Matches Ëtrid Logo)

**Description**: Letter "Ë" inside rounded square with connection pins

```
        ⚬  ⚬  ⚬  ⚬
         ━━━━━━━━
    ━━━━│        │━━━━
        │   Ë    │
    ━━━━│        │━━━━
         ━━━━━━━━
        ⚬  ⚬  ⚬  ⚬
```

**Elements:**
- Rounded square border (chip frame)
- 4 connection pins on each side (12 total) - represents **12-month consensus cycle**
- Glowing "Ë" in center
- Gradient border (purple → cyan)
- Neon glow effect

**Size**: 512x512px base (scalable SVG)

---

### Variation 2: Circular Cycle (Consensus Day Emphasis)

**Description**: "Ë" inside circular ring representing annual cycle

```
           ⚬
       ╱       ╲
      ⚬    Ë    ⚬
       ╲       ╱
           ⚬
```

**Elements:**
- Circular outer ring (year cycle)
- 4 nodes at cardinal points (4 Consensus Day phases)
- "Ë" in center
- Rotation effect suggesting cyclical governance

**Phases Represented by 4 Nodes:**
1. Registration (top)
2. Voting (right)
3. Minting (bottom)
4. Distribution (left)

---

### Variation 3: Voting Checkmark Integration

**Description**: "Ë" combined with checkmark (✓) symbol

```
    Ë ✓
```

**Elements:**
- "Ë" forms left part
- Checkmark extends from right side
- Represents: Decision + Verification
- Clean, minimal design

---

## 🖋️ Typography

### Wordmark: "CONSENSUS"

**Font Family**:
- **Primary**: Inter Semi-Bold (matches Ëtrid)
- **Alternative**: Futura Bold

**Letter Spacing**: `-0.02em` (tight, technical)

**Case**: ALL CAPS

**Color**:
- Light background: `#000000` (Base Black)
- Dark background: `#F8FAFC` (Warm White)

### Full Logo Composition

```
┌─────────────────────────────┐
│         [Ë SYMBOL]          │
│                             │
│        CONSENSUS            │
└─────────────────────────────┘
```

**Spacing**: Symbol width = 1.5x wordmark height

---

## 📐 Technical Specifications

### SVG Logo (Variation 1 - Microchip Style)

```svg
<svg width="512" height="512" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <!-- Gradient Definition -->
    <linearGradient id="consensusGradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#B83FE0;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#4FE2C9;stop-opacity:1" />
    </linearGradient>

    <!-- Glow Effect -->
    <filter id="glow">
      <feGaussianBlur stdDeviation="8" result="coloredBlur"/>
      <feMerge>
        <feMergeNode in="coloredBlur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
  </defs>

  <!-- Chip Border (Rounded Square) -->
  <rect x="96" y="96" width="320" height="320" rx="40" ry="40"
        fill="none" stroke="url(#consensusGradient)" stroke-width="8"
        filter="url(#glow)"/>

  <!-- Connection Pins - Top -->
  <rect x="160" y="40" width="20" height="40" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="210" y="40" width="20" height="40" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="260" y="40" width="20" height="40" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="310" y="40" width="20" height="40" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>

  <!-- Connection Pins - Right -->
  <rect x="432" y="160" width="40" height="20" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="432" y="210" width="40" height="20" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="432" y="260" width="40" height="20" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="432" y="310" width="40" height="20" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>

  <!-- Connection Pins - Bottom -->
  <rect x="160" y="432" width="20" height="40" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="210" y="432" width="20" height="40" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="260" y="432" width="20" height="40" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="310" y="432" width="20" height="40" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>

  <!-- Connection Pins - Left -->
  <rect x="40" y="160" width="40" height="20" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="40" y="210" width="40" height="20" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="40" y="260" width="40" height="20" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <rect x="40" y="310" width="40" height="20" rx="10" fill="url(#consensusGradient)" filter="url(#glow)"/>

  <!-- Letter Ë (E with diaeresis) -->
  <text x="256" y="290" font-family="Inter, sans-serif" font-weight="700"
        font-size="180" fill="url(#consensusGradient)" text-anchor="middle"
        filter="url(#glow)">
    Ë
  </text>

  <!-- Alternative: Draw E + dots manually for more control -->
  <!-- E shape -->
  <path d="M 180 180 L 330 180 M 180 180 L 180 330 M 180 255 L 310 255 M 180 330 L 330 330"
        stroke="url(#consensusGradient)" stroke-width="24" stroke-linecap="round"
        fill="none" filter="url(#glow)"/>

  <!-- Two dots (diaeresis) -->
  <circle cx="220" cy="140" r="12" fill="url(#consensusGradient)" filter="url(#glow)"/>
  <circle cx="280" cy="140" r="12" fill="url(#consensusGradient)" filter="url(#glow)"/>
</svg>
```

**Save as**: `consensus-logo.svg`

---

## 🎨 Color Variations

### Dark Mode (Primary)
- Background: `#000000` (Base Black)
- Symbol: Purple-Cyan gradient
- Glow: Subtle cyan/purple halo

### Light Mode
- Background: `#F8FAFC` (Warm White)
- Symbol: Darker gradient (`#9333EA` → `#0D9488`)
- Glow: Subtle shadow instead of glow

### Monochrome
- Background: Transparent
- Symbol: Single color `#4FE2C9` (Tech Green-Blue)
- No glow effect

---

## 📏 Size Guidelines

### Minimum Sizes
- **Digital Display**: 32x32px (favicon)
- **Print**: 0.5 inch / 13mm

### Recommended Sizes
- **App Icon**: 512x512px, 1024x1024px
- **Website Header**: 64px height
- **Social Media**: 400x400px (square), 1200x630px (banner)

### Clear Space
- Minimum padding: **25%** of logo width on all sides

---

## 🖼️ Usage Contexts

### Primary Usage
- Consensus Day announcements
- Governance dashboard header
- Voting interface
- Proposal cards
- Annual reports

### Secondary Usage
- Documentation section headers (governance docs)
- Social media posts about governance
- Governance UI components
- Consensus Day countdown timers

---

## 🎯 Design Rationale

### Why Microchip Style?
1. **Brand Consistency**: Matches Ëtrid's technical, processor-like aesthetic
2. **12 Pins = 12 Months**: Represents annual cycle
3. **Circuit Board = Network**: Symbolizes interconnected stakeholders
4. **Modern & Technical**: Aligns with E³20 protocol sophistication

### Why the Ë (E with dots)?
1. **Brand Unity**: Maintains Ëtrid visual language
2. **Dual Meaning**: Two dots = dual quorum system
3. **Unique Identity**: Distinctive from plain "C" for Consensus
4. **Phonetic Link**: Ë sound connects to "equal" and "equilibrium"

### Why Purple-Cyan Gradient?
1. **Warm → Cool**: Represents community (warm) meeting technical (cool)
2. **Visible Spectrum**: Shows transparency and full visibility
3. **Brand Colors**: Matches established Ëtrid palette
4. **Trust & Innovation**: Purple (trust, wisdom) + Cyan (innovation, clarity)

---

## 📦 Deliverables Checklist

For GizziClaude to generate:

### Logo Files
- [ ] `consensus-logo.svg` (primary, scalable vector)
- [ ] `consensus-logo.png` (@1x, @2x, @3x)
- [ ] `consensus-logo-dark.png` (for light backgrounds)
- [ ] `consensus-logo-light.png` (for dark backgrounds)
- [ ] `consensus-logo-mono.png` (single color)
- [ ] `consensus-favicon.ico` (16x16, 32x32, 48x48)

### Wordmark Files
- [ ] `consensus-wordmark.svg`
- [ ] `consensus-wordmark.png`

### Full Logo (Symbol + Wordmark)
- [ ] `consensus-full-logo.svg`
- [ ] `consensus-full-logo.png`

### Application Icons
- [ ] `consensus-icon-512.png` (for apps)
- [ ] `consensus-icon-1024.png` (high-res)

### Social Media
- [ ] `consensus-social-square.png` (1200x1200px)
- [ ] `consensus-social-banner.png` (1200x630px)

---

## 🎨 Figma/Canva Instructions

### For Figma:
1. Create 512x512px frame
2. Draw rounded rectangle (40px corner radius)
3. Add 16 small rectangles as pins (4 per side)
4. Center letter "Ë" (Inter Bold, 180pt)
5. Apply linear gradient (purple → cyan, 45° angle)
6. Add glow effect:
   - Effects → Drop Shadow
   - Color: `#B83FE0` at 60% opacity
   - Blur: 24px
   - Spread: 8px

### For Canva:
1. Create custom size: 512x512px
2. Use "Elements" → "Shapes" → Rounded square
3. Add text: "Ë" (font: Futura Bold or similar)
4. Apply gradient fill (use brand colors)
5. Duplicate and blur for glow effect
6. Add small rectangles as chip pins

---

## 🚀 Next Steps

1. **Generate SVG** using code above
2. **Export PNG variants** at multiple resolutions
3. **Test on dark/light backgrounds**
4. **Create favicon** (.ico format)
5. **Add to brand kit** alongside Ëtrid logo

---

**Status**: ✅ Design Spec Complete
**Ready For**: GizziClaude implementation in Figma/Canva
**Approval Needed**: Yes (from Eoj/Foundation)

---

*"Two dots, one voice. Consensus through coordination."*
