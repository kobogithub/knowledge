# OpenCode Configuration

Este directorio contiene la configuración local para **OpenCode** y **Antigravity**, permitiendo que el proyecto tenga su propio workspace configurado con agentes, skills y MCP servers específicos.

## 📁 Estructura

```
.opencode/
├── opencode.json       # Configuración principal del workspace
├── skills/             # Skills instalados (OpenCode standard)
│   ├── rust-best-practices/
│   │   └── SKILL.md
│   ├── docker-best-practices/
│   │   └── SKILL.md
│   └── ...
└── .gitignore         # Archivos a ignorar (node_modules, logs, etc.)
```

## 🎯 ¿Qué es opencode.json?

El archivo `opencode.json` define la configuración del workspace para OpenCode/Antigravity, incluyendo:

### 1. **MCP Servers** (Model Context Protocol)
Servidores que proveen contexto adicional a los agentes de IA:

```json
"mcpServers": {
  "filesystem": {
    "command": "npx",
    "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/project"]
  },
  "github": {
    "command": "npx",
    "args": ["-y", "@modelcontextprotocol/server-github"],
    "env": {
      "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"
    }
  }
}
```

**Servidores disponibles:**
- `filesystem` - Acceso al sistema de archivos del proyecto
- `github` - Integración con GitHub (issues, PRs, commits)
- `postgres` - Acceso a base de datos PostgreSQL

### 2. **Skills**
Habilidades específicas instaladas en el proyecto:

```json
"skills": [
  {
    "name": "rust-best-practices",
    "path": "skills/rust-best-practices"
  }
]
```

**Skills Location:** `.opencode/skills/<skill-name>/SKILL.md` (OpenCode standard)  
**Auto-generated** based on installed skills in `.opencode/skills/`  
**Antigravity Access:** Symlinked from `.agent/skills/` → `.opencode/skills/`

### 3. **Agents**
Agentes especializados para este proyecto:

```json
"agents": {
  "planner": {
    "id": "knowledge-planner",
    "role": "planner",
    "instructionsPath": "/path/to/agents/planner/AGENTS.md"
  }
}
```

Cada agente tiene:
- **id**: Identificador único del agente
- **role**: Rol del agente (planner, implementation, review, etc.)
- **instructionsPath**: Ruta al archivo AGENTS.md con instrucciones específicas

### 4. **Workspace**
Metadatos del workspace:

```json
"workspace": {
  "name": "knowledge",
  "type": "rust-cli",
  "rootPath": "/path/to/project",
  "beadsEnabled": true,
  "beadsPath": "/path/to/project/.beads"
}
```

## 🚀 Generación Automática

El archivo `opencode.json` se genera automáticamente al ejecutar:

```bash
kn init
```

Este comando:
1. ✅ Detecta el tipo de proyecto (Rust, Node, Python, Go)
2. ✅ Detecta frameworks (Astro, FastAPI, etc.)
3. ✅ Instala skills recomendados
4. ✅ Genera configuración de MCP servers
5. ✅ Configura agentes basados en AGENTS.md
6. ✅ Crea `.opencode/opencode.json` con rutas absolutas

## 📝 Personalización

Puedes editar manualmente `opencode.json` para:

### Agregar MCP Servers adicionales

```json
"mcpServers": {
  "postgres": {
    "command": "npx",
    "args": ["-y", "@modelcontextprotocol/server-postgres", "postgresql://localhost/mydb"]
  },
  "brave-search": {
    "command": "npx",
    "args": ["-y", "@modelcontextprotocol/server-brave-search"],
    "env": {
      "BRAVE_API_KEY": "${BRAVE_API_KEY}"
    }
  }
}
```

### Agregar más agentes

```json
"agents": {
  "rust": {
    "id": "knowledge-rust",
    "role": "rust-dev",
    "instructionsPath": "/path/to/agents/rust/AGENTS.md"
  },
  "devops": {
    "id": "knowledge-devops",
    "role": "devops",
    "instructionsPath": "/path/to/agents/devops/AGENTS.md"
  }
}
```

### Cambiar tipo de workspace

```json
"workspace": {
  "type": "rust-monorepo",  // Cambiar a monorepo
  "beadsEnabled": true
}
```

## 🔧 Integración con OpenCode/Antigravity

OpenCode y Antigravity buscan automáticamente el archivo `.opencode/opencode.json` en el directorio raíz del proyecto.

Cuando lo encuentran:
- ✅ Cargan los MCP servers configurados
- ✅ Activan los skills del proyecto
- ✅ Configuran los agentes con sus instrucciones específicas
- ✅ Establecen el workspace context

## 🌐 Variables de Entorno

Puedes usar variables de entorno en la configuración:

```json
"env": {
  "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}",
  "POSTGRES_URL": "${DATABASE_URL}",
  "API_KEY": "${MY_API_KEY}"
}
```

Las variables se resuelven desde:
1. Variables de entorno del sistema
2. Archivo `.env` en el proyecto
3. Configuración global de OpenCode

## 📚 Recursos

- [OpenCode Documentation](https://opencode.ai/docs)
- [MCP Protocol Specification](https://modelcontextprotocol.io)
- [Beads Issue Tracking](https://github.com/beadlist/beads)
- [AgentSkills.io](https://agentskills.io)

## ⚠️ Notas Importantes

1. **Skills Location**: Skills se instalan en `.opencode/skills/` (OpenCode standard)
2. **Relative Paths**: El archivo usa rutas relativas para portabilidad
3. **Git Ignore**: `node_modules/` y archivos temporales están en `.gitignore`
4. **Regeneración**: Ejecutar `kn init` nuevamente NO sobrescribe el archivo existente
5. **Antigravity Compatibility**: `.agent/skills/` contiene symlinks a `.opencode/skills/`

## 🔄 Actualización Manual

Si agregas skills manualmente, actualiza la sección `skills`:

```bash
# Después de instalar un nuevo skill
kn skills install python-best-practices

# El skill se instala automáticamente en .opencode/skills/python-best-practices/

# Edita .opencode/opencode.json y agrega:
{
  "name": "python-best-practices",
  "path": "skills/python-best-practices"
}
```

O regenera el archivo:

```bash
# Backup del actual
mv .opencode/opencode.json .opencode/opencode.json.bak

# Regenerar
kn init -y

# Restaurar customizaciones desde backup si es necesario
```

---

**Este archivo es generado y mantenido por `kn init`**
