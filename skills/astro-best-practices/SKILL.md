---
name: astro-best-practices
description: Astro framework best practices for content-driven sites and web apps
version: 1.0.0
tags:
  - best-practices
  - astro
  - frontend
  - ssr
  - islands
---

# astro-best-practices

Mejores practicas para Astro: arquitectura de islands, rendering strategies, content collections, integraciones y performance.

## Overview

Astro se usa en este proyecto para:
- **Frontend web**: Sitio principal con SSR/SSG
- **Content-driven**: Blog, docs, paginas estaticas
- **Island architecture**: Componentes interactivos solo donde se necesitan
- **Performance**: Zero JS by default, progressive enhancement

## Estructura de Proyecto

### Layout estandar

```
frontend/
├── astro.config.mjs
├── package.json
├── tsconfig.json
├── public/
│   ├── favicon.svg
│   └── fonts/
├── src/
│   ├── components/
│   │   ├── ui/              # Componentes base (Button, Card, Input)
│   │   ├── layout/          # Header, Footer, Sidebar, Nav
│   │   └── features/        # Componentes de feature especificos
│   ├── layouts/
│   │   ├── Base.astro       # Layout HTML base
│   │   ├── Page.astro       # Layout de pagina con header/footer
│   │   └── Dashboard.astro  # Layout de dashboard con sidebar
│   ├── pages/
│   │   ├── index.astro
│   │   ├── login.astro
│   │   ├── dashboard/
│   │   │   ├── index.astro
│   │   │   └── settings.astro
│   │   └── api/             # API routes (endpoints)
│   │       └── health.ts
│   ├── content/
│   │   ├── config.ts
│   │   └── blog/
│   ├── styles/
│   │   └── global.css
│   ├── lib/                 # Utilities, helpers
│   │   ├── api.ts           # API client
│   │   ├── auth.ts          # Auth helpers
│   │   └── utils.ts
│   └── env.d.ts
└── tailwind.config.mjs
```

## Island Architecture

### Directivas de client

```astro
---
// src/pages/dashboard.astro
import Counter from '../components/Counter.tsx';
import Chart from '../components/Chart.tsx';
import Sidebar from '../components/Sidebar.tsx';
import StaticCard from '../components/StaticCard.astro';
---

<!-- Sin directiva: renderiza HTML estatico, 0 JS -->
<StaticCard title="Revenue" value="$12,345" />

<!-- client:load - Se hidrata inmediatamente -->
<!-- Usar para: contenido above-the-fold que necesita interactividad -->
<Counter client:load initialCount={0} />

<!-- client:visible - Se hidrata cuando entra en viewport -->
<!-- Usar para: contenido below-the-fold, graficos, carousels -->
<Chart client:visible data={chartData} />

<!-- client:idle - Se hidrata cuando el browser esta idle -->
<!-- Usar para: analytics, features no criticas -->
<Sidebar client:idle />

<!-- client:media - Se hidrata segun media query -->
<!-- Usar para: componentes que solo se muestran en ciertos viewports -->
<MobileMenu client:media="(max-width: 768px)" />

<!-- client:only="react" - Solo se renderiza en el cliente -->
<!-- Usar para: componentes que dependen de APIs del browser -->
<MapWidget client:only="react" />
```

### Elegir la directiva correcta

```
¿El componente necesita JavaScript?
├── NO → Componente .astro (sin directiva)
└── SI →
    ¿Es visible al cargar la pagina (above-the-fold)?
    ├── SI → client:load
    └── NO →
        ¿Es critico para la experiencia?
        ├── SI → client:idle
        └── NO → client:visible
```

## Componentes Astro

### Componente basico

```astro
---
// src/components/ui/Card.astro
interface Props {
  title: string;
  description?: string;
  href?: string;
  class?: string;
}

const { title, description, href, class: className } = Astro.props;
---

<div class:list={["card", className]}>
  <h3 class="card-title">
    {href ? <a href={href}>{title}</a> : title}
  </h3>
  {description && <p class="card-description">{description}</p>}
  <slot />
</div>

<style>
  .card {
    @apply rounded-lg border border-gray-200 p-6 shadow-sm;
  }
  .card-title {
    @apply text-lg font-semibold;
  }
  .card-description {
    @apply mt-2 text-gray-600;
  }
</style>
```

### Layout con slots

```astro
---
// src/layouts/Page.astro
import Base from './Base.astro';
import Header from '../components/layout/Header.astro';
import Footer from '../components/layout/Footer.astro';

interface Props {
  title: string;
  description?: string;
}

const { title, description } = Astro.props;
---

<Base {title} {description}>
  <Header />
  <main class="container mx-auto px-4 py-8">
    <slot name="hero" />
    <slot />
    <slot name="sidebar" />
  </main>
  <Footer />
</Base>
```

### Componente con fetch de datos

```astro
---
// src/pages/dashboard/index.astro
import DashboardLayout from '../../layouts/Dashboard.astro';
import StatsGrid from '../../components/features/StatsGrid.astro';
import { getStats } from '../../lib/api';

// Fetch en build/request time (no en el browser)
const stats = await getStats();
---

<DashboardLayout title="Dashboard">
  <StatsGrid stats={stats} />
</DashboardLayout>
```

## Rendering Strategies

### Static (SSG) - Default

```javascript
// astro.config.mjs
export default defineConfig({
  output: 'static',  // Default: pre-render todo
});
```

### Server (SSR)

```javascript
// astro.config.mjs
export default defineConfig({
  output: 'server',  // SSR por default
  adapter: node({ mode: 'standalone' }),
});
```

### Hybrid (SSG + SSR por pagina)

```javascript
// astro.config.mjs
export default defineConfig({
  output: 'hybrid',  // SSG default, opt-in SSR
  adapter: node({ mode: 'standalone' }),
});
```

```astro
---
// Esta pagina se pre-renderiza (SSG)
export const prerender = true;
---

---
// Esta pagina es SSR (en modo hybrid, opt-out de prerender)
export const prerender = false;

// Acceso a request, cookies, headers
const user = Astro.cookies.get('session');
if (!user) {
  return Astro.redirect('/login');
}
---
```

## API Routes (Endpoints)

```typescript
// src/pages/api/health.ts
import type { APIRoute } from 'astro';

export const GET: APIRoute = async () => {
  return new Response(JSON.stringify({ status: 'ok' }), {
    status: 200,
    headers: { 'Content-Type': 'application/json' },
  });
};

// src/pages/api/users/[id].ts
export const GET: APIRoute = async ({ params }) => {
  const { id } = params;
  const user = await fetchUser(id);

  if (!user) {
    return new Response(JSON.stringify({ error: 'Not found' }), {
      status: 404,
      headers: { 'Content-Type': 'application/json' },
    });
  }

  return new Response(JSON.stringify(user), {
    status: 200,
    headers: { 'Content-Type': 'application/json' },
  });
};

export const POST: APIRoute = async ({ request }) => {
  const body = await request.json();
  // Validar y procesar...
  return new Response(JSON.stringify({ created: true }), { status: 201 });
};
```

## Content Collections

### Configuracion

```typescript
// src/content/config.ts
import { defineCollection, z } from 'astro:content';

const blog = defineCollection({
  type: 'content',
  schema: z.object({
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
    draft: z.boolean().default(false),
    tags: z.array(z.string()).default([]),
    author: z.string().default('Team'),
  }),
});

export const collections = { blog };
```

### Usar content collection

```astro
---
// src/pages/blog/index.astro
import { getCollection } from 'astro:content';

const posts = (await getCollection('blog', ({ data }) => !data.draft))
  .sort((a, b) => b.data.pubDate.valueOf() - a.data.pubDate.valueOf());
---

<ul>
  {posts.map((post) => (
    <li>
      <a href={`/blog/${post.slug}`}>{post.data.title}</a>
      <time>{post.data.pubDate.toLocaleDateString()}</time>
    </li>
  ))}
</ul>
```

## Tailwind CSS

### Configuracion

```javascript
// tailwind.config.mjs
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f0f9ff',
          500: '#0ea5e9',
          900: '#0c4a6e',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
  ],
};
```

### Scoped styles vs Tailwind

```astro
<!-- Preferir Tailwind para utility-first -->
<div class="flex items-center gap-4 rounded-lg bg-white p-6 shadow-sm">
  <h2 class="text-xl font-bold text-gray-900">{title}</h2>
</div>

<!-- Scoped styles para componentes complejos -->
<style>
  .complex-animation {
    /* Animaciones complejas que no encajan en utilities */
    animation: slide-in 0.3s ease-out;
  }
  @keyframes slide-in {
    from { transform: translateX(-100%); opacity: 0; }
    to { transform: translateX(0); opacity: 1; }
  }
</style>
```

## Performance

### Optimizacion de imagenes

```astro
---
import { Image } from 'astro:assets';
import heroImage from '../assets/hero.png';
---

<!-- Optimizacion automatica: WebP, srcset, lazy loading -->
<Image
  src={heroImage}
  alt="Hero image description"
  width={1200}
  height={600}
  loading="eager"  <!-- above-the-fold -->
/>

<!-- Para imagenes below-the-fold -->
<Image
  src={heroImage}
  alt="Description"
  loading="lazy"   <!-- default -->
/>
```

### Prefetch

```astro
<!-- Prefetch en hover (default) -->
<a href="/about">About</a>

<!-- Prefetch inmediato -->
<a href="/dashboard" data-astro-prefetch="load">Dashboard</a>

<!-- Sin prefetch -->
<a href="/external" data-astro-prefetch="false">External</a>
```

### Bundle analysis

```bash
# Ver tamano del bundle
npx astro build -- --verbose

# Analizar bundle
npx astro build && npx vite-bundle-visualizer
```

## Mejores Practicas

### DO

- Usar componentes `.astro` para todo lo estatico (zero JS)
- Elegir la directiva client correcta (load/visible/idle)
- Fetch datos en el frontmatter (server-side), no en el browser
- Usar `Image` de `astro:assets` para optimizacion automatica
- Tipar props con `interface Props`
- Usar layouts para estructura comun (no repetir HTML)
- Separar componentes UI base de componentes de feature
- Usar content collections para contenido tipado
- Configurar `prerender` explicitamente en paginas SSR

### DON'T

- Usar `client:load` en todo — la mayoria de componentes no necesitan JS
- Hacer fetch en el browser si se puede hacer en el server
- Crear componentes React/Vue para contenido estatico — usar `.astro`
- Olvidar el `alt` en imagenes
- Mezclar logica de negocio en componentes — extraer a `lib/`
- Usar `<script>` global cuando un island es mejor
- Importar librerias pesadas sin `client:visible` o `client:idle`
- Ignorar TypeScript — Astro tiene soporte nativo
- Poner estilos globales que deberian ser scoped
- Crear paginas sin layout

## Recursos

- [Astro Documentation](https://docs.astro.build/)
- [Astro Islands](https://docs.astro.build/en/concepts/islands/)
- [Astro Content Collections](https://docs.astro.build/en/guides/content-collections/)
- [Astro View Transitions](https://docs.astro.build/en/guides/view-transitions/)
- [Astro + Tailwind](https://docs.astro.build/en/guides/integrations-guide/tailwind/)
