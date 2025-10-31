# ËTRID Website - Deployment Guide

## Overview
This is a professional, production-ready landing page for ËTRID Protocol, designed to be uploaded directly to Hostinger.

## File Structure
```
website/
├── index.html          # Main landing page
├── css/
│   └── styles.css      # All custom styles
├── js/
│   └── main.js         # Interactive functionality
├── images/
│   └── (add logo here)
└── README.md          # This file
```

## Features
- ✅ Single-page responsive design
- ✅ Modern dark theme with blue/purple gradients
- ✅ Smooth animations and transitions
- ✅ Mobile-responsive navigation
- ✅ SEO optimized
- ✅ Fast loading (uses CDN for fonts/Tailwind)
- ✅ Accessibility features
- ✅ Cross-browser compatible

## Uploading to Hostinger

### Method 1: File Manager (Recommended for beginners)
1. Log in to your Hostinger control panel
2. Navigate to **File Manager**
3. Go to `public_html` directory
4. Upload all files maintaining the directory structure:
   - Upload `index.html` to `public_html/`
   - Create `css` folder and upload `styles.css`
   - Create `js` folder and upload `main.js`
   - Create `images` folder and upload your logo

### Method 2: FTP (Recommended for developers)
1. Get your FTP credentials from Hostinger control panel
2. Use an FTP client (FileZilla, Cyberduck, etc.)
3. Connect to your hosting:
   - Host: ftp.yourdomain.com
   - Username: [from Hostinger]
   - Password: [from Hostinger]
   - Port: 21
4. Navigate to `public_html`
5. Upload all files maintaining the directory structure

### Method 3: Git Deployment (Advanced)
If you have SSH access:
```bash
# On your hosting server
cd public_html
git clone [your-repo-url] .
# Or manually upload files
```

## Post-Upload Checklist
- [ ] Verify all files uploaded correctly
- [ ] Check that CSS and JS files are loading (no 404 errors)
- [ ] Test on multiple devices (desktop, tablet, mobile)
- [ ] Test on multiple browsers (Chrome, Firefox, Safari, Edge)
- [ ] Verify all external links work
- [ ] Check page load speed
- [ ] Test mobile menu functionality
- [ ] Verify animations work smoothly

## Customization

### Adding Your Logo
1. Add your logo file to the `images/` folder (e.g., `logo.svg` or `logo.png`)
2. Update `index.html` line ~36 to include an image:
   ```html
   <a href="#" class="flex items-center">
       <img src="images/logo.svg" alt="ËTRID" class="h-8 w-auto mr-2">
       <span class="text-2xl font-display font-bold...">ËTRID</span>
   </a>
   ```

### Updating Colors
Edit `index.html` tailwind.config section (around line 38):
```javascript
colors: {
    'etrid-blue': '#3B82F6',      // Change this
    'etrid-purple': '#8B5CF6',    // Change this
    'etrid-dark': '#0A0E1A',
    'etrid-darker': '#050812',
}
```

### Updating Stats
Edit the stats section in `index.html` (around line 170):
```html
<div class="text-4xl font-bold...">
    142k+  <!-- Change this -->
</div>
```

### Adding More Sections
Follow the existing pattern:
```html
<section id="new-section" class="py-20 bg-etrid-dark">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <!-- Your content -->
    </div>
</section>
```

## Performance Optimization

### Current Optimizations
- Tailwind CSS loaded via CDN (cached globally)
- Google Fonts with preconnect
- Minimal external dependencies
- GPU-accelerated animations
- Lazy loading support built-in

### Further Optimization (Optional)
1. **Minify CSS/JS**: Use tools like `cssnano` or `terser`
2. **Enable Gzip**: Usually enabled by default on Hostinger
3. **Add Service Worker**: Uncomment code in `main.js`
4. **Use WebP Images**: Convert images to WebP format
5. **Add Caching Headers**: Configure via `.htaccess`

## SEO Enhancements

### Already Included
- Meta descriptions
- Open Graph tags
- Twitter Card tags
- Semantic HTML5
- Descriptive alt texts

### Additional Steps
1. Add `robots.txt`:
   ```
   User-agent: *
   Allow: /
   Sitemap: https://etrid.org/sitemap.xml
   ```

2. Add `sitemap.xml`:
   ```xml
   <?xml version="1.0" encoding="UTF-8"?>
   <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
       <url>
           <loc>https://etrid.org/</loc>
           <lastmod>2025-10-28</lastmod>
           <priority>1.0</priority>
       </url>
   </urlset>
   ```

3. Submit to search engines:
   - Google Search Console
   - Bing Webmaster Tools

## Analytics (Optional)

Add Google Analytics by inserting before `</head>`:
```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_MEASUREMENT_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_MEASUREMENT_ID');
</script>
```

## Browser Support
- Chrome/Edge: Latest 2 versions
- Firefox: Latest 2 versions
- Safari: Latest 2 versions
- Mobile Safari: iOS 12+
- Chrome Mobile: Latest

## Troubleshooting

### CSS Not Loading
- Check file path is correct: `css/styles.css`
- Verify file uploaded to correct directory
- Check browser console for 404 errors

### JS Not Working
- Check browser console for errors
- Verify `js/main.js` path is correct
- Ensure file has correct permissions (644)

### Fonts Not Loading
- CDN links should work globally
- Check internet connection
- Verify no ad blockers blocking Google Fonts

### Mobile Menu Not Working
- Check JavaScript is enabled
- Verify `main.js` loaded correctly
- Test in different browsers

## Support Links
- Hostinger Support: https://www.hostinger.com/tutorials
- Tailwind CSS Docs: https://tailwindcss.com/docs
- Web Performance: https://web.dev/measure/

## License
© 2025 ËTRID Foundation. All rights reserved.

## Contact
For technical support: etridfoundation@proton.me

---

**Ready to upload!** Simply drag and drop all files to your Hostinger File Manager or use FTP.

The website is fully self-contained and will work immediately after upload.