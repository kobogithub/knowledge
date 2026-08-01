---
name: htmx-best-practices
description: HTMX best practices — hypermedia-driven UIs, partial rendering, swaps, and progressive enhancement
version: 1.0.0
tags:
  - best-practices
  - htmx
  - hypermedia
  - frontend
  - web
---

# htmx-best-practices

Mejores prácticas para construir UIs con HTMX: hipermedia como motor de estado,
fragmentos de HTML desde el servidor, swaps, y mejora progresiva. Encaja con
backends que renderizan HTML (FastAPI + Jinja, Astro endpoints, etc.).

## Overview

HTMX permite interactividad rica **sin SPA**: el servidor devuelve fragmentos de
HTML y HTMX los inserta en el DOM. El estado vive en el HTML (hipermedia), no en un
store de JS.

Ideal para:
- Apps CRUD, dashboards internos, formularios dinámicos
- Reducir JavaScript y complejidad de build
- Backends que ya renderizan HTML (FastAPI/Jinja, Astro)

## Principios

1. **El servidor devuelve HTML, no JSON.** Los endpoints HTMX renderizan fragmentos.
2. **El HTML es la fuente de estado.** Evitá duplicar estado en JS.
3. **Mejora progresiva.** La página debe funcionar (o degradar con dignidad) sin JS.

## Atributos core

```html
<!-- Cargar un fragmento al hacer click y reemplazar el contenido -->
<button hx-get="/items?page=2"
        hx-target="#list"
        hx-swap="innerHTML">
  Cargar más
</button>

<div id="list">...</div>
```

- `hx-get|post|put|patch|delete`: verbo + URL.
- `hx-target`: dónde va la respuesta (selector CSS).
- `hx-swap`: cómo se inserta (`innerHTML`, `outerHTML`, `beforeend`, `delete`…).
- `hx-trigger`: qué evento dispara (`click`, `submit`, `keyup changed delay:300ms`,
  `revealed`, `load`).

## Patrones útiles

### Formulario con validación server-side

```html
<form hx-post="/users" hx-target="#form-wrapper" hx-swap="outerHTML">
  <input name="email" required>
  <button type="submit">Crear</button>
</form>
```
El servidor devuelve el mismo formulario con errores, o el estado de éxito.

### Búsqueda activa (active search)

```html
<input type="search" name="q"
       hx-get="/search" hx-target="#results"
       hx-trigger="input changed delay:300ms">
```

### Actualizar varias zonas — Out of Band swaps

```html
<!-- respuesta del servidor -->
<div id="cart-count" hx-swap-oob="true">3</div>
<div id="main">...contenido principal...</div>
```

## Integración con el backend

- Detectá peticiones HTMX con el header `HX-Request: true` para devolver el
  **fragmento** en vez de la página completa.
- Usá headers de respuesta HTMX: `HX-Redirect`, `HX-Trigger` (disparar eventos JS),
  `HX-Retarget`, `HX-Reswap`.
- Devolvé el status HTTP correcto: `422` para errores de validación re-renderiza el
  form; `204` cuando no hay contenido que insertar.

```python
# FastAPI
@router.get("/items")
def items(request: Request):
    template = "items_fragment.html" if request.headers.get("HX-Request") else "items_page.html"
    return templates.TemplateResponse(template, {...})
```

## Seguridad

- **CSRF**: incluí el token en un `hx-headers` o hidden input; los POST/PUT/DELETE lo
  requieren igual que un form normal.
- **Escapá** siempre el HTML del servidor (autoescape de Jinja on).
- No confíes en atributos del cliente: validá en el servidor.

## Performance

- Fragmentos chicos y cacheables. Evitá devolver la página entera en cada swap.
- Usá `hx-trigger` con `delay:` para debounce en inputs.
- `hx-boost` para navegación tipo SPA sobre links/forms existentes, con degradación.

## DO / DON'T

**DO**
- Devolver HTML server-rendered; mantener el estado en el DOM.
- Distinguir fragmento vs página completa con `HX-Request`.
- Escapar salida y proteger CSRF en mutaciones.
- Usar `hx-swap-oob` para actualizaciones multi-zona.

**DON'T**
- Construir un store de estado en JS paralelo al HTML.
- Devolver JSON a un endpoint HTMX (rompe el modelo hipermedia).
- Olvidar la mejora progresiva / degradación sin JS.
- Insertar HTML sin escapar (XSS).

## Recursos

- [htmx.org docs](https://htmx.org/docs/)
- [Hypermedia Systems (libro gratis)](https://hypermedia.systems/)
- [htmx examples](https://htmx.org/examples/)
