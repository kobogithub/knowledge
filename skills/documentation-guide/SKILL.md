---
name: documentation-guide
description: Comprehensive guide for writing technical documentation
license: MIT
compatibility: opencode
metadata:
  stack: documentation, technical-writing
  audience: developers, technical-writers
---

## What I do

I provide a complete guide for creating high-quality technical documentation including:

- Documentation types and when to use them
- Writing templates and examples
- Best practices for clarity and maintainability
- Tools and workflows for documentation
- API documentation standards
- Code comment conventions

## When to use me

Use this skill when:

- Starting documentation for a new project
- Need templates for specific doc types
- Improving existing documentation
- Setting up documentation infrastructure
- Creating API documentation
- Writing user guides or tutorials
- Establishing documentation standards for a team

## Documentation Types

### 1. README Template

```markdown
# Project Name

One-line description of what this project does.

[![Build Status](badge)](link)
[![Coverage](badge)](link)
[![License](badge)](LICENSE)

## Features

- Feature 1
- Feature 2
- Feature 3

## Quick Start

\```bash
# Installation
npm install package-name

# Basic usage
import { something } from 'package-name'
\```

## Documentation

- [Full Documentation](link)
- [API Reference](link)
- [Examples](link)

## Installation

Detailed installation instructions...

## Usage

Basic usage examples...

## Configuration

Configuration options...

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## License

[License Name](LICENSE)
```

### 2. API Documentation Template

```markdown
## Endpoint Name

Brief description of what this endpoint does.

**Endpoint:** `METHOD /path/to/endpoint`

**Authentication:** Required/Not Required

**Rate Limit:** X requests per minute

### Request

**Headers:**
\```
Content-Type: application/json
Authorization: Bearer <token>
\```

**Parameters:**

| Name | Type | Required | Description | Constraints |
|------|------|----------|-------------|-------------|
| param1 | string | Yes | Description | Min: 1, Max: 50 |
| param2 | integer | No | Description | > 0 |

**Request Body:**
\```json
{
  "field": "value"
}
\```

### Response

**Success (200):**
\```json
{
  "id": "123",
  "status": "success"
}
\```

**Error (400):**
\```json
{
  "error": "Error message"
}
\```

### Examples

**cURL:**
\```bash
curl -X POST "https://api.example.com/endpoint" \
  -H "Authorization: Bearer TOKEN" \
  -d '{"field": "value"}'
\```

**Python:**
\```python
import requests

response = requests.post(
    'https://api.example.com/endpoint',
    headers={'Authorization': 'Bearer TOKEN'},
    json={'field': 'value'}
)
\```
```

### 3. Architecture Documentation Template

```markdown
# System Architecture

## Overview

High-level description of the system architecture.

## Architecture Diagram

\```
[Component A] ──▶ [Component B] ──▶ [Component C]
     │                  │                 │
     └──────────────────┴─────────────────┘
                        │
                    [Database]
\```

## Components

### Component A
- **Purpose:** What it does
- **Technology:** Tech stack
- **Responsibilities:** Key responsibilities
- **Dependencies:** What it depends on

### Component B
- **Purpose:** What it does
- **Technology:** Tech stack
- **Responsibilities:** Key responsibilities

## Data Flow

1. User action triggers...
2. Component A processes...
3. Component B validates...
4. Result is returned

## Security

- Authentication method
- Authorization model
- Data encryption
- Security considerations

## Scalability

- Horizontal scaling approach
- Caching strategy
- Load balancing
- Database scaling

## Technology Stack

| Component | Technology | Version |
|-----------|------------|---------|
| Backend | FastAPI | 0.104.x |
| Database | PostgreSQL | 16.x |
| Cache | Redis | 7.x |
```

## Code Documentation Standards

### Python Docstring (Google Style)

```python
def function_name(param1: str, param2: int, optional: bool = False) -> dict:
    """
    Brief one-line description.
    
    More detailed description if needed. Can span multiple lines
    and include additional context about the function's purpose.
    
    Args:
        param1: Description of first parameter
        param2: Description of second parameter
        optional: Description of optional parameter. Defaults to False.
    
    Returns:
        Dictionary containing:
            - key1 (str): Description
            - key2 (int): Description
    
    Raises:
        ValueError: When param2 is negative
        TypeError: When param1 is not a string
    
    Example:
        >>> result = function_name("test", 42)
        >>> print(result['key1'])
        'test'
    
    Note:
        Important notes about usage, performance, or limitations.
    
    See Also:
        - related_function(): For related functionality
    """
    pass
```

### TypeScript/JavaScript (JSDoc)

```typescript
/**
 * Brief one-line description.
 * 
 * More detailed description if needed.
 * 
 * @param {string} param1 - Description of first parameter
 * @param {number} param2 - Description of second parameter
 * @param {boolean} [optional=false] - Description of optional parameter
 * 
 * @returns {Promise<Object>} Promise resolving to result object
 * @returns {string} returns.key1 - Description of key1
 * @returns {number} returns.key2 - Description of key2
 * 
 * @throws {Error} When param2 is negative
 * 
 * @example
 * const result = await functionName("test", 42);
 * console.log(result.key1);
 * 
 * @see {@link relatedFunction} for related functionality
 */
async function functionName(
  param1: string,
  param2: number,
  optional: boolean = false
): Promise<{key1: string; key2: number}> {
  // Implementation
}
```

## Documentation Best Practices

### Writing Style

1. **Be Concise**
   ```markdown
   ❌ Bad: "This function is responsible for the purpose of performing validation on user input"
   ✅ Good: "Validates user input"
   ```

2. **Use Active Voice**
   ```markdown
   ❌ Bad: "The data is processed by the service"
   ✅ Good: "The service processes the data"
   ```

3. **Be Specific**
   ```markdown
   ❌ Bad: "Returns data"
   ✅ Good: "Returns a list of user objects with id, name, and email fields"
   ```

4. **Include Examples**
   ```markdown
   Always show concrete examples:
   
   \```python
   # Create a new user
   user = User(name="John", email="john@example.com")
   user.save()
   \```
   ```

### Structure

1. **Start with Overview**
   - What does this do?
   - Why does it exist?
   - When should you use it?

2. **Provide Quick Start**
   - Minimal example to get running
   - Most common use case

3. **Detailed Reference**
   - All options and parameters
   - Edge cases
   - Advanced usage

4. **Troubleshooting**
   - Common errors
   - Solutions
   - FAQ

### Maintenance

1. **Keep Docs With Code**
   ```
   src/
   ├── feature/
   │   ├── feature.py
   │   └── README.md  ← Feature documentation
   ```

2. **Version Documentation**
   - Tag docs with version numbers
   - Maintain changelog
   - Archive old versions

3. **Review Process**
   - Docs reviewed with code
   - Check for broken links
   - Update examples

## Documentation Tools

### Static Site Generators

**MkDocs (Python):**
```yaml
# mkdocs.yml
site_name: My Project
theme:
  name: material
  features:
    - navigation.tabs
    - search.suggest

nav:
  - Home: index.md
  - Getting Started: getting-started.md
  - API Reference: api/
  - Examples: examples.md

plugins:
  - search
  - mkdocstrings:
      handlers:
        python:
          options:
            show_source: true
```

**Docusaurus (JavaScript):**
```javascript
// docusaurus.config.js
module.exports = {
  title: 'My Project',
  tagline: 'Project tagline',
  url: 'https://docs.example.com',
  baseUrl: '/',
  themeConfig: {
    navbar: {
      title: 'My Project',
      items: [
        {to: 'docs/', label: 'Docs', position: 'left'},
        {to: 'blog', label: 'Blog', position: 'left'},
        {to: 'api', label: 'API', position: 'left'},
      ],
    },
  },
};
```

### API Documentation

**OpenAPI/Swagger (FastAPI):**
```python
from fastapi import FastAPI
from pydantic import BaseModel, Field

app = FastAPI(
    title="My API",
    description="API for managing resources",
    version="1.0.0",
    openapi_tags=[
        {
            "name": "users",
            "description": "Operations with users",
        },
        {
            "name": "posts",
            "description": "Operations with posts",
        }
    ]
)

class UserCreate(BaseModel):
    """User creation schema."""
    username: str = Field(..., min_length=3, description="Unique username")
    email: str = Field(..., description="Valid email address")
```

## Changelog Format

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New feature description

### Changed
- Changed feature description

### Fixed
- Bug fix description

## [1.2.0] - 2024-01-15

### Added
- Feature A with detailed explanation
- Feature B that does X, Y, Z

### Changed
- Updated dependency X from v1 to v2
- Improved performance of feature Y by 40%

### Deprecated
- Old endpoint `/api/v1/old` (use `/api/v2/new` instead)

### Removed
- Removed deprecated feature Z

### Fixed
- Fixed bug where X caused Y (#123)
- Resolved memory leak in component A (#456)

### Security
- Updated dependency to patch CVE-2024-XXXX

## [1.1.0] - 2024-01-01

...

[Unreleased]: https://github.com/user/repo/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/user/repo/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/user/repo/releases/tag/v1.1.0
```

## Contributing Guide Template

```markdown
# Contributing to Project Name

Thank you for your interest in contributing!

## Code of Conduct

This project adheres to the [Code of Conduct](CODE_OF_CONDUCT.md).

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports:
- Check the issue tracker
- Check the documentation
- Try the latest version

When creating a bug report, include:
- Clear title and description
- Steps to reproduce
- Expected vs actual behavior
- Version information
- Screenshots if applicable

### Suggesting Features

Feature requests are welcome! Include:
- Clear use case
- Expected behavior
- Why this would be useful

### Pull Requests

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests
5. Update documentation
6. Commit with clear messages
7. Push to your fork
8. Open a Pull Request

### Development Setup

\```bash
# Clone your fork
git clone https://github.com/your-username/project.git

# Install dependencies
pip install -r requirements.txt
pip install -r requirements-dev.txt

# Run tests
pytest

# Check code style
black .
flake8
\```

### Coding Standards

- Follow PEP 8 for Python
- Write tests for new features
- Update documentation
- Keep commits focused
- Write clear commit messages

### Commit Messages

Format:
\```
type(scope): brief description

Detailed explanation if needed.

Fixes #123
\```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Tests
- `chore`: Maintenance

## Review Process

1. Automated checks must pass
2. Code review by maintainer
3. Approval required for merge
4. Squash and merge to main

## Questions?

- Open a discussion
- Join our Discord
- Email: maintainer@example.com
```

## Documentation Workflow

### 1. Planning
- Identify audience
- Define scope
- Choose tools
- Set standards

### 2. Writing
- Use templates
- Include examples
- Add diagrams
- Review for clarity

### 3. Review
- Technical accuracy
- Completeness
- Clarity
- Examples work

### 4. Publishing
- Build docs site
- Deploy to hosting
- Update links
- Announce changes

### 5. Maintenance
- Regular reviews
- Update with code changes
- Fix broken links
- Archive old versions

## Tips

- **Write as you code** - Don't save docs for later
- **User perspective** - Write for your audience, not yourself
- **Show, don't tell** - Examples are worth 1000 words
- **Keep it current** - Outdated docs are worse than no docs
- **Make it searchable** - Use clear headings and keywords
- **Link generously** - Connect related documentation
- **Test examples** - Ensure all code examples actually work
