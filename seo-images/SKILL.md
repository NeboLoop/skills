---
name: seo-images
description: Audit and optimize website images for SEO and performance, covering alt text quality, file sizes, modern formats (WebP/AVIF), lazy loading, image sitemaps, and file naming. Use when images aren't showing in search, pages load slowly due to images, or after an SEO audit flags image-related issues.
---

# SEO Images

Optimize your website images for search engines and performance — alt text, file sizes, formats, lazy loading, and image sitemaps.

---

## When to Use

Trigger this skill when you hear:
- "Check my images for SEO"
- "My images aren't showing up in search"
- "Are my alt tags good?"
- "My site loads slowly" (images are often the cause)
- "Optimize my images"
- "Help with image SEO"
- "My pages are too heavy"
- After an seo-audit flags image-related issues

This skill focuses specifically on how images affect your search rankings and page speed. Slow, poorly-labeled images hurt both.

---

## Context Gathering

Ask these questions conversationally. Don't dump them all at once.

### The Basics
1. **What's your website URL?** (Need to see the actual images in context)
2. **Which pages matter most?** (Homepage, product pages, blog posts — focus effort where it counts)
3. **Do you sell visual products?** (E-commerce and portfolio sites need image search traffic more than others)

### Current Setup
4. **Who manages your images?** (You, a designer, a CMS that auto-generates them?)
5. **Do you use a CDN or image optimization service?** (Cloudflare, Cloudinary, imgix, etc.)
6. **What CMS or platform are you on?** (WordPress, Shopify, custom — affects what's possible)

### Goals
7. **Are you trying to rank in Google Image Search?** (Some businesses get real traffic from image results)
8. **Is page speed a known problem?** (If Google PageSpeed Insights scores are low, images are usually the biggest fix)

---

## Methodology

### Step 1: Check Product Marketing Context
```
agent(resource: "memory", action: "search", query: "product marketing context")
```

Load existing context to understand what keywords matter and what the business does. This shapes what good alt text looks like.

### Step 2: Scan the Pages
Use Nebo's browser to visit the target pages:
```
web(action: "navigate", url: "their-website.com")
web(action: "read_page")
```

For each page, identify every image and collect:
- Image URL and file name
- Alt text (present or missing)
- File format (JPEG, PNG, GIF, SVG, WebP, AVIF)
- Approximate file size
- Image dimensions vs. display dimensions
- Whether lazy loading is applied
- Whether the image is above or below the fold

### Step 3: Evaluate Alt Text
For each image, check:

**Missing alt text** — Screen readers can't describe it, search engines can't understand it. Flag immediately.

**Generic alt text** — Things like "image1.jpg", "photo", "banner", or "IMG_4532" tell search engines nothing. Flag these.

**Keyword-stuffed alt text** — "Best cheap affordable pizza delivery near me pizza restaurant" is spam. Alt text should describe the image naturally while including relevant keywords where they fit.

**Good alt text formula:**
- Describes what's actually in the image
- Includes a relevant keyword naturally (not forced)
- Stays under 125 characters
- Reads like something you'd say out loud: "Team reviewing analytics dashboard during Monday standup"

**Decorative images** — Background textures, dividers, and spacers should have empty alt attributes (`alt=""`) so screen readers skip them.

### Step 4: Check File Sizes and Formats
Review each image for performance:

**Format recommendations:**
- Photos: WebP (80-90% smaller than JPEG with similar quality). AVIF if browser support isn't a concern.
- Graphics/logos with few colors: SVG (infinitely scalable, tiny file size)
- Graphics with transparency: WebP or PNG (WebP preferred for smaller size)
- Animated content: WebP animation or short video instead of GIF

**File size targets:**
- Hero/banner images: Under 200KB
- Content images: Under 100KB
- Thumbnails: Under 30KB
- If any single image exceeds 500KB, it's almost certainly too large

**Dimension check:**
- Are images being served at their display size? A 4000x3000 photo displayed at 400x300 wastes 99% of the data.
- Recommend responsive images using `srcset` for different screen sizes.

### Step 5: Check Lazy Loading
- Images below the fold should have `loading="lazy"` to defer loading until the user scrolls near them.
- Images above the fold (hero images, logos) should NOT be lazy-loaded — they need to appear immediately.
- Check if the CMS or framework handles this automatically.

### Step 6: Check Image Sitemaps
- Does the site's XML sitemap include image entries?
- For sites that depend on image search traffic (e-commerce, stock photos, portfolios), an image sitemap helps Google discover images it might miss.
- Each `<url>` entry can include `<image:image>` tags with `<image:loc>`, `<image:title>`, and `<image:caption>`.

### Step 7: Check File Names
- File names should be descriptive: `blue-running-shoes-side-view.webp` not `DSC_0042.jpg`
- Use hyphens, not underscores: `product-photo.webp` not `product_photo.webp`
- Keep them short but meaningful

### Step 8: Compile Recommendations
Prioritize findings by impact:
1. **Critical** — Missing alt text, images over 500KB, no lazy loading on heavy pages
2. **Important** — Generic alt text, wrong formats (PNG photos), oversized dimensions
3. **Nice to have** — File name improvements, image sitemap additions, AVIF adoption

---

## Output Format

```markdown
# Image SEO Audit
**Site:** [URL]
**Pages reviewed:** [List of pages]
**Date:** [DATE]

## Summary
- **Total images found:** [number]
- **Missing alt text:** [number]
- **Oversized images (>200KB):** [number]
- **Not using modern formats:** [number]
- **Missing lazy loading:** [number]

## Critical Issues
[List each issue with the specific image, what's wrong, and the fix]

### Example:
| Image | Issue | Fix |
|-------|-------|-----|
| homepage hero.png (1.2MB) | PNG photo, no compression | Convert to WebP, target under 150KB |
| /products/item3.jpg | Missing alt text | Add: "Wireless noise-canceling headphones in matte black" |
| blog/post-image.jpg (800x600 displayed at 200x150) | Oversized dimensions | Resize to 400x300, serve 200x150 for mobile |

## Alt Text Recommendations
[For each image with missing or poor alt text, provide the recommended text]

## Performance Wins
[Estimated page weight savings if recommendations are followed]
- Current total image weight: [X MB]
- Projected after optimization: [X KB]
- Estimated speed improvement: [X seconds faster]

## Image Sitemap
[Whether one exists, whether one is needed, and a sample entry if applicable]

## Next Steps
1. [Highest priority action]
2. [Second priority]
3. [Third priority]
```

---

## Quality Checks

Before delivering your audit, verify:

- [ ] **Every image on reviewed pages is accounted for** — don't miss images loaded via CSS or JavaScript
- [ ] **Alt text suggestions are natural** — read them out loud; they should sound like a human description
- [ ] **Alt text includes relevant keywords without stuffing** — one keyword per alt tag maximum
- [ ] **File size targets are realistic** — don't recommend 10KB for a full-width hero image
- [ ] **Format recommendations match the image type** — don't suggest WebP for simple logos (SVG is better)
- [ ] **Lazy loading advice accounts for above-the-fold content** — never lazy-load the hero image
- [ ] **Recommendations are prioritized** — the user knows what to fix first
- [ ] **CMS/platform constraints are considered** — don't recommend things their platform can't do

---

## Examples

### Example 1: E-commerce Product Pages

**User says:** "My product pages load slowly and my images don't show up in Google."

**You do:**
1. Check product marketing context — learn they sell handmade jewelry
2. Browse their product pages, find 12 images per page
3. Discover: all images are full-resolution PNGs (2-5MB each), none have alt text, no lazy loading
4. Report: "Your product pages are loading 40MB of images. Here's the fix."

**Key recommendations:**
- Convert all product photos to WebP (saves ~90% file size)
- Add descriptive alt text: "Handmade silver pendant necklace with turquoise stone" instead of "IMG_2847.png"
- Add lazy loading to all images except the main product photo
- Create an image sitemap — jewelry shoppers often search by image

### Example 2: Blog with Stock Photos

**User says:** "Are my blog images hurting my SEO?"

**You do:**
1. Check context — they run a B2B SaaS blog
2. Browse 5 recent posts, find generic stock photos
3. Discover: alt text on every image just says "blog image", all are JPEG at reasonable sizes
4. Report: "Your images aren't hurting page speed, but they're not helping SEO either."

**Key recommendations:**
- Replace generic alt text with descriptions that include the post's target keyword
- Example: Change `alt="blog image"` to `alt="Dashboard showing customer churn metrics by quarter"`
- Consider custom graphics instead of stock photos — they get shared more and can rank in image search
- Add `loading="lazy"` to images below the first paragraph

---

## Related Skills

- **seo-audit** — comprehensive SEO check that includes image issues alongside everything else
- **seo-sitemap** — generates XML sitemaps including image sitemap entries
- **seo-speed** — full page speed analysis where images are one factor among many
- **seo-plan** — strategic SEO roadmap that may include image optimization as a phase
- **product-marketing-context** — provides the keywords and positioning that shape good alt text
- **page-cro** — conversion optimization where image placement and loading speed affect results
