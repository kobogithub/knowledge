---
name: python-best-practices
description: Python best practices for modern application development
version: 1.0.0
author: Knowledge Framework
tags: [python, backend, scripting, data]
---

# Python Best Practices

Expert guidelines for writing clean, maintainable, and performant Python code.

## Core Principles

### 1. Code Style (Critical)

**Follow PEP 8**
```python
# ✅ GOOD: Snake case, clear naming
def calculate_total_price(items: list[Item]) -> Decimal:
    return sum(item.price for item in items)

# ❌ BAD: camelCase, unclear
def calcTotalPrc(i):
    return sum(x.p for x in i)
```

**Use Type Hints**
```python
from typing import Optional, List, Dict

def process_data(
    items: List[str],
    config: Dict[str, Any],
    timeout: Optional[int] = None
) -> Dict[str, int]:
    """Process items according to config."""
    result: Dict[str, int] = {}
    # ...
    return result
```

### 2. Error Handling (Critical)

```python
# ✅ GOOD: Specific exceptions
try:
    with open('file.txt') as f:
        data = json.load(f)
except FileNotFoundError:
    logger.error("File not found")
    raise
except json.JSONDecodeError as e:
    logger.error(f"Invalid JSON: {e}")
    return None

# ❌ BAD: Bare except
try:
    data = json.load(open('file.txt'))
except:
    pass
```

### 3. Context Managers (High)

```python
# ✅ GOOD: Automatic cleanup
with open('file.txt', 'w') as f:
    f.write(data)

# Custom context manager
from contextlib import contextmanager

@contextmanager
def database_connection(url: str):
    conn = create_connection(url)
    try:
        yield conn
    finally:
        conn.close()

with database_connection('postgresql://...') as conn:
    cursor = conn.execute("SELECT * FROM users")
```

### 4. List/Dict Comprehensions (High)

```python
# ✅ GOOD: Comprehensions
squares = [x**2 for x in range(10) if x % 2 == 0]
lookup = {user.id: user.name for user in users}

# ✅ Generator for large datasets
large_data = (process(item) for item in huge_list)

# ❌ BAD: Unnecessary loops
squares = []
for x in range(10):
    if x % 2 == 0:
        squares.append(x**2)
```

### 5. Functions & Decorators (High)

```python
from functools import wraps
import time

def retry(max_attempts: int = 3, delay: float = 1.0):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            for attempt in range(max_attempts):
                try:
                    return func(*args, **kwargs)
                except Exception as e:
                    if attempt == max_attempts - 1:
                        raise
                    time.sleep(delay)
            return None
        return wrapper
    return decorator

@retry(max_attempts=5, delay=2.0)
def fetch_data(url: str) -> dict:
    response = requests.get(url)
    response.raise_for_status()
    return response.json()
```

### 6. Classes & Data Classes (High)

```python
from dataclasses import dataclass, field
from datetime import datetime

@dataclass
class User:
    id: int
    name: str
    email: str
    created_at: datetime = field(default_factory=datetime.now)
    roles: list[str] = field(default_factory=list)
    
    def is_admin(self) -> bool:
        return 'admin' in self.roles

# Pydantic for validation
from pydantic import BaseModel, EmailStr, validator

class UserCreate(BaseModel):
    name: str
    email: EmailStr
    age: int
    
    @validator('age')
    def validate_age(cls, v):
        if v < 18:
            raise ValueError('Must be 18 or older')
        return v
```

### 7. Async/Await (High)

```python
import asyncio
import aiohttp

async def fetch_multiple(urls: list[str]) -> list[dict]:
    async with aiohttp.ClientSession() as session:
        tasks = [fetch_one(session, url) for url in urls]
        return await asyncio.gather(*tasks)

async def fetch_one(session: aiohttp.ClientSession, url: str) -> dict:
    async with session.get(url) as response:
        return await response.json()

# Run
results = asyncio.run(fetch_multiple(urls))
```

### 8. Testing (High)

```python
import pytest
from unittest.mock import Mock, patch

@pytest.fixture
def sample_user():
    return User(id=1, name="Test", email="test@example.com")

def test_user_creation(sample_user):
    assert sample_user.id == 1
    assert sample_user.name == "Test"

@pytest.mark.parametrize("input,expected", [
    (2, 4),
    (3, 9),
    (4, 16),
])
def test_square(input, expected):
    assert square(input) == expected

@patch('requests.get')
def test_api_call(mock_get):
    mock_get.return_value.json.return_value = {'status': 'ok'}
    result = fetch_data('https://api.example.com')
    assert result['status'] == 'ok'
```

### 9. Virtual Environments & Dependencies (Critical)

```bash
# Create venv
python -m venv .venv
source .venv/bin/activate  # Linux/Mac
.venv\Scripts\activate     # Windows

# Install dependencies
pip install -r requirements.txt

# Generate requirements
pip freeze > requirements.txt

# Better: Use pyproject.toml
[project]
name = "myapp"
version = "0.1.0"
dependencies = [
    "fastapi>=0.109.0",
    "uvicorn[standard]>=0.27.0",
    "pydantic>=2.5.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.4.0",
    "black>=23.12.0",
    "ruff>=0.1.9",
]
```

### 10. Performance (Medium)

**Use Built-ins**
```python
# ✅ GOOD: Built-in sum
total = sum(numbers)

# ❌ BAD: Manual loop
total = 0
for n in numbers:
    total += n

# ✅ Use list() for itertools
from itertools import chain
combined = list(chain(list1, list2, list3))
```

**Generators for Large Data**
```python
def read_large_file(filename: str):
    with open(filename) as f:
        for line in f:
            yield line.strip()

# Memory-efficient processing
for line in read_large_file('huge.txt'):
    process(line)
```

## Common Pitfalls

❌ **Mutable default arguments**: `def func(items=[]):`  
✅ Use `None`: `def func(items=None): items = items or []`

❌ **Bare `except:`**: Catches everything  
✅ Catch specific exceptions

❌ **Global variables**: Hard to test/maintain  
✅ Use parameters and return values

❌ **`import *`**: Namespace pollution  
✅ Import explicitly: `from module import Class`

❌ **Not using virtual environments**: Dependency conflicts  
✅ Always use `venv` or `poetry`

## Essential Tools

```bash
# Formatting
black .
isort .

# Linting
ruff check .
pylint myapp/

# Type checking
mypy myapp/

# Testing
pytest tests/ -v --cov=myapp

# Security
bandit -r myapp/
safety check
```

## FastAPI Example

```python
from fastapi import FastAPI, HTTPException, Depends
from pydantic import BaseModel
from typing import List

app = FastAPI()

class Item(BaseModel):
    id: int
    name: str
    price: float

items_db: List[Item] = []

@app.post("/items/", response_model=Item)
async def create_item(item: Item):
    items_db.append(item)
    return item

@app.get("/items/{item_id}", response_model=Item)
async def get_item(item_id: int):
    for item in items_db:
        if item.id == item_id:
            return item
    raise HTTPException(status_code=404, detail="Item not found")

@app.get("/items/", response_model=List[Item])
async def list_items(skip: int = 0, limit: int = 10):
    return items_db[skip : skip + limit]
```

## Resources

- [PEP 8](https://pep8.org/)
- [Python Type Hints](https://docs.python.org/3/library/typing.html)
- [Real Python](https://realpython.com/)
- [FastAPI Docs](https://fastapi.tiangolo.com/)
