---
name: supabase-postgres-best-practices
description: Supabase and PostgreSQL best practices, RLS, migrations, Edge Functions
version: 1.0.0
tags:
  - best-practices
  - supabase
  - postgresql
  - database
  - auth
  - rls
---

# supabase-postgres-best-practices

Mejores practicas para Supabase y PostgreSQL: schema design, Row Level Security, migrations, Edge Functions, Auth y patrones de produccion.

## Overview

Supabase provee:
- **PostgreSQL**: Base de datos relacional completa con extensiones
- **Auth**: Autenticacion y autorizacion con JWT
- **RLS**: Row Level Security para control de acceso a nivel de fila
- **Realtime**: Subscripciones a cambios en base de datos
- **Storage**: Almacenamiento de archivos con politicas de acceso
- **Edge Functions**: Funciones serverless en Deno
- **PostgREST**: API REST automatica desde el schema de DB

## Supabase CLI

### Setup

```bash
# Instalar
npm install -g supabase

# O con brew
brew install supabase/tap/supabase

# Inicializar proyecto
supabase init

# Login
supabase login

# Linkear con proyecto remoto
supabase link --project-ref <project-id>

# Iniciar servicios locales
supabase start

# Status
supabase status
```

### Estructura del proyecto

```
project/
├── supabase/
│   ├── config.toml              # Configuracion local
│   ├── migrations/
│   │   ├── 20250101000000_create_users.sql
│   │   ├── 20250102000000_create_posts.sql
│   │   └── 20250103000000_add_rls_policies.sql
│   ├── functions/
│   │   ├── hello/
│   │   │   └── index.ts
│   │   └── process-webhook/
│   │       └── index.ts
│   └── seed.sql                 # Datos iniciales
├── src/
│   └── ...
└── package.json
```

## Schema Design

### Tablas con convenciones

```sql
-- Usar snake_case para todo
-- Incluir timestamps en todas las tablas
-- Usar UUID como primary key (compatible con Auth)

CREATE TABLE public.profiles (
    id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    username TEXT UNIQUE NOT NULL,
    display_name TEXT,
    avatar_url TEXT,
    bio TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trigger para updated_at automatico
CREATE OR REPLACE FUNCTION public.handle_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON public.profiles
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_updated_at();

-- Tabla con relaciones
CREATE TABLE public.posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES public.profiles(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indices para queries frecuentes
CREATE INDEX idx_posts_author_id ON public.posts(author_id);
CREATE INDEX idx_posts_published ON public.posts(published) WHERE published = TRUE;
CREATE INDEX idx_posts_created_at ON public.posts(created_at DESC);
```

### Profile automatico al registrarse

```sql
-- Crear profile automaticamente cuando se registra un usuario
CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.profiles (id, username, display_name, avatar_url)
    VALUES (
        NEW.id,
        NEW.raw_user_meta_data ->> 'username',
        COALESCE(NEW.raw_user_meta_data ->> 'full_name', NEW.raw_user_meta_data ->> 'name'),
        NEW.raw_user_meta_data ->> 'avatar_url'
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_new_user();
```

### Enums y tipos custom

```sql
-- Usar enums de Postgres (no strings arbitrarios)
CREATE TYPE public.post_status AS ENUM ('draft', 'published', 'archived');

ALTER TABLE public.posts
    ADD COLUMN status public.post_status NOT NULL DEFAULT 'draft';

-- Check constraints como alternativa a enums (mas flexibles)
ALTER TABLE public.posts
    ADD CONSTRAINT valid_priority CHECK (priority BETWEEN 0 AND 4);
```

## Row Level Security (RLS)

### Habilitar RLS

```sql
-- SIEMPRE habilitar RLS en tablas con datos de usuario
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.posts ENABLE ROW LEVEL SECURITY;

-- IMPORTANTE: Sin politicas, RLS bloquea TODO el acceso
-- Hay que crear politicas explicitas para cada operacion
```

### Politicas basicas

```sql
-- Profiles: cualquiera puede ver, solo el owner puede editar
CREATE POLICY "Profiles are viewable by everyone"
    ON public.profiles FOR SELECT
    USING (true);

CREATE POLICY "Users can update own profile"
    ON public.profiles FOR UPDATE
    USING (auth.uid() = id)
    WITH CHECK (auth.uid() = id);

-- Posts: ver publicados, owner ve todo
CREATE POLICY "Published posts are viewable by everyone"
    ON public.posts FOR SELECT
    USING (published = TRUE OR auth.uid() = author_id);

CREATE POLICY "Users can create own posts"
    ON public.posts FOR INSERT
    WITH CHECK (auth.uid() = author_id);

CREATE POLICY "Users can update own posts"
    ON public.posts FOR UPDATE
    USING (auth.uid() = author_id)
    WITH CHECK (auth.uid() = author_id);

CREATE POLICY "Users can delete own posts"
    ON public.posts FOR DELETE
    USING (auth.uid() = author_id);
```

### Politicas con roles

```sql
-- Tabla de roles
CREATE TABLE public.user_roles (
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('admin', 'moderator', 'user')),
    PRIMARY KEY (user_id, role)
);

-- Helper function para verificar roles
CREATE OR REPLACE FUNCTION public.has_role(role_name TEXT)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM public.user_roles
        WHERE user_id = auth.uid() AND role = role_name
    );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER STABLE;

-- Politica: admins pueden ver todo
CREATE POLICY "Admins can view all posts"
    ON public.posts FOR SELECT
    USING (public.has_role('admin'));

-- Politica: moderadores pueden actualizar cualquier post
CREATE POLICY "Moderators can update any post"
    ON public.posts FOR UPDATE
    USING (public.has_role('moderator'))
    WITH CHECK (public.has_role('moderator'));
```

### Politicas con organizaciones (multi-tenant)

```sql
CREATE TABLE public.organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE public.org_members (
    org_id UUID REFERENCES public.organizations(id) ON DELETE CASCADE,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'member' CHECK (role IN ('owner', 'admin', 'member')),
    PRIMARY KEY (org_id, user_id)
);

CREATE TABLE public.projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES public.organizations(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE public.projects ENABLE ROW LEVEL SECURITY;

-- Solo miembros de la org pueden ver proyectos
CREATE POLICY "Org members can view projects"
    ON public.projects FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM public.org_members
            WHERE org_id = projects.org_id
            AND user_id = auth.uid()
        )
    );

-- Solo admins/owners de la org pueden crear proyectos
CREATE POLICY "Org admins can create projects"
    ON public.projects FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM public.org_members
            WHERE org_id = projects.org_id
            AND user_id = auth.uid()
            AND role IN ('owner', 'admin')
        )
    );
```

## Migrations

### Crear y ejecutar

```bash
# Crear nueva migration
supabase migration new create_users

# Esto crea: supabase/migrations/20250101120000_create_users.sql
# Editar el archivo con tu SQL

# Ejecutar migrations localmente
supabase db reset      # Reset y re-run todas
supabase db push       # Push a remoto (produccion)

# Ver status de migrations
supabase migration list

# Diff entre local y remoto
supabase db diff
supabase db diff --use-migra    # Usar migra para diff mas preciso
```

### Migration ejemplo

```sql
-- supabase/migrations/20250101000000_create_users.sql

-- Create tables
CREATE TABLE public.profiles (
    id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    username TEXT UNIQUE NOT NULL,
    display_name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;

-- Create policies
CREATE POLICY "Public profiles are viewable by everyone"
    ON public.profiles FOR SELECT
    USING (true);

CREATE POLICY "Users can update own profile"
    ON public.profiles FOR UPDATE
    USING (auth.uid() = id);

-- Create indexes
CREATE INDEX idx_profiles_username ON public.profiles(username);

-- Create trigger
CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON public.profiles
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_updated_at();
```

### Seed data

```sql
-- supabase/seed.sql
-- Se ejecuta con supabase db reset

INSERT INTO public.profiles (id, username, display_name) VALUES
    ('00000000-0000-0000-0000-000000000001', 'admin', 'Admin User'),
    ('00000000-0000-0000-0000-000000000002', 'testuser', 'Test User');
```

## Client-Side (JavaScript/TypeScript)

### Setup del cliente

```typescript
import { createClient } from "@supabase/supabase-js";
import type { Database } from "./types/supabase";

// Tipos generados automaticamente
const supabase = createClient<Database>(
  import.meta.env.PUBLIC_SUPABASE_URL,
  import.meta.env.PUBLIC_SUPABASE_ANON_KEY,
);
```

### Generar tipos

```bash
# Generar tipos TypeScript desde el schema
supabase gen types typescript --project-id <project-id> > src/types/supabase.ts

# O desde la instancia local
supabase gen types typescript --local > src/types/supabase.ts
```

### Queries tipicas

```typescript
// SELECT
const { data: posts, error } = await supabase
  .from("posts")
  .select("*, profiles(username, avatar_url)")
  .eq("published", true)
  .order("created_at", { ascending: false })
  .limit(20);

// INSERT
const { data, error } = await supabase
  .from("posts")
  .insert({
    title: "My Post",
    content: "Content here",
    author_id: user.id,
  })
  .select()
  .single();

// UPDATE
const { data, error } = await supabase
  .from("posts")
  .update({ title: "Updated Title" })
  .eq("id", postId)
  .eq("author_id", user.id)  // Extra safety con RLS
  .select()
  .single();

// DELETE
const { error } = await supabase
  .from("posts")
  .delete()
  .eq("id", postId);

// RPC (llamar funciones SQL)
const { data, error } = await supabase
  .rpc("search_posts", { query: "typescript" });
```

### Auth

```typescript
// Sign up
const { data, error } = await supabase.auth.signUp({
  email: "user@example.com",
  password: "secure-password",
  options: {
    data: { username: "newuser", full_name: "New User" },
  },
});

// Sign in
const { data, error } = await supabase.auth.signInWithPassword({
  email: "user@example.com",
  password: "secure-password",
});

// OAuth
const { data, error } = await supabase.auth.signInWithOAuth({
  provider: "github",
  options: { redirectTo: "http://localhost:4321/auth/callback" },
});

// Get current user
const { data: { user } } = await supabase.auth.getUser();

// Sign out
await supabase.auth.signOut();

// Listen to auth changes
supabase.auth.onAuthStateChange((event, session) => {
  if (event === "SIGNED_IN") {
    console.log("User signed in:", session?.user);
  }
});
```

### Realtime

```typescript
// Subscribirse a cambios en una tabla
const channel = supabase
  .channel("posts-changes")
  .on(
    "postgres_changes",
    {
      event: "*",          // INSERT, UPDATE, DELETE, o *
      schema: "public",
      table: "posts",
      filter: "published=eq.true",
    },
    (payload) => {
      console.log("Change:", payload.eventType, payload.new);
    },
  )
  .subscribe();

// Cleanup
channel.unsubscribe();
```

## Edge Functions

### Crear y desplegar

```bash
# Crear nueva function
supabase functions new process-webhook

# Servir localmente
supabase functions serve

# Desplegar
supabase functions deploy process-webhook
```

### Ejemplo de Edge Function

```typescript
// supabase/functions/process-webhook/index.ts
import { serve } from "https://deno.land/std@0.177.0/http/server.ts";
import { createClient } from "https://esm.sh/@supabase/supabase-js@2";

serve(async (req: Request) => {
  // CORS headers
  if (req.method === "OPTIONS") {
    return new Response(null, {
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods": "POST",
        "Access-Control-Allow-Headers": "authorization, content-type",
      },
    });
  }

  try {
    // Crear cliente con service role (bypass RLS)
    const supabase = createClient(
      Deno.env.get("SUPABASE_URL")!,
      Deno.env.get("SUPABASE_SERVICE_ROLE_KEY")!,
    );

    const payload = await req.json();

    // Verificar auth del usuario
    const authHeader = req.headers.get("Authorization");
    const { data: { user }, error: authError } = await supabase.auth.getUser(
      authHeader?.replace("Bearer ", "") ?? "",
    );

    if (authError || !user) {
      return new Response(JSON.stringify({ error: "Unauthorized" }), {
        status: 401,
        headers: { "Content-Type": "application/json" },
      });
    }

    // Procesar
    const { data, error } = await supabase
      .from("processed_events")
      .insert({ user_id: user.id, payload })
      .select()
      .single();

    if (error) throw error;

    return new Response(JSON.stringify(data), {
      headers: { "Content-Type": "application/json" },
    });
  } catch (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: { "Content-Type": "application/json" },
    });
  }
});
```

## PostgreSQL Performance

### Indices efectivos

```sql
-- Indice para queries frecuentes
CREATE INDEX idx_posts_author_published
    ON public.posts(author_id, published)
    WHERE published = TRUE;

-- Indice GIN para busqueda full-text
ALTER TABLE public.posts ADD COLUMN search_vector tsvector
    GENERATED ALWAYS AS (
        setweight(to_tsvector('spanish', COALESCE(title, '')), 'A') ||
        setweight(to_tsvector('spanish', COALESCE(content, '')), 'B')
    ) STORED;

CREATE INDEX idx_posts_search ON public.posts USING gin(search_vector);

-- Funcion de busqueda
CREATE OR REPLACE FUNCTION public.search_posts(query TEXT)
RETURNS SETOF public.posts AS $$
    SELECT *
    FROM public.posts
    WHERE search_vector @@ plainto_tsquery('spanish', query)
    AND published = TRUE
    ORDER BY ts_rank(search_vector, plainto_tsquery('spanish', query)) DESC
    LIMIT 20;
$$ LANGUAGE sql STABLE;

-- Indice GIN para JSONB
CREATE INDEX idx_profiles_metadata ON public.profiles USING gin(metadata);
```

### Analizar queries

```sql
-- Siempre analizar queries lentas
EXPLAIN ANALYZE
SELECT p.*, pr.username
FROM public.posts p
JOIN public.profiles pr ON p.author_id = pr.id
WHERE p.published = TRUE
ORDER BY p.created_at DESC
LIMIT 20;

-- Ver indices no usados
SELECT schemaname, tablename, indexname, idx_scan
FROM pg_stat_user_indexes
WHERE idx_scan = 0
ORDER BY schemaname, tablename;
```

## Storage

```typescript
// Upload archivo
const { data, error } = await supabase.storage
  .from("avatars")
  .upload(`${user.id}/avatar.png`, file, {
    cacheControl: "3600",
    upsert: true,
  });

// Obtener URL publica
const { data: { publicUrl } } = supabase.storage
  .from("avatars")
  .getPublicUrl(`${user.id}/avatar.png`);

// Storage policies (SQL)
// CREATE POLICY "Users can upload own avatar"
//     ON storage.objects FOR INSERT
//     WITH CHECK (
//         bucket_id = 'avatars'
//         AND auth.uid()::text = (storage.foldername(name))[1]
//     );
```

## Mejores Practicas

### DO

- Habilitar RLS en TODAS las tablas con datos de usuario
- Usar `auth.uid()` en politicas RLS, nunca confiar en el cliente
- Generar tipos TypeScript con `supabase gen types`
- Usar migrations para cambios de schema (nunca SQL directo en prod)
- Crear indices para queries frecuentes y filtros de RLS
- Usar `SECURITY DEFINER` en funciones que necesitan bypass de RLS
- Usar triggers para `updated_at` y profile creation
- Usar enums de Postgres para valores fijos
- Hacer `supabase db reset` localmente para verificar migrations
- Usar `.select().single()` cuando esperas un solo resultado
- Validar datos con constraints SQL ademas de validacion del cliente

### DON'T

- Exponer `service_role` key en el cliente — solo `anon` key en frontend
- Deshabilitar RLS "para que funcione" — crear politicas correctas
- Usar `SECURITY DEFINER` sin necesidad — ejecuta como owner de la funcion
- Hacer queries N+1 — usar joins o `.select("*, relation(*)")`
- Guardar secrets en Edge Functions code — usar Supabase Secrets
- Crear tablas sin `created_at`/`updated_at` — siempre incluirlos
- Usar `text` para valores enumerados — usar enums o check constraints
- Ignorar `EXPLAIN ANALYZE` en queries lentas
- Hacer migrations destructivas sin backup — siempre `pg_dump` antes
- Olvidar politicas DELETE — si habilitas RLS, necesitas politicas para CADA operacion
- Usar `*` en selects de produccion — seleccionar columnas especificas

## Recursos

- [Supabase Docs](https://supabase.com/docs)
- [Supabase CLI Reference](https://supabase.com/docs/reference/cli)
- [PostgREST Docs](https://postgrest.org/)
- [PostgreSQL Docs](https://www.postgresql.org/docs/)
- [Supabase Auth](https://supabase.com/docs/guides/auth)
- [RLS Guide](https://supabase.com/docs/guides/database/postgres/row-level-security)
- [Edge Functions](https://supabase.com/docs/guides/functions)
- [Supabase Community](https://github.com/supabase/supabase/discussions)
