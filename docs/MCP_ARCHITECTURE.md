# Arquitectura de Gestión de MCPs en Knowledge Framework

## 📋 Resumen

El Knowledge Framework ahora incluye un sistema de gestión centralizada de servidores MCP (Model Context Protocol) que permite:

1. **Instalación global** de MCPs en `~/.kn/mcps/`
2. **Configuración por proyecto** en `kn.toml`
3. **Generación automática** de `.opencode/opencode.json` compatible con OpenCode

## 🏗️ Estructura de Directorios

```
~/.kn/
├── agents/           # Agentes globales
├── skills/           # Skills globales
└── mcps/            # 🆕 MCPs globales
    ├── filesystem/
    │   └── mcp.toml  # Metadata del MCP
    ├── github/
    │   └── mcp.toml
    └── postgres/
        └── mcp.toml
```

## 📄 Archivo `mcp.toml` (Metadata)

Cada MCP instalado tiene un archivo `mcp.toml` con su configuración:

```toml
[mcp]
name = "filesystem"
description = "Access local files and directories"
package = "@modelcontextprotocol/server-filesystem"

[command]
type = "local"
executable = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem"]

[environment]
# Variables de entorno (opcional)

[config]
supports_custom_args = true
default_args = ["."]
required_env = []
```

## 🔄 Flujo de Trabajo

### 1. Instalación Global

```bash
# Instalar desde preset
kn mcp install filesystem

# Instalar desde npm
kn mcp install @my-org/custom-mcp --as-name custom

# Instalar con comando personalizado
kn mcp install my-mcp --command docker --args run my-image
```

**Resultado:** Se crea `~/.kn/mcps/<name>/mcp.toml`

### 2. Configuración en Proyecto (`kn.toml`)

```toml
[project]
name = "my-project"
workspace_standard = "both"

[mcp]
enabled = ["filesystem", "github"]

# Configuración específica del proyecto
[mcp.config.filesystem]
args = ["/home/user/project"]  # Override args

[mcp.config.github]
environment = { GITHUB_PERSONAL_ACCESS_TOKEN = "token" }
enabled = true  # Puede deshabilitarse
```

### 3. Sincronización (`kn sync`)

```bash
kn sync
```

Genera `.opencode/opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "filesystem": {
      "type": "local",
      "command": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "/home/user/project"],
      "enabled": true
    },
    "github": {
      "type": "local",
      "command": ["npx", "-y", "@modelcontextprotocol/server-github"],
      "enabled": true,
      "environment": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "token"
      }
    }
  }
}
```

## 🎯 Comandos CLI

### Gestión Global

```bash
kn mcp presets                # Lista presets disponibles
kn mcp list                   # Lista MCPs instalados
kn mcp list --detailed        # Lista con detalles
kn mcp info <name>           # Muestra info de un MCP
kn mcp install <name>        # Instala MCP globalmente
kn mcp uninstall <name>      # Desinstala MCP
```

### Gestión de Proyecto

```bash
kn mcp add <name>            # Agrega MCP al proyecto
kn mcp add <name> --args "custom args"  # Con args personalizados
kn mcp add <name> --env KEY=VALUE       # Con variables de entorno
kn mcp remove <name>         # Remueve MCP del proyecto
kn mcp enable <name>         # Habilita MCP deshabilitado
kn mcp disable <name>        # Deshabilita MCP
```

### Sincronización

```bash
kn sync                      # Sincroniza todo (skills, agents, mcps)
```

## 🔌 Presets Disponibles

### 1. **filesystem**
- **Descripción:** Access local files and directories
- **Package:** `@modelcontextprotocol/server-filesystem`
- **Args personalizados:** ✅ Sí (directorio raíz)
- **Env requeridas:** Ninguna

### 2. **github**
- **Descripción:** GitHub API integration
- **Package:** `@modelcontextprotocol/server-github`
- **Args personalizados:** ❌ No
- **Env requeridas:** `GITHUB_PERSONAL_ACCESS_TOKEN`

### 3. **postgres**
- **Descripción:** PostgreSQL database access
- **Package:** `@modelcontextprotocol/server-postgres`
- **Args personalizados:** ❌ No
- **Env requeridas:** `POSTGRES_URL`

### 4. **brave-search**
- **Descripción:** Web search via Brave Search API
- **Package:** `@modelcontextprotocol/server-brave-search`
- **Args personalizados:** ❌ No
- **Env requeridas:** `BRAVE_API_KEY`

### 5. **puppeteer**
- **Descripción:** Web scraping and browser automation
- **Package:** `@modelcontextprotocol/server-puppeteer`
- **Args personalizados:** ❌ No
- **Env requeridas:** Ninguna

## 📊 Ejemplo Completo

### Paso 1: Instalar MCPs

```bash
kn mcp install filesystem
kn mcp install github
```

### Paso 2: Agregar al Proyecto

```bash
cd my-project
kn mcp add filesystem --args /home/user/my-project
kn mcp add github --env GITHUB_PERSONAL_ACCESS_TOKEN=$GITHUB_TOKEN
```

**Resultado en `kn.toml`:**

```toml
[mcp]
enabled = ["filesystem", "github"]

[mcp.config.filesystem]
args = ["/home/user/my-project"]

[mcp.config.github]
environment = { GITHUB_PERSONAL_ACCESS_TOKEN = "$GITHUB_TOKEN" }
```

### Paso 3: Sincronizar

```bash
kn sync
```

**Resultado:** Se genera `.opencode/opencode.json` con la configuración completa.

## 🔍 Detalles Técnicos

### Módulos Implementados

1. **`cli/src/models/mcp.rs`**
   - `McpMetadata` - Estructura de metadata
   - `from_preset()` - Presets predefinidos
   - `generate_command()` - Generación de comandos
   - `validate_environment()` - Validación de env vars

2. **`cli/src/commands/mcp.rs`**
   - Comandos de gestión de MCPs
   - Instalación/desinstalación
   - Agregar/remover de proyectos

3. **`cli/src/config/opencode.rs`**
   - `OpenCodeConfig` - Estructura de `.opencode/opencode.json`
   - `generate_from_project()` - Generación desde `kn.toml`
   - Serialización JSON compatible con schema de OpenCode

4. **`cli/src/core/kn_home.rs`**
   - `mcps_dir()` - Directorio de MCPs
   - `list_installed_mcps()` - Listado de MCPs
   - `ensure_kn_home()` - Creación de estructura

### Transformación de Datos

**De `kn.toml` (TOML):**
```toml
[mcp.config.filesystem]
args = ["/custom/path"]
```

**A `.opencode/opencode.json` (JSON):**
```json
{
  "filesystem": {
    "type": "local",
    "command": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "/custom/path"],
    "enabled": true
  }
}
```

## ✅ Estado de Implementación

- [x] Modelo de datos `McpMetadata`
- [x] Comandos `kn mcp install/uninstall/list/info`
- [x] Comandos `kn mcp add/remove/enable/disable`
- [x] Generación de `.opencode/opencode.json` en `kn sync`
- [x] 5 presets predefinidos
- [ ] Generación en `kn init`
- [ ] Symlinks de MCPs (si necesario)
- [ ] Tests unitarios completos

## 🚀 Próximos Pasos

1. **Integrar en `kn init`**: Generar `.opencode/opencode.json` al inicializar proyecto
2. **Validación de MCPs**: Verificar que los paquetes npm existen
3. **Templates de MCPs**: Permitir crear MCPs personalizados desde templates
4. **Auto-detección**: Detectar MCPs instalados localmente en node_modules
5. **Versionado**: Soporte para versiones específicas de MCPs

## 📚 Referencias

- [OpenCode Config Schema](https://opencode.ai/config.json)
- [Model Context Protocol](https://modelcontextprotocol.io)
- [MCP Servers Repository](https://github.com/modelcontextprotocol/servers)
