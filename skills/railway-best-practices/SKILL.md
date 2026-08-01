---
name: railway-best-practices
description: Railway deployment best practices — services, env vars, build/deploy config, and databases
version: 1.0.0
tags:
  - best-practices
  - railway
  - deployment
  - devops
  - paas
---

# railway-best-practices

Mejores prácticas para desplegar en Railway: estructura de servicios, variables de
entorno, configuración de build/deploy, bases de datos y observabilidad.

## Overview

Railway es un PaaS para desplegar apps y servicios con poco setup. Encaja con
backends FastAPI/Go, sitios Astro (SSR/estático), workers y bases de datos.

Conceptos:
- **Project**: contenedor lógico de servicios y environments.
- **Service**: una unidad desplegable (tu app, una DB, un worker).
- **Environment**: `production`, `staging`, etc. — variables y deploys separados.

## Build & deploy config

Railway detecta el stack automáticamente (Nixpacks) o usa tu `Dockerfile`.
Preferí configuración versionada con `railway.json` / `railway.toml`:

```json
{
  "$schema": "https://railway.app/railway.schema.json",
  "build": { "builder": "NIXPACKS" },
  "deploy": {
    "startCommand": "uvicorn app.main:app --host 0.0.0.0 --port $PORT",
    "healthcheckPath": "/health",
    "restartPolicyType": "ON_FAILURE"
  }
}
```

- **Usá `$PORT`**: Railway inyecta el puerto; bindeá a `0.0.0.0:$PORT`, nunca a un
  puerto fijo.
- Definí un `healthcheckPath` para que los deploys fallidos no reciban tráfico.
- Para builds reproducibles/control fino, usá un `Dockerfile` propio.

## Variables de entorno

- Configurá secretos en las variables del servicio; **nunca** los commitees.
- Usá **variable references** entre servicios (ej. `DATABASE_URL` desde el plugin de
  Postgres) en vez de copiar valores.
- Variables por environment: distintos secretos en `staging` vs `production`.
- Railway expone `RAILWAY_*` (ej. `RAILWAY_ENVIRONMENT`) para lógica condicional.

```bash
railway variables --set "JWT_SECRET=..." --service api
railway variables            # listar
```

## Bases de datos

- Agregá Postgres/Redis como servicio del proyecto; referenciá su URL desde la app.
- Corré **migraciones** en el arranque o como paso de deploy (release command), no a
  mano en producción.
- Configurá backups; para Supabase Postgres externo, apuntá `DATABASE_URL` a Supabase
  y dejá Railway solo para la app.

## Networking

- **Private networking** entre servicios del mismo proyecto (más rápido y sin exponer
  la DB públicamente).
- Generá un dominio público solo para los servicios que lo necesitan
  (`railway domain`); mantené las DB privadas.

## CLI y CI

```bash
railway login
railway link                 # vincular repo local al proyecto
railway up                   # deploy manual
railway logs --service api   # logs
railway run <cmd>            # correr local con las env del servicio
```

- Deploy automático por push conectando el repo de GitHub (deploy on push).
- En CI, usá un token de proyecto (`RAILWAY_TOKEN`) para deploys no interactivos.

## Observabilidad

- Revisá logs y métricas por servicio; alertas de crash/restart.
- El healthcheck + `restartPolicyType` evita servir tráfico a instancias caídas.

## DO / DON'T

**DO**
- Bindear a `0.0.0.0:$PORT` y definir healthcheck.
- Referenciar variables entre servicios en vez de copiar valores.
- Versionar `railway.json`/`Dockerfile` y separar environments.
- Mantener las bases de datos en private networking.

**DON'T**
- Hardcodear puertos o secretos en el código.
- Correr migraciones manualmente en producción.
- Exponer la base de datos con un dominio público innecesario.
- Compartir el mismo set de variables entre `staging` y `production`.

## Recursos

- [Railway docs](https://docs.railway.com/)
- [Config as code](https://docs.railway.com/reference/config-as-code)
- [Railway CLI](https://docs.railway.com/guides/cli)
- [Private networking](https://docs.railway.com/reference/private-networking)
