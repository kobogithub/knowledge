---
name: astro-best-practices
description: Astro framework best practices and performance optimization guidelines
version: 1.0.0
author: Knowledge Framework
tags: [astro, web, performance, ssg, ssr]
---

# Astro Best Practices

Expert guidelines for building performant, modern websites with Astro.

## Core Principles

### 1. Islands Architecture (Critical)
- **Use islands for interactivity**: Only ship JavaScript where needed
- **Prefer static over interactive**: Default to server-rendered HTML
- **Client directives**: Use `client:load`, `client:idle`, `client:visible` strategically

```astro
---
import Counter from '../components/Counter';
---
<!-- Only hydrate when visible -->
<Counter client:visible />
```

### 2. Performance Optimization (Critical)

**Image Optimization**
```astro
---
import { Image } from 'astro:assets';
import heroImage from '../assets/hero.jpg';
---
<Image src={heroImage} alt="Hero" width={800} height={600} />
```

**Lazy Loading**
- Use `loading="lazy"` for images below the fold
- Leverage `client:visible` for components
- Implement view transitions sparingly

**Build Optimization**
- Enable image optimization in `astro.config.mjs`
- Use `experimental.contentCollectionCache` for large sites
- Configure compression (gzip/brotli)

### 3. Content Collections (High)

**Structure**
```
src/content/
  blog/
    post-1.md
    post-2.md
  config.ts
```

**Define Schema**
```typescript
// src/content/config.ts
import { defineCollection, z } from 'astro:content';

const blog = defineCollection({
  type: 'content',
  schema: z.object({
    title: z.string(),
    pubDate: z.date(),
    description: z.string(),
    author: z.string(),
    tags: z.array(z.string()),
  }),
});

export const collections = { blog };
```

**Query Efficiently**
```astro
---
import { getCollection } from 'astro:content';
const posts = await getCollection('blog', ({ data }) => {
  return data.pubDate < new Date();
});
---
```

### 4. Routing & Pages (High)

**File-based Routing**
- Use `pages/` for routes
- `[slug].astro` for dynamic routes
- `[...slug].astro` for catch-all routes

**API Routes**
```typescript
// pages/api/posts.json.ts
export async function GET() {
  const posts = await fetchPosts();
  return new Response(JSON.stringify(posts), {
    headers: { 'Content-Type': 'application/json' }
  });
}
```

**Redirects**
```javascript
// astro.config.mjs
export default defineConfig({
  redirects: {
    '/old-page': '/new-page',
    '/blog/[slug]': '/articles/[slug]'
  }
});
```

### 5. Component Patterns (High)

**Props & TypeScript**
```astro
---
interface Props {
  title: string;
  description?: string;
  variant?: 'primary' | 'secondary';
}

const { title, description, variant = 'primary' } = Astro.props;
---
<div class={`card card--${variant}`}>
  <h2>{title}</h2>
  {description && <p>{description}</p>}
</div>
```

**Slots**
```astro
---
// BaseLayout.astro
---
<html>
  <head>
    <slot name="head" />
  </head>
  <body>
    <slot />
  </body>
</html>
```

### 6. Framework Integration (Medium)

**React, Vue, Svelte**
```javascript
// astro.config.mjs
import { defineConfig } from 'astro/config';
import react from '@astrojs/react';

export default defineConfig({
  integrations: [react()]
});
```

**Best Practices**
- Use Astro components for static content
- Reserve framework components for interactive features
- Minimize framework bundle size

### 7. Middleware & Security (Medium)

**Middleware**
```typescript
// src/middleware.ts
export function onRequest({ locals, request }, next) {
  locals.userId = getUserId(request);
  return next();
}
```

**Environment Variables**
```typescript
// Use PUBLIC_ prefix for client-side vars
const apiKey = import.meta.env.PUBLIC_API_KEY;

// Server-only vars
const secret = import.meta.env.SECRET_KEY;
```

### 8. View Transitions (Medium)

```astro
---
import { ViewTransitions } from 'astro:transitions';
---
<html>
  <head>
    <ViewTransitions />
  </head>
</html>
```

**Disable per-page**
```astro
<a href="/no-transition" data-astro-reload>No transition</a>
```

### 9. SEO & Meta Tags (High)

```astro
---
import { SEO } from 'astro-seo';
---
<SEO
  title="My Page Title"
  description="Page description for SEO"
  openGraph={{
    basic: {
      title: "My Page",
      type: "website",
      image: "/og-image.jpg",
    }
  }}
  twitter={{
    creator: "@handle"
  }}
/>
```

### 10. Build & Deploy (Medium)

**Static Site**
```javascript
export default defineConfig({
  output: 'static',
  build: {
    assets: 'assets'
  }
});
```

**SSR**
```javascript
import node from '@astrojs/node';

export default defineConfig({
  output: 'server',
  adapter: node({ mode: 'standalone' })
});
```

**Hybrid (Recommended)**
```javascript
export default defineConfig({
  output: 'hybrid',
  adapter: node()
});

// Mark specific pages as pre-rendered
export const prerender = true;
```

## Common Pitfalls

❌ **Overusing client directives**: Don't hydrate everything  
✅ Use `client:load` only for critical interactive components

❌ **Large JavaScript bundles**: Too many framework components  
✅ Prefer Astro components, use frameworks sparingly

❌ **Missing image optimization**: Using raw `<img>` tags  
✅ Always use `<Image>` component from `astro:assets`

❌ **No content collections**: Managing content manually  
✅ Use Content Collections for type-safe content management

❌ **Ignoring build output size**: Not checking bundle analyzer  
✅ Monitor build output, optimize heavy dependencies

## Performance Checklist

- [ ] Images optimized with `<Image>` component
- [ ] Client directives used strategically
- [ ] Content Collections for structured content
- [ ] TypeScript for type safety
- [ ] Proper caching headers configured
- [ ] Compression enabled (gzip/brotli)
- [ ] View transitions (if beneficial)
- [ ] Meta tags and SEO configured
- [ ] Bundle size monitored
- [ ] Lighthouse score > 95

## Resources

- [Official Docs](https://docs.astro.build)
- [Astro Cookbook](https://docs.astro.build/en/recipes/)
- [Astro Integrations](https://astro.build/integrations/)
- [Performance Guide](https://docs.astro.build/en/guides/performance/)
