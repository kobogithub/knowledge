---
name: fastapi-best-practices
description: FastAPI best practices — project structure, async, Pydantic, dependencies, auth and testing
version: 1.0.0
tags:
  - best-practices
  - fastapi
  - python
  - api
  - async
---

# fastapi-best-practices

Mejores prácticas para construir APIs con FastAPI: estructura de proyecto, modelos
Pydantic, inyección de dependencias, async correcto, autenticación y testing.

## Overview

FastAPI es ideal para:
- **APIs REST** async de alto rendimiento con tipado fuerte
- **Validación automática** de request/response vía Pydantic
- **Documentación** OpenAPI/Swagger generada sola (`/docs`, `/redoc`)
- **Microservicios** y backends que consumen frontends (Astro/HTMX, SPA, móvil)

## Project Structure

```
app/
├── main.py                 # crea la app, incluye routers, middleware
├── core/
│   ├── config.py           # Settings con pydantic-settings
│   └── security.py         # hashing, JWT
├── api/
│   ├── deps.py             # dependencias compartidas (db, current_user)
│   └── routes/
│       ├── users.py
│       └── items.py
├── models/                 # ORM (SQLModel / SQLAlchemy)
├── schemas/                # Pydantic request/response
├── services/               # lógica de negocio (sin FastAPI adentro)
└── db/
    └── session.py
tests/
```

Regla clave: los routers son finos, la lógica vive en `services/`, y `schemas/`
(Pydantic API) se mantiene separado de `models/` (persistencia).

## Configuración con pydantic-settings

```python
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env", extra="ignore")
    database_url: str
    jwt_secret: str
    debug: bool = False

settings = Settings()  # falla al arrancar si falta una var requerida
```

Nunca leas `os.environ` disperso por el código: centralizá en `Settings`.

## Schemas Pydantic

```python
from pydantic import BaseModel, EmailStr, Field

class UserCreate(BaseModel):
    email: EmailStr
    password: str = Field(min_length=8)

class UserOut(BaseModel):
    id: int
    email: EmailStr
    # nunca exponer password/hash en la respuesta
```

- Modelo de entrada ≠ modelo de salida. No reutilices el ORM como response model.
- Usá `response_model=UserOut` en el endpoint para filtrar campos sensibles.

## Async correcto

```python
@router.get("/items")
async def list_items(db: AsyncSession = Depends(get_db)) -> list[ItemOut]:
    return await item_service.list(db)
```

- Si usás un driver **async** (asyncpg, databases), los handlers `async def` y las
  queries deben ser `await`. No mezcles I/O bloqueante en un handler async.
- Para trabajo CPU-bound o librerías síncronas, definí el handler como `def` normal:
  FastAPI lo corre en un threadpool y no bloquea el event loop.

## Inyección de dependencias

```python
async def get_current_user(
    token: str = Depends(oauth2_scheme),
    db: AsyncSession = Depends(get_db),
) -> User:
    user = await auth_service.user_from_token(db, token)
    if user is None:
        raise HTTPException(status_code=401, detail="Invalid credentials")
    return user
```

- Recursos con ciclo de vida (DB session) → dependencias con `yield`.
- Reutilizá dependencias para auth, paginación y filtros comunes.

## Errores

- Lanzá `HTTPException` con `status_code` y `detail` claros; evitá 500 genéricos.
- Registrá un `exception_handler` para tus excepciones de dominio y mapealas a
  respuestas HTTP consistentes.

## Testing

```python
from fastapi.testclient import TestClient

def test_create_user(client: TestClient):
    r = client.post("/users", json={"email": "a@b.com", "password": "secret123"})
    assert r.status_code == 201
    assert "password" not in r.json()
```

- Usá `TestClient` (o `httpx.AsyncClient` para tests async).
- Sobreescribí dependencias con `app.dependency_overrides` para DB de test.

## DO / DON'T

**DO**
- Separar `schemas/` (API) de `models/` (DB) y lógica en `services/`.
- Tipar handlers con `-> ResponseModel` y usar `response_model`.
- Centralizar config en `Settings`; validar al arranque.
- Versionar la API (`/api/v1`).

**DON'T**
- Poner lógica de negocio dentro de los routers.
- Devolver el modelo ORM crudo (fuga de campos sensibles).
- Bloquear el event loop con I/O síncrono en handlers `async`.
- Guardar secretos en el código: usá variables de entorno.

## Recursos

- [FastAPI docs](https://fastapi.tiangolo.com/)
- [Pydantic v2](https://docs.pydantic.dev/latest/)
- [SQLModel](https://sqlmodel.tiangolo.com/)
- [full-stack-fastapi-template](https://github.com/fastapi/full-stack-fastapi-template)
