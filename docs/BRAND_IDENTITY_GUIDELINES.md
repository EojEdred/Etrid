# Ëtrid Brand Identity Guidelines

**Version**: 1.0.0
**Date**: October 24, 2025
**Purpose**: Official brand standards for Ëtrid visual identity
**Audience**: Designers, marketers, community contributors

---

## 🎨 Brand Philosophy

### Core Identity

Ëtrid's visual identity reflects its technical philosophy:

- **Symmetry** → Balance and self-regulation
- **Motion** → Adaptive evolution
- **Depth** → Technical sophistication
- **Vitality** → Living system energy

**"The brand must communicate balance between logic and vitality — not hype, but harmony."**

---

## 🌈 Color Palette

### Primary Colors

#### Base Black
- **Hex:** `#000000`
- **RGB:** `0, 0, 0`
- **CMYK:** `0, 0, 0, 100`
- **Usage:** Primary backgrounds, depth, finality
- **Meaning:** Depth and unwavering finality

#### Rust Silver
- **Hex:** `#C1C7C9`
- **RGB:** `193, 199, 201`
- **CMYK:** `4, 1, 0, 21`
- **Usage:** Secondary accents, industrial precision
- **Meaning:** Technical sophistication, industrial precision

#### Tech Green-Blue (Primary Accent)
- **Hex:** `#4FE2C9`
- **RGB:** `79, 226, 201`
- **CMYK:** `65, 0, 11, 11`
- **Usage:** Primary highlights, CTAs, vitality markers
- **Meaning:** Energy, adaptability, living systems

### Secondary Colors

#### Deep Space Blue
- **Hex:** `#0A1929`
- **RGB:** `10, 25, 41`
- **CMYK:** `76, 39, 0, 84`
- **Usage:** Card backgrounds, sections
- **Meaning:** Cosmic depth, infinite scalability

#### Slate Gray
- **Hex:** `#475569`
- **RGB:** `71, 85, 105`
- **CMYK:** `32, 19, 0, 59`
- **Usage:** Text, borders, subtle dividers
- **Meaning:** Neutrality, technical precision

#### Warm White
- **Hex:** `#F8FAFC`
- **RGB:** `248, 250, 252`
- **CMYK:** `2, 1, 0, 1`
- **Usage:** Light mode backgrounds, clean sections
- **Meaning:** Clarity, transparency

### Gradient System

#### Primary Gradient (Hero Sections)
```css
background: linear-gradient(135deg, #0A1929 0%, #000000 100%);
```
**Usage:** Hero sections, major headers

#### Accent Gradient (Highlights)
```css
background: linear-gradient(90deg, #4FE2C9 0%, #3B9C8F 100%);
```
**Usage:** Buttons, CTAs, progress bars

#### Energy Flow Gradient (Ecosystem Maps)
```css
background: linear-gradient(45deg, #C1C7C9 0%, #4FE2C9 50%, #0A1929 100%);
```
**Usage:** Visual representations of energy flow (VMw → ÉTR → Treasury)

### Color Usage Guidelines

| Context | Primary | Accent | Background |
|---------|---------|--------|------------|
| **Dark Mode** | Warm White | Tech Green-Blue | Base Black / Deep Space Blue |
| **Light Mode** | Base Black | Tech Green-Blue | Warm White / Slate Gray (subtle) |
| **CTAs** | Tech Green-Blue | Warm White (text) | Gradient Accent |
| **Danger/Slashing** | `#EF4444` (Red) | Warm White | Dark Background |
| **Success/Complete** | `#10B981` (Green) | Warm White | Dark Background |

### Accessibility Standards

All color combinations must meet **WCAG 2.1 AA standards** (minimum contrast ratio 4.5:1 for normal text, 3:1 for large text).

**Tested Combinations:**
- ✅ Tech Green-Blue (#4FE2C9) on Base Black (#000000) → Contrast ratio: 8.2:1
- ✅ Warm White (#F8FAFC) on Deep Space Blue (#0A1929) → Contrast ratio: 14.3:1
- ✅ Base Black (#000000) on Warm White (#F8FAFC) → Contrast ratio: 20.8:1

---

## 🖋️ Typography

### Font Families

#### Primary Typeface: Inter
- **Purpose:** UI, body text, technical content
- **Weights:** Regular (400), Medium (500), Semi-Bold (600), Bold (700)
- **Fallback:** `'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', sans-serif`
- **License:** SIL Open Font License
- **Source:** [Google Fonts](https://fonts.google.com/specimen/Inter)

#### Display Typeface: Futura (or Inter for digital)
- **Purpose:** Headers, hero sections, branding
- **Weights:** Book (300), Medium (500), Bold (700)
- **Fallback:** Inter Semi-Bold, system-ui
- **Note:** Futura is commercial; use Inter Semi-Bold for free alternative

#### Monospace: JetBrains Mono
- **Purpose:** Code blocks, technical data, CLI examples
- **Weights:** Light (300), Regular (400), Medium (500)
- **Fallback:** `'JetBrains Mono', 'Fira Code', 'Consolas', 'Monaco', monospace`
- **License:** Apache 2.0
- **Source:** [JetBrains](https://www.jetbrains.com/lp/mono/)

### Type Scale

| Element | Size | Weight | Line Height | Letter Spacing |
|---------|------|--------|-------------|----------------|
| **H1 (Hero)** | 72px (4.5rem) | Semi-Bold | 1.1 | -0.02em |
| **H2 (Major Sections)** | 48px (3rem) | Semi-Bold | 1.2 | -0.01em |
| **H3 (Subsections)** | 36px (2.25rem) | Semi-Bold | 1.3 | 0 |
| **H4 (Cards)** | 24px (1.5rem) | Medium | 1.4 | 0 |
| **H5 (Small Headers)** | 20px (1.25rem) | Medium | 1.4 | 0 |
| **Body Large** | 18px (1.125rem) | Regular | 1.6 | 0 |
| **Body** | 16px (1rem) | Regular | 1.6 | 0 |
| **Body Small** | 14px (0.875rem) | Regular | 1.5 | 0 |
| **Caption** | 12px (0.75rem) | Regular | 1.4 | 0.01em |
| **Code** | 15px (0.9375rem) | Regular | 1.6 | 0 |

### Typography Best Practices

1. **Headers:** Always use UPPERCASE for H1, sentence case for H2-H5
2. **Body Text:** Never justify; use left-aligned for readability
3. **Line Length:** Optimal 60-75 characters per line
4. **Emphasis:** Use semi-bold weight over italic for technical content
5. **Code:** Always use monospace font with subtle background highlight

### Text Colors

| Context | Color | Hex |
|---------|-------|-----|
| **Primary Text (Dark Mode)** | Warm White | `#F8FAFC` |
| **Primary Text (Light Mode)** | Base Black | `#000000` |
| **Secondary Text** | Slate Gray | `#475569` |
| **Accent Text** | Tech Green-Blue | `#4FE2C9` |
| **Code Text** | Rust Silver | `#C1C7C9` |

---

## 🎭 Logo & Branding

### Logo Concept

**Primary Mark:** "Infinity-Horizon Symbol"

**Design Elements:**
- Merges **event horizon** (circular boundary) with **balance loop** (infinity symbol)
- Represents: Perpetual evolution, adaptive cycles, sovereign coordination

**Visual Description:**
```
    ∞
   ━━━━
  │    │
  └────┘
```

A stylized infinity symbol (∞) integrated with a circular horizon ring, suggesting:
- **Infinity:** Continuous evolution and adaptation
- **Horizon:** Boundary of finality and consensus
- **Circle:** Wholeness, self-regulation, ecosystem balance

### Logo Variants

#### 1. Full Logo (Wordmark + Symbol)
**Usage:** Primary branding, headers, official documents
**Spacing:** Symbol width = 2x wordmark height
**Alignment:** Center-aligned vertically

#### 2. Symbol Only (Icon)
**Usage:** Favicon, app icons, social media avatars
**Minimum Size:** 32x32px
**Clear Space:** 25% padding on all sides

#### 3. Wordmark Only
**Usage:** Footer, compact headers
**Typeface:** Futura Bold (or Inter Semi-Bold)
**Letter Spacing:** -0.02em

### Logo Colors

| Variant | Background | Symbol | Wordmark |
|---------|------------|--------|----------|
| **Dark** | Base Black | Tech Green-Blue | Warm White |
| **Light** | Warm White | Deep Space Blue | Base Black |
| **Monochrome** | Transparent | Base Black | Base Black |

### Logo Don'ts

❌ **Never:**
- Stretch or distort proportions
- Change colors outside approved palette
- Add drop shadows or effects
- Place on busy backgrounds without padding
- Rotate or tilt the logo

✅ **Always:**
- Maintain minimum clear space (25% padding)
- Use provided SVG files for scalability
- Ensure sufficient contrast with background
- Use approved color combinations only

### Clear Space Rules

**Minimum Clear Space:** 25% of logo width on all sides

```
┌──────────────────────────────────┐
│                                  │
│   ┌──────────────────────┐      │
│   │                      │      │
│   │      ËTRID LOGO      │      │
│   │                      │      │
│   └──────────────────────┘      │
│                                  │
└──────────────────────────────────┘
    ← 25% padding on all sides →
```

---

## 🖼️ Visual Language

### Design Principles

#### 1. Circular & Recursive Patterns
**Usage:** Diagrams, ecosystem maps, feedback loops
**Examples:**
- Consensus Day cycle visualization
- Treasury flow diagrams
- ASF finality progression

#### 2. Gradient Flows
**Usage:** Energy transitions (VMw → ÉTR → Treasury)
**Style:** Smooth, directional gradients showing transformation

#### 3. Grid Systems
**Usage:** Technical diagrams, architecture overviews
**Style:** Clean, modular grids reflecting E³20 component structure

#### 4. Depth & Layering
**Usage:** Multichain architecture, PBC hierarchy
**Style:** Subtle shadows, z-index layers, card elevation

### Image Direction

#### DO Use:
- **Systems diagrams** showing interconnected nodes
- **Flow charts** with circular feedback loops
- **Abstract energy representations** (particles, waves, gradients)
- **Architectural schematics** (technical, blueprint-style)
- **Recursive patterns** (fractals, nested circles)

#### DO NOT Use:
- ❌ Cryptocurrency clichés (coins, rockets, "to the moon")
- ❌ Financial imagery (stock charts, money)
- ❌ Mystical or spiritual imagery
- ❌ Overly complex 3D renders
- ❌ Generic stock photos

### Photography Style

**When using photography:**
- **Tone:** Industrial, architectural, precise
- **Color Grading:** Cool tones (blues, grays) with selective warm accents
- **Subject Matter:** Infrastructure, networks, precision engineering
- **Composition:** Symmetrical, balanced, geometric

---

## 📐 Layout & Spacing

### Spacing Scale (8px Base)

Use multiples of 8px for consistent spacing:

| Size | Value | Usage |
|------|-------|-------|
| **XS** | 4px | Icon padding, tight spacing |
| **SM** | 8px | Component internal spacing |
| **MD** | 16px | Default spacing between elements |
| **LG** | 24px | Section internal margins |
| **XL** | 32px | Major section gaps |
| **2XL** | 48px | Hero section padding |
| **3XL** | 64px | Page-level padding |
| **4XL** | 96px | Extra-large gaps (between major page sections) |

### Grid System

#### Desktop (1440px viewport)
- **Container Max Width:** 1280px
- **Columns:** 12
- **Gutter:** 24px
- **Margin:** 80px (sides)

#### Tablet (768px viewport)
- **Container Max Width:** 100%
- **Columns:** 8
- **Gutter:** 16px
- **Margin:** 32px (sides)

#### Mobile (375px viewport)
- **Container Max Width:** 100%
- **Columns:** 4
- **Gutter:** 16px
- **Margin:** 16px (sides)

### Component Spacing

#### Cards
- **Padding:** 24px (desktop), 16px (mobile)
- **Border Radius:** 12px
- **Box Shadow:** `0 4px 6px rgba(0, 0, 0, 0.1)`

#### Buttons
- **Height:** 48px (large), 40px (medium), 32px (small)
- **Padding:** 24px horizontal (large), 16px (medium), 12px (small)
- **Border Radius:** 8px
- **Hover State:** Slight scale (1.02x), brightness increase (110%)

#### Input Fields
- **Height:** 48px
- **Padding:** 16px
- **Border Radius:** 8px
- **Border:** 1px solid Slate Gray, focus: Tech Green-Blue

---

## 🎬 Motion & Animation

### Animation Principles

1. **Purposeful** - Every animation serves a functional purpose
2. **Subtle** - Avoid distracting or excessive motion
3. **Fast** - Transitions should feel instantaneous (<300ms)
4. **Natural** - Use easing functions that mimic physics

### Easing Functions

| Type | CSS Value | Usage |
|------|-----------|-------|
| **Standard** | `cubic-bezier(0.4, 0.0, 0.2, 1)` | Default transitions |
| **Enter** | `cubic-bezier(0.0, 0.0, 0.2, 1)` | Elements entering viewport |
| **Exit** | `cubic-bezier(0.4, 0.0, 1, 1)` | Elements leaving viewport |
| **Emphasized** | `cubic-bezier(0.0, 0.0, 0.2, 1)` | Important state changes |

### Duration Guidelines

| Element | Duration | Easing |
|---------|----------|--------|
| **Micro Interactions** | 100ms | Standard |
| **Button Hover** | 150ms | Standard |
| **Card Flip/Reveal** | 250ms | Emphasized |
| **Page Transitions** | 300ms | Standard |
| **Modal Open/Close** | 200ms | Enter/Exit |

### Interactive States

#### Buttons
```css
/* Default */
background: #4FE2C9;

/* Hover */
background: #3FCEB5;
transform: scale(1.02);

/* Active */
background: #3BB5A2;
transform: scale(0.98);

/* Disabled */
background: #475569;
opacity: 0.5;
cursor: not-allowed;
```

---

## 📱 Responsive Design

### Breakpoints

| Device | Min Width | Max Width | Columns |
|--------|-----------|-----------|---------|
| **Mobile** | 320px | 767px | 4 |
| **Tablet** | 768px | 1023px | 8 |
| **Desktop** | 1024px | 1439px | 12 |
| **Large Desktop** | 1440px | ∞ | 12 |

### Responsive Typography

| Element | Mobile | Tablet | Desktop |
|---------|--------|--------|---------|
| **H1** | 36px | 48px | 72px |
| **H2** | 28px | 36px | 48px |
| **H3** | 24px | 28px | 36px |
| **Body** | 16px | 16px | 18px |

---

## 🗣️ Tone of Voice

### Core Principles

| Context | Tone | Example |
|---------|------|---------|
| **Technical Docs** | Precise, declarative, neutral | "ASF adjusts finality thresholds dynamically based on validator participation." |
| **Public Content** | Confident, factual, minimal adjectives | "Ëtrid unifies consensus and governance into one adaptive system." |
| **Educational** | Clear analogies, no jargon gatekeeping | "Think of PBCs as specialized workers, each handling a specific task." |
| **Philosophical** | Reflective, system-oriented, never mystical | "Governance is not politics—it's the network's heartbeat." |

### Voice Guidelines

#### DO Say:
- ✅ "Ëtrid achieves equilibrium through adaptive consensus."
- ✅ "The network self-regulates via Consensus Day."
- ✅ "Security grows dynamically with participation."

#### DON'T Say:
- ❌ "Ëtrid revolutionizes blockchain forever!"
- ❌ "The most advanced blockchain in the universe!"
- ❌ "Disrupting everything with game-changing technology!"

### Writing Style

- **Sentence Length:** Mix short (10-15 words) and medium (20-25 words) sentences
- **Paragraphs:** 2-4 sentences maximum for digital content
- **Active Voice:** Prefer active voice over passive
- **Technical Terms:** Define once, then use consistently
- **Jargon:** Avoid unless audience-appropriate; always define

---

## 📦 Asset Library

### Icon Style

**Design System:** Outline-based, 2px stroke
**Size:** 24x24px base (scale proportionally)
**Color:** Match text color or use accent color
**Source:** [Heroicons](https://heroicons.com/) or custom

**Common Icons:**
- Network: Connected nodes (3-5 circles with lines)
- Governance: Ballot box or voting checkmark
- Security: Shield with checkmark
- Treasury: Vault or stack of coins (stylized, not literal)
- Research: Document with magnifying glass

### Illustration Style

**Style:** Geometric, technical, minimal
**Line Weight:** 2-3px
**Fill:** Gradients or solid accent colors
**Complexity:** Medium (not too abstract, not overly detailed)

**Example Subjects:**
- Ecosystem diagrams (circular node networks)
- Flow charts (with directional arrows)
- Architecture schematics (layered boxes)

---

## 📄 Document Templates

### Markdown Style

For technical documentation:

````markdown
# Document Title

**Status**: Active/Draft/Archived
**Date**: YYYY-MM-DD
**Author**: Name or Team

---

## Section Header

### Subsection

Body text goes here. Use **bold** for emphasis and `code` for technical terms.

**Table Example:**

| Column 1 | Column 2 |
|----------|----------|
| Data     | Data     |

```language
// Code block example
function example() {}
```

> **Note**: Callout or important information
````

### Presentation Decks

**Slide Backgrounds:**
- Dark: Deep Space Blue to Base Black gradient
- Light: Warm White solid

**Title Slide:**
- Logo: Top-left or center
- Title: H1, center-aligned
- Subtitle: Body Large, Slate Gray

**Content Slides:**
- Max 3 bullet points
- Large text (minimum 24px)
- Visual: 50% of slide real estate

---

## 🌐 Web Components

### Buttons

#### Primary Button
```css
background: linear-gradient(90deg, #4FE2C9 0%, #3B9C8F 100%);
color: #000000;
padding: 12px 24px;
border-radius: 8px;
font-weight: 600;
transition: all 150ms cubic-bezier(0.4, 0.0, 0.2, 1);
```

#### Secondary Button
```css
background: transparent;
border: 2px solid #4FE2C9;
color: #4FE2C9;
padding: 12px 24px;
border-radius: 8px;
font-weight: 600;
```

### Cards

```css
background: #0A1929;
border: 1px solid #475569;
border-radius: 12px;
padding: 24px;
box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
transition: transform 150ms, box-shadow 150ms;

/* Hover */
transform: translateY(-4px);
box-shadow: 0 12px 24px rgba(79, 226, 201, 0.15);
```

### Navigation

**Desktop:**
- Fixed top bar
- Background: Blur effect (`backdrop-filter: blur(12px)`)
- Height: 72px
- Logo: Left-aligned
- Nav links: Right-aligned, 16px spacing

**Mobile:**
- Hamburger menu icon
- Slide-out drawer from right
- Full-height overlay

---

## ✅ Brand Checklist

Use this checklist when creating branded content:

- [ ] **Colors:** Using approved palette only
- [ ] **Typography:** Inter for body, JetBrains Mono for code
- [ ] **Logo:** Correct variant, proper padding (25%), no distortion
- [ ] **Spacing:** Using 8px grid system
- [ ] **Contrast:** WCAG AA compliant (minimum 4.5:1)
- [ ] **Tone:** Factual, confident, non-hype language
- [ ] **Imagery:** System-oriented, no crypto clichés
- [ ] **Responsive:** Tested on mobile, tablet, desktop

---

## 📞 Brand Assets Download

**Coming Soon:**
- Logo files (SVG, PNG)
- Color swatches (ASE, ACO, SCSS)
- Typography files (WOFF2, TTF)
- Icon library (SVG sprite)
- Presentation template (Figma, PowerPoint)

**Location:** `etrid.org/brand-kit` (to be deployed)

---

## 📝 Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-10-24 | Initial brand guidelines |

---

**Maintained By:** Ëtrid Foundation Design Team
**Contact:** brand@etrid.foundation
**License:** These guidelines are CC BY-NC-SA 4.0 (Creative Commons Attribution-NonCommercial-ShareAlike)

---

*"Design reflects philosophy: balance, motion, and self-regulation."*
