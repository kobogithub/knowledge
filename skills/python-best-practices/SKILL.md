---
name: python-best-practices
description: Python development best practices, project structure and modern tooling
version: 1.0.0
tags:
  - best-practices
  - python
  - fastapi
  - testing
  - packaging
---

# python-best-practices

Mejores practicas para desarrollo en Python: estructura de proyecto, typing, tooling moderno, testing y patrones robustos.

## Overview

Python es el lenguaje principal para:
- **APIs**: FastAPI, Django, Flask
- **Scripting**: Automatizacion, data processing, ETL
- **CLI tools**: Click, Typer, argparse
- **Data science**: Pandas, NumPy, ML pipelines
- **DevOps**: Ansible, scripts de infra, Boto3

## Project Structure

### Estructura estandar con src layout

```
project/
├── src/
│   └── mypackage/
│       ├── __init__.py
│       ├── main.py
│       ├── config.py
│       ├── models/
│       │   ├── __init__.py
│       │   └── user.py
│       ├── services/
│       │   ├── __init__.py
│       │   └── auth.py
│       ├── api/
│       │   ├── __init__.py
│       │   ├── routes/
│       │   │   ├── __init__.py
│       │   │   └── users.py
│       │   └── dependencies.py
│       └── utils/
│           ├── __init__.py
│           └── helpers.py
├── tests/
│   ├── conftest.py
│   ├── unit/
│   │   └── test_auth.py
│   └── integration/
│       └── test_api.py
├── pyproject.toml
├── uv.lock
├── .python-version
└── .env.example
```

### Por que src layout

```toml
# pyproject.toml — el src layout evita importar el paquete sin instalar
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "mypackage"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "fastapi>=0.115",
    "uvicorn[standard]>=0.30",
    "pydantic>=2.0",
    "pydantic-settings>=2.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=8.0",
    "pytest-cov>=5.0",
    "pytest-asyncio>=0.24",
    "ruff>=0.8",
    "mypy>=1.13",
    "pre-commit>=4.0",
]
```

## Package Management con uv

### Instalacion y uso basico

```bash
# Instalar uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# Crear proyecto nuevo
uv init myproject
cd myproject

# Agregar dependencias
uv add fastapi uvicorn pydantic
uv add --dev pytest ruff mypy

# Instalar todas las dependencias
uv sync

# Ejecutar comandos dentro del venv
uv run python -m mypackage.main
uv run pytest
uv run ruff check .

# Lockfile — SIEMPRE commitear uv.lock
uv lock
```

### Version de Python

```bash
# Fijar version de Python
uv python pin 3.12

# Instalar una version especifica
uv python install 3.12

# .python-version se crea automaticamente
cat .python-version
# 3.12
```

## Type Hints

### Typing moderno (Python 3.10+)

```python
# Usar built-in types, NO importar de typing
# Bien (3.10+)
def process_items(items: list[str]) -> dict[str, int]:
    return {item: len(item) for item in items}

# Mal (legacy)
from typing import List, Dict
def process_items(items: List[str]) -> Dict[str, int]: ...

# Union types con | (3.10+)
def find_user(user_id: int) -> User | None:
    ...

# Mal (legacy)
from typing import Optional, Union
def find_user(user_id: int) -> Optional[User]: ...
```

### Patterns comunes de typing

```python
from collections.abc import Sequence, Mapping, Callable, AsyncIterator
from typing import TypeVar, Protocol, Any, TypeAlias

# TypeAlias para tipos complejos
UserId: TypeAlias = int
Headers: TypeAlias = dict[str, str]

# TypeVar para generics
T = TypeVar("T")

def first_or_none(items: Sequence[T]) -> T | None:
    return items[0] if items else None

# Protocol para structural subtyping (duck typing con tipos)
class Renderable(Protocol):
    def render(self) -> str: ...

def display(item: Renderable) -> None:
    print(item.render())

# Callable types
Handler = Callable[[str, int], bool]

# Self type (3.11+)
from typing import Self

class Builder:
    def with_name(self, name: str) -> Self:
        self.name = name
        return self
```

### Typing en clases

```python
from dataclasses import dataclass, field
from enum import StrEnum

# StrEnum (3.11+) — valores son strings automaticamente
class Status(StrEnum):
    ACTIVE = "active"
    INACTIVE = "inactive"
    SUSPENDED = "suspended"

# Dataclasses — preferir sobre NamedTuple para mutabilidad
@dataclass(frozen=True, slots=True)
class Config:
    host: str
    port: int = 8000
    debug: bool = False
    tags: list[str] = field(default_factory=list)

# frozen=True: inmutable (hashable, usable en sets/dicts)
# slots=True: mas eficiente en memoria, mas rapido
```

## Pydantic

### Modelos y validacion

```python
from pydantic import BaseModel, Field, field_validator, model_validator
from pydantic import EmailStr, HttpUrl
from datetime import datetime

class UserCreate(BaseModel):
    """Schema para crear un usuario."""
    model_config = {"strict": True}

    name: str = Field(min_length=1, max_length=100)
    email: EmailStr
    age: int = Field(ge=0, le=150)
    tags: list[str] = Field(default_factory=list, max_length=10)

    @field_validator("name")
    @classmethod
    def name_must_be_trimmed(cls, v: str) -> str:
        stripped = v.strip()
        if not stripped:
            raise ValueError("name cannot be empty or whitespace")
        return stripped

class UserResponse(BaseModel):
    """Schema de respuesta — nunca exponer campos internos."""
    id: int
    name: str
    email: EmailStr
    created_at: datetime

    model_config = {"from_attributes": True}
```

### Settings con pydantic-settings

```python
from pydantic_settings import BaseSettings, SettingsConfigDict
from pydantic import SecretStr

class Settings(BaseSettings):
    """Configuracion de la aplicacion — carga de env vars y .env."""
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        case_sensitive=False,
    )

    # Database
    database_url: str
    db_pool_size: int = 5

    # Auth
    secret_key: SecretStr
    access_token_expire_minutes: int = 30

    # App
    debug: bool = False
    environment: str = "development"
    log_level: str = "INFO"

# Singleton pattern
from functools import lru_cache

@lru_cache
def get_settings() -> Settings:
    return Settings()
```

## FastAPI Patterns

### Estructura de app

```python
# src/mypackage/main.py
from contextlib import asynccontextmanager
from fastapi import FastAPI
from mypackage.api.routes import users, health
from mypackage.config import get_settings

@asynccontextmanager
async def lifespan(app: FastAPI):
    """Startup y shutdown logic."""
    # Startup
    settings = get_settings()
    # init db pool, cache, etc.
    yield
    # Shutdown
    # close connections, flush buffers

def create_app() -> FastAPI:
    settings = get_settings()
    app = FastAPI(
        title="My API",
        version="0.1.0",
        lifespan=lifespan,
        docs_url="/docs" if settings.debug else None,
    )
    app.include_router(health.router)
    app.include_router(users.router, prefix="/api/v1")
    return app

app = create_app()
```

### Dependencies y error handling

```python
# src/mypackage/api/dependencies.py
from typing import Annotated
from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials

security = HTTPBearer()

async def get_current_user(
    credentials: Annotated[
        HTTPAuthorizationCredentials, Depends(security)
    ],
) -> User:
    token = credentials.credentials
    user = await verify_token(token)
    if user is None:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid or expired token",
        )
    return user

# Usar Annotated para DRY
CurrentUser = Annotated[User, Depends(get_current_user)]

# En el router
@router.get("/me")
async def get_profile(user: CurrentUser) -> UserResponse:
    return UserResponse.model_validate(user)
```

### Custom exceptions

```python
# src/mypackage/exceptions.py
from fastapi import Request
from fastapi.responses import JSONResponse

class AppError(Exception):
    """Base application error."""
    def __init__(self, message: str, status_code: int = 500):
        self.message = message
        self.status_code = status_code

class NotFoundError(AppError):
    def __init__(self, resource: str, id: int | str):
        super().__init__(
            message=f"{resource} with id {id} not found",
            status_code=404,
        )

class ConflictError(AppError):
    def __init__(self, message: str):
        super().__init__(message=message, status_code=409)

# Registrar handler en la app
async def app_error_handler(request: Request, exc: AppError) -> JSONResponse:
    return JSONResponse(
        status_code=exc.status_code,
        content={"detail": exc.message},
    )

# En create_app()
app.add_exception_handler(AppError, app_error_handler)
```

## Async Patterns

### Asyncio correcto

```python
import asyncio
from collections.abc import AsyncIterator

# Gather para concurrencia
async def fetch_all_users(user_ids: list[int]) -> list[User]:
    tasks = [fetch_user(uid) for uid in user_ids]
    return await asyncio.gather(*tasks)

# TaskGroup (3.11+) — mejor que gather, cancela en error
async def fetch_all_users_safe(user_ids: list[int]) -> list[User]:
    results: list[User] = []
    async with asyncio.TaskGroup() as tg:
        for uid in user_ids:
            tg.create_task(fetch_user(uid))
    return results

# Async context manager
from contextlib import asynccontextmanager

@asynccontextmanager
async def db_transaction() -> AsyncIterator[Connection]:
    conn = await get_connection()
    try:
        yield conn
        await conn.commit()
    except Exception:
        await conn.rollback()
        raise
    finally:
        await conn.close()

# Async generator
async def stream_results(query: str) -> AsyncIterator[dict]:
    async with db_transaction() as conn:
        async for row in conn.execute(query):
            yield dict(row)
```

### Evitar bloqueo del event loop

```python
import asyncio
from functools import partial

# NUNCA ejecutar operaciones bloqueantes en async
# Mal
async def bad_read_file(path: str) -> str:
    with open(path) as f:  # BLOQUEA el event loop!
        return f.read()

# Bien — usar run_in_executor para I/O bloqueante
async def read_file(path: str) -> str:
    loop = asyncio.get_running_loop()
    return await loop.run_in_executor(
        None,  # default ThreadPoolExecutor
        partial(open(path).read),
    )

# Mejor — usar aiofiles
import aiofiles

async def read_file_async(path: str) -> str:
    async with aiofiles.open(path) as f:
        return await f.read()
```

## Error Handling

### Patron estandar

```python
import logging
from contextlib import suppress

logger = logging.getLogger(__name__)

# Capturar excepciones especificas, NUNCA bare except
try:
    result = process_data(payload)
except ValueError as e:
    logger.warning("Invalid data: %s", e)
    raise AppError(f"Invalid input: {e}") from e
except ConnectionError as e:
    logger.error("Connection failed: %s", e)
    raise AppError("Service unavailable", status_code=503) from e
# NUNCA hacer esto:
# except Exception: pass
# except: pass

# suppress para ignorar excepciones esperadas
with suppress(FileNotFoundError):
    os.remove(temp_file)

# Re-raise con cadena de causa (from e)
try:
    user = db.get_user(user_id)
except DatabaseError as e:
    raise NotFoundError("User", user_id) from e
```

### Logging correcto

```python
import logging
import structlog  # pip install structlog (recomendado)

# Configuracion basica
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
)

logger = logging.getLogger(__name__)

# Usar lazy formatting (% style), NO f-strings en logging
# Bien — lazy, no se formatea si el nivel no aplica
logger.info("Processing user %s with %d items", user_id, len(items))

# Mal — f-string se evalua SIEMPRE, incluso si el nivel no aplica
logger.info(f"Processing user {user_id} with {len(items)} items")

# Excepciones con exc_info
try:
    result = risky_operation()
except Exception:
    logger.exception("Operation failed")  # Incluye traceback automatico

# Structlog para logging estructurado (recomendado en produccion)
log = structlog.get_logger()
log.info("user_created", user_id=user.id, email=user.email)
```

## Testing con pytest

### Estructura y fixtures

```python
# tests/conftest.py
import pytest
from httpx import AsyncClient, ASGITransport
from mypackage.main import create_app
from mypackage.config import Settings

@pytest.fixture
def settings() -> Settings:
    """Settings override para tests."""
    return Settings(
        database_url="sqlite+aiosqlite:///test.db",
        secret_key="test-secret-key-not-for-production",
        debug=True,
    )

@pytest.fixture
async def client(settings: Settings) -> AsyncClient:
    """HTTP client para integration tests."""
    app = create_app()
    app.dependency_overrides[get_settings] = lambda: settings
    transport = ASGITransport(app=app)
    async with AsyncClient(transport=transport, base_url="http://test") as ac:
        yield ac
```

### Tests con parametrize

```python
import pytest
from mypackage.services.auth import hash_password, verify_password

class TestPasswordHashing:
    def test_hash_and_verify(self):
        password = "secure-password-123"
        hashed = hash_password(password)
        assert verify_password(password, hashed) is True

    def test_different_passwords_different_hashes(self):
        h1 = hash_password("password1")
        h2 = hash_password("password2")
        assert h1 != h2

    @pytest.mark.parametrize("invalid_password", [
        "",
        " ",
        "short",
        "a" * 201,
    ])
    def test_invalid_passwords(self, invalid_password: str):
        with pytest.raises(ValueError, match="Invalid password"):
            hash_password(invalid_password)
```

### Async tests

```python
import pytest
from httpx import AsyncClient

@pytest.mark.asyncio
async def test_create_user(client: AsyncClient):
    response = await client.post("/api/v1/users", json={
        "name": "Test User",
        "email": "test@example.com",
        "age": 25,
    })
    assert response.status_code == 201
    data = response.json()
    assert data["name"] == "Test User"
    assert data["email"] == "test@example.com"
    assert "id" in data

@pytest.mark.asyncio
async def test_get_user_not_found(client: AsyncClient):
    response = await client.get("/api/v1/users/99999")
    assert response.status_code == 404
    assert "not found" in response.json()["detail"].lower()
```

### Mocking

```python
from unittest.mock import AsyncMock, patch, MagicMock

@pytest.mark.asyncio
async def test_send_email_on_registration(client: AsyncClient):
    with patch("mypackage.services.email.send_welcome_email") as mock_send:
        mock_send.return_value = None
        response = await client.post("/api/v1/users", json={
            "name": "New User",
            "email": "new@example.com",
            "age": 30,
        })
        assert response.status_code == 201
        mock_send.assert_called_once_with("new@example.com", "New User")

# Async mock
async def test_external_api():
    mock_client = AsyncMock()
    mock_client.get.return_value = MagicMock(
        status_code=200,
        json=lambda: {"data": "value"},
    )
    service = ExternalService(client=mock_client)
    result = await service.fetch_data()
    assert result == {"data": "value"}
```

### Coverage

```bash
# pytest con coverage
uv run pytest --cov=src/mypackage --cov-report=term-missing --cov-report=html

# pyproject.toml config
[tool.pytest.ini_options]
testpaths = ["tests"]
asyncio_mode = "auto"
addopts = "-v --tb=short"

[tool.coverage.run]
source = ["src/mypackage"]
omit = ["*/tests/*", "*/__main__.py"]

[tool.coverage.report]
fail_under = 80
show_missing = true
exclude_lines = [
    "pragma: no cover",
    "if TYPE_CHECKING:",
    "if __name__ == .__main__.",
]
```

## Linting y Formatting con Ruff

### Configuracion

```toml
# pyproject.toml
[tool.ruff]
target-version = "py312"
line-length = 88
src = ["src"]

[tool.ruff.lint]
select = [
    "E",    # pycodestyle errors
    "W",    # pycodestyle warnings
    "F",    # pyflakes
    "I",    # isort
    "N",    # pep8-naming
    "UP",   # pyupgrade
    "B",    # flake8-bugbear
    "S",    # flake8-bandit (security)
    "A",    # flake8-builtins
    "C4",   # flake8-comprehensions
    "DTZ",  # flake8-datetimez
    "T20",  # flake8-print
    "SIM",  # flake8-simplify
    "RUF",  # ruff-specific rules
]
ignore = [
    "S101",  # allow assert in tests
]

[tool.ruff.lint.per-file-ignores]
"tests/**/*.py" = ["S101", "S106"]  # allow assert and hardcoded passwords in tests

[tool.ruff.lint.isort]
known-first-party = ["mypackage"]
```

### Comandos

```bash
# Check
uv run ruff check .

# Fix automatico
uv run ruff check --fix .

# Format (reemplazo de black)
uv run ruff format .

# Check format sin modificar
uv run ruff format --check .
```

## Type Checking con mypy

### Configuracion

```toml
# pyproject.toml
[tool.mypy]
python_version = "3.12"
strict = true
warn_return_any = true
warn_unused_configs = true
disallow_untyped_defs = true
disallow_any_generics = true
check_untyped_defs = true

[[tool.mypy.overrides]]
module = "tests.*"
disallow_untyped_defs = false

[[tool.mypy.overrides]]
module = "migrations.*"
ignore_errors = true
```

### Comandos

```bash
# Ejecutar
uv run mypy src/

# Con reporte
uv run mypy src/ --html-report mypy-report
```

## Patrones Comunes

### Context managers

```python
from contextlib import contextmanager, asynccontextmanager
from collections.abc import Iterator, AsyncIterator
import time

@contextmanager
def timer(label: str) -> Iterator[None]:
    """Medir tiempo de ejecucion."""
    start = time.perf_counter()
    try:
        yield
    finally:
        elapsed = time.perf_counter() - start
        logger.info("%s took %.3fs", label, elapsed)

# Uso
with timer("database_query"):
    results = db.execute(query)

# Async version
@asynccontextmanager
async def managed_connection(url: str) -> AsyncIterator[Connection]:
    conn = await connect(url)
    try:
        yield conn
    finally:
        await conn.close()
```

### Retry decorator

```python
import asyncio
from functools import wraps
from collections.abc import Callable
from typing import TypeVar, ParamSpec

P = ParamSpec("P")
R = TypeVar("R")

def retry(
    max_attempts: int = 3,
    delay: float = 1.0,
    backoff: float = 2.0,
    exceptions: tuple[type[Exception], ...] = (Exception,),
) -> Callable[[Callable[P, R]], Callable[P, R]]:
    """Retry decorator con backoff exponencial."""
    def decorator(func: Callable[P, R]) -> Callable[P, R]:
        @wraps(func)
        async def async_wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
            last_exception: Exception | None = None
            current_delay = delay
            for attempt in range(1, max_attempts + 1):
                try:
                    return await func(*args, **kwargs)
                except exceptions as e:
                    last_exception = e
                    if attempt == max_attempts:
                        break
                    logger.warning(
                        "Attempt %d/%d failed: %s. Retrying in %.1fs",
                        attempt, max_attempts, e, current_delay,
                    )
                    await asyncio.sleep(current_delay)
                    current_delay *= backoff
            raise last_exception  # type: ignore[misc]
        return async_wrapper  # type: ignore[return-value]
    return decorator

# Uso
@retry(max_attempts=3, exceptions=(ConnectionError, TimeoutError))
async def fetch_data(url: str) -> dict:
    async with httpx.AsyncClient() as client:
        response = await client.get(url)
        response.raise_for_status()
        return response.json()
```

### Repository pattern

```python
from abc import ABC, abstractmethod

class UserRepository(ABC):
    """Interface para acceso a datos de usuarios."""

    @abstractmethod
    async def get_by_id(self, user_id: int) -> User | None: ...

    @abstractmethod
    async def create(self, user: UserCreate) -> User: ...

    @abstractmethod
    async def update(self, user_id: int, data: UserUpdate) -> User: ...

    @abstractmethod
    async def delete(self, user_id: int) -> bool: ...

class PostgresUserRepository(UserRepository):
    def __init__(self, pool: asyncpg.Pool) -> None:
        self._pool = pool

    async def get_by_id(self, user_id: int) -> User | None:
        async with self._pool.acquire() as conn:
            row = await conn.fetchrow(
                "SELECT * FROM users WHERE id = $1", user_id
            )
            return User(**dict(row)) if row else None

    async def create(self, user: UserCreate) -> User:
        async with self._pool.acquire() as conn:
            row = await conn.fetchrow(
                """INSERT INTO users (name, email, age)
                   VALUES ($1, $2, $3) RETURNING *""",
                user.name, user.email, user.age,
            )
            return User(**dict(row))
    # ... etc
```

## Seguridad

### Practicas basicas

```python
# NUNCA hardcodear secrets
# Mal
API_KEY = "sk-1234567890abcdef"

# Bien — usar variables de entorno via pydantic-settings
settings = get_settings()
api_key = settings.secret_key.get_secret_value()

# SQL injection — SIEMPRE usar parametros
# Mal — vulnerable a SQL injection
query = f"SELECT * FROM users WHERE name = '{name}'"

# Bien — query parametrizado
query = "SELECT * FROM users WHERE name = $1"
await conn.fetch(query, name)

# Path traversal — validar paths
from pathlib import Path

def safe_read(base_dir: Path, filename: str) -> str:
    """Lee un archivo asegurando que esta dentro de base_dir."""
    filepath = (base_dir / filename).resolve()
    if not filepath.is_relative_to(base_dir.resolve()):
        raise ValueError("Path traversal detected")
    return filepath.read_text()

# Hashing de passwords — NUNCA md5/sha256
from passlib.context import CryptContext

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

def hash_password(password: str) -> str:
    return pwd_context.hash(password)

def verify_password(plain: str, hashed: str) -> bool:
    return pwd_context.verify(plain, hashed)
```

## Pre-commit Hooks

### Configuracion

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.8.0
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format

  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.13.0
    hooks:
      - id: mypy
        additional_dependencies:
          - pydantic>=2.0
          - fastapi>=0.115

  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v5.0.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml
      - id: check-added-large-files
      - id: debug-statements
```

```bash
# Instalar hooks
uv run pre-commit install

# Ejecutar en todos los archivos
uv run pre-commit run --all-files
```

## CI con GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v4
        with:
          version: "latest"

      - name: Install dependencies
        run: uv sync --all-extras

      - name: Lint
        run: uv run ruff check .

      - name: Format check
        run: uv run ruff format --check .

      - name: Type check
        run: uv run mypy src/

      - name: Tests
        run: uv run pytest --cov --cov-report=xml

      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          file: coverage.xml
```

## Mejores Practicas

### DO

- Usar `uv` para package management (rapido, lockfile, reproducible)
- Usar type hints en TODAS las funciones publicas
- Usar `ruff` para lint + format (reemplaza flake8, isort, black)
- Usar `mypy --strict` para type checking
- Usar `pydantic` para validacion de datos y settings
- Usar `src/` layout para packages
- Usar `pytest` con fixtures y parametrize
- Usar `structlog` o logging lazy (`%s` style) en produccion
- Usar `dataclass(frozen=True, slots=True)` para value objects
- Usar `Annotated[T, Depends(...)]` en FastAPI para DRY
- Commitear `uv.lock` al repositorio
- Usar `from __future__ import annotations` si soportas <3.10

### DON'T

- Usar `pip` directamente — usar `uv` o `pip-tools`
- Usar `typing.Optional`/`typing.Union` — usar `X | None` (3.10+)
- Usar `typing.List`/`typing.Dict` — usar `list`/`dict` builtins
- Hacer `from typing import *` — importar solo lo necesario
- Usar bare `except:` o `except Exception: pass`
- Usar f-strings en `logger.info()` — usar `%s` lazy formatting
- Hardcodear secrets en el codigo — usar env vars
- Ignorar type errors con `# type: ignore` sin justificacion
- Usar `os.path` — preferir `pathlib.Path`
- Retornar `dict` desde APIs — usar Pydantic models
- Mezclar sync y async sin `run_in_executor`
- Usar `requirements.txt` sin lockfile — usar `uv.lock`

## Recursos

- [Python Docs](https://docs.python.org/3/)
- [FastAPI](https://fastapi.tiangolo.com/)
- [Pydantic v2](https://docs.pydantic.dev/latest/)
- [uv](https://docs.astral.sh/uv/)
- [Ruff](https://docs.astral.sh/ruff/)
- [mypy](https://mypy.readthedocs.io/)
- [pytest](https://docs.pytest.org/)
- [Real Python](https://realpython.com/)
- [Python Type Hints Cheat Sheet](https://mypy.readthedocs.io/en/stable/cheat_sheet_py3.html)
