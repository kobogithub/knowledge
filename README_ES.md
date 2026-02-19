# Knowledge Framework

> **Un meta-framework CLI para flujos de trabajo de desarrollo asistido por IA**

**[🇪🇸 Leer en Español](./README_ES.md)** | **[🇬🇧 Read in English](./README.md)**

Knowledge Framework (`kn`) es una herramienta de línea de comandos que resuelve el "problema del arranque en frío" en el desarrollo asistido por IA, automatizando la configuración de proyectos con agentes, skills y flujos de trabajo estandarizados.

[![Rust](https://img.shields.io/badge/rust-1.93+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## 🎯 ¿Qué Problema Resuelve?

Cada vez que inicias un nuevo proyecto con agentes de IA, necesitas:
- ❌ Configurar manualmente los roles y flujos de trabajo de los agentes
- ❌ Configurar el seguimiento de issues desde cero
- ❌ Instalar y configurar skills para cada proyecto
- ❌ Configurar servidores MCP para documentación
- ❌ Crear plantillas estándar para planificación

**Knowledge Framework automatiza todo esto en segundos.**

---

## ✨ Características

### 🚀 Inicialización Instantánea de Proyectos
```bash
cd mi-proyecto/
kn init
# → Auto-detecta el tipo de proyecto
# → Crea AGENTS.md con instrucciones de flujo de trabajo
# → Genera configuración kn.toml
```

### 📚 Gestión de Skills
```bash
kn skills install typescript        # Desde agentskills.io
kn skills install https://...       # Desde URL
kn skills install ./SKILL.md        # Desde ruta local
kn skills list                      # Ver skills instalados
```

### 📋 Plantillas de Beads
```bash
kn beads template epic -o epic.md   # Generar plantillas estructuradas de issues
kn beads template task              # Imprimir a stdout para piping
kn beads template bug --force       # Sobrescribir archivos existentes
```

### 🔌 Gestión de Servidores MCP
```bash
kn mcp add filesystem               # Agregar servidores MCP preconfigurados
kn mcp add postgres -e POSTGRES_URL=... # Con variables de entorno
kn mcp list                         # Ver servidores configurados
```

---

## 🚀 Inicio Rápido

### Instalación

#### 🚀 Instalación Automatizada (Recomendada)

La forma más fácil de instalar `kn` es usando nuestro instalador automatizado. Descarga binarios precompilados desde GitHub releases:

```bash
# Instalación de una línea (recomendado)
curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh | bash

# O descargar e inspeccionar primero
wget https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.sh
chmod +x install.sh
./install.sh
```

**Funciones Inteligentes:**
- ✅ **Verificación de Versión**: Evita reinstalar si ya tienes la versión objetivo
- ✅ **Configuración Automática de PATH**: Agrega automáticamente `~/.local/bin` a tu configuración de shell (bash, zsh, fish)
- ✅ **Detección de Rosetta**: En macOS, usa binarios ARM64 incluso en terminal Rosetta para mejor rendimiento
- ✅ **Progreso Visual**: Muestra el progreso de descarga con una barra de progreso limpia
- ✅ **Binarios Precompilados**: No requiere compilación - instalación rápida

**Opciones Disponibles:**
```bash
./install.sh --help              # Mostrar todas las opciones
./install.sh --version v0.2.0    # Instalar versión específica
./install.sh --skip-deps         # Saltar instalación de dependencias (Git, Node.js, bd)
./install.sh --no-confirm        # Modo no interactivo
./install.sh --no-modify-path    # No modificar archivos de configuración del shell
```

**Plataformas Soportadas:**
- Linux x86_64
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon)

**Actualizaciones:**
```bash
kn update                        # Actualizar a la última versión
```

---

### Configuración Inicial

Después de instalar el CLI, necesitas poblar `~/.kn/` con agentes, skills y MCPs:

```bash
# Clonar este repositorio (si aún no lo has hecho)
git clone https://github.com/kobogithub/knowledge.git
cd knowledge

# Ejecutar el script de configuración para instalar todos los recursos globales
./setup-kn.sh
```

**Qué hace esto:**
- ✅ Crea `~/.kn/agents/` con todos los agentes disponibles (planner, frontend, backend, devops, qa, rust)
- ✅ Crea `~/.kn/skills/` (skills se instalan bajo demanda)
- ✅ Crea `~/.kn/mcps/` (MCPs se instalan bajo demanda)

**Nota:** Solo necesitas ejecutar esto una vez. Después, todos los proyectos pueden reutilizar estos recursos globales.

---

#### 🔨 Instalación Manual (Desde el Código Fuente)

Si prefieres compilar desde el código fuente o necesitas personalizar la instalación:

```bash
# 1. Instalar dependencias
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # Rust
cargo install bd                                                  # Beads
# Instalar Node.js desde https://nodejs.org/ o tu gestor de paquetes

# 2. Clonar y compilar
git clone https://github.com/kobogithub/knowledge.git
cd knowledge/cli
cargo build --release

# 3. Instalar globalmente
sudo cp target/release/kn /usr/local/bin/
# o al directorio de usuario
mkdir -p ~/.local/bin
cp target/release/kn ~/.local/bin/

# 4. Agregar al PATH (si no está ya)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # o ~/.zshrc

# 5. Verificar
kn doctor
```

---

#### 🪟 Instalación en Windows

```powershell
# Descargar y ejecutar el script de PowerShell
irm https://raw.githubusercontent.com/kobogithub/knowledge/prod/install.ps1 | iex

# O seguir las instrucciones manuales en install.ps1
```

**Nota**: La instalación automatizada para Windows está en desarrollo. Ver `install.ps1` para pasos manuales.

---

#### 📦 Gestores de Paquetes (Próximamente)

Estamos trabajando en paquetes oficiales para gestores de paquetes populares:

- **Homebrew** (macOS): `brew install kobogithub/knowledge/kn`
- **APT** (Ubuntu/Debian): `apt install kn`
- **DNF/YUM** (Fedora/RHEL): `dnf install kn`

Por ahora, por favor usa el instalador automatizado arriba.

---

### Inicializar Tu Primer Proyecto

```bash
cd ~/tu-proyecto/
kn init

# Seguir los prompts o usar -y para valores por defecto
kn init -y

# Saltar auto-instalación de skills recomendados
kn init -y --no-skills
```

**Los skills se auto-instalan según el proyecto detectado:**
- **Lenguaje Base + Detección de Framework**:
  - Proyectos Rust → rust-best-practices + docker + bash
  - Proyectos Python → python-best-practices + docker + bash
  - Proyectos Node → docker + bash
  - Proyectos Go → docker + bash
- **Skills específicos de framework**:
  - Astro detectado → astro-best-practices
  - FastAPI detectado → python-best-practices
  - ¡Más frameworks próximamente!
- **Workspace/Monorepo**: Detectado automáticamente para workspaces de Cargo, pnpm, npm

### Gestionar Skills

```bash
# Ver skills instalados
kn skills list

# Instalar skills adicionales
kn skills install <nombre-skill>

# Instalar desde URL o ruta local
kn skills install https://example.com/skill/SKILL.md
kn skills install ./local/skill/SKILL.md
```

### Generar Plantillas de Issues

```bash
# Crear directorio de plantillas
mkdir -p .beads/templates

# Generar todas las plantillas
kn beads template epic -o .beads/templates/epic.md
kn beads template task -o .beads/templates/task.md
kn beads template bug -o .beads/templates/bug.md
kn beads template feature -o .beads/templates/feature.md
kn beads template chore -o .beads/templates/chore.md
```

---

## 🏗️ Estructura del Proyecto

```
knowledge/
├── cli/                      # Herramienta CLI en Rust
│   ├── src/
│   │   ├── main.rs           # Punto de entrada
│   │   └── commands/
│   │       ├── init.rs       # Inicialización de proyecto
│   │       ├── skills.rs     # Gestión de skills
│   │       ├── beads.rs      # Plantillas de issues
│   │       └── mcp.rs        # Gestión de servidores MCP
│   ├── Cargo.toml
│   └── README.md
├── agents/                   # Instrucciones específicas por agente
│   ├── planner/
│   │   └── AGENTS.md         # Guía del agente planificador
│   ├── frontend/
│   │   └── AGENTS.md         # Guía del agente frontend
│   ├── backend/
│   │   └── AGENTS.md         # Guía del agente backend
│   ├── rust/
│   │   └── AGENTS.md         # Guía del agente Rust
│   └── devops/
│       └── AGENTS.md         # Guía del agente DevOps
├── skills/                   # Skills de agentes IA instalados
│   ├── astro-best-practices/
│   ├── bash-best-practices/
│   ├── docker-best-practices/
│   ├── python-best-practices/
│   ├── rust-best-practices/
│   ├── supabase-postgres-best-practices/
│   └── README.md
├── docs/                     # Documentación
│   ├── GETTING_STARTED.md
│   └── ARCHITECTURE.md
├── .beads/                   # Seguimiento de issues (Beads + Dolt)
│   └── issues.jsonl          # Base de datos de issues
├── AGENTS.md                 # Coordinación principal de agentes
└── README.md                 # Este archivo
```

---

## 🤖 Flujo de Trabajo Multi-Agente

Knowledge Framework usa [Beads](https://github.com/beadlist/beads) para seguimiento de issues con agentes IA especializados:

| Agente | ID | Responsabilidades |
|--------|-----|-------------------|
| **Planner** | `knowledge-x6e` | Coordina trabajo, crea épicas, asigna tareas |
| **Frontend** | `knowledge-4yh` | UI/UX, React, componentes, lado cliente |
| **Backend** | `knowledge-vlf` | APIs, bases de datos, lógica de negocio, seguridad |
| **Rust** | `knowledge-r5t` | Herramientas CLI, bibliotecas, programación de sistemas |
| **DevOps** | `knowledge-w5p` | Infraestructura, CI/CD, despliegue, monitoreo |

Cada agente:
- ✅ Tiene conocimiento y herramientas especializadas
- ✅ Cierra sus propias tareas de forma autónoma
- ✅ Reporta progreso de forma transparente
- ✅ Se coordina con otros agentes

Ver [AGENTS.md](./AGENTS.md) para instrucciones detalladas.

---

## 📚 Documentación

- [Documentación del CLI](./cli/README.md) - Referencia completa del CLI
- [Instrucciones de Agentes](./AGENTS.md) - Guía de flujo de trabajo multi-agente
- [Primeros Pasos](./docs/GETTING_STARTED.md) - Tutorial paso a paso
- [Arquitectura](./docs/ARCHITECTURE.md) - Diseño del sistema y decisiones
- [Guía de Desarrollo](./docs/DEVELOPMENT.md) - Guías para contribuir

---

## 🛣️ Hoja de Ruta

### ✅ Fase 1: CLI Base (Completada - 100%)
- [x] `kn init` - Inicialización de proyecto con auto-detección
- [x] `kn skills install/list` - Gestión de skills
- [x] `kn beads template` - Generación de plantillas de issues
- [x] `kn mcp add/list/remove` - Configuración de servidores MCP

### ✅ Fase 2: Mejoras (Completada - 100%)
- [x] Detección mejorada de proyectos (workspaces, frameworks)
- [ ] Soporte cross-platform (symlinks en Windows)
- [ ] Suite de tests para todos los comandos

### 🔮 Fase 3: Características Avanzadas
- [ ] `kn agent create` - Generación de agentes personalizados
- [ ] `kn workflow init` - Plantillas de flujos de trabajo
- [ ] `kn sync` - Sincronización multi-proyecto
- [ ] Sistema de plugins para extensibilidad

### 🌟 Fase 4: Ecosistema
- [ ] Repositorio público de skills (integración con agentskills.io)
- [ ] Dashboard web para vista general del proyecto
- [ ] Características de colaboración en equipo
- [ ] Analíticas e insights

---

## 🔧 Stack Tecnológico

- **CLI**: Rust (clap, reqwest, serde)
- **Seguimiento de Issues**: Beads + Dolt
- **Formato de Skills**: Markdown + frontmatter YAML
- **Configuración**: TOML
- **Agentes**: Flujo de trabajo multi-agente IA (Claude, GPT-4, etc.)

---

## 🎯 Casos de Uso

### 1. Onboarding de Nuevo Proyecto
```bash
cd nuevo-proyecto/
kn init
# → Configuración instantánea de agentes, listo para programar
```

### 2. Flujos de Trabajo Estandarizados
```bash
kn skills install estándares-empresa
# → Todos los proyectos siguen los mismos patrones
```

### 3. Planificación de Issues
```bash
kn beads template epic > planificacion/mvp.md
# → Plantillas de planificación estructuradas
```

### 4. Acceso a Documentación
```bash
kn mcp add rust-docs mdn-web-docs
# → Los agentes IA tienen acceso instantáneo a docs
```

---

## 🤝 Contribuir

¡Damos la bienvenida a contribuciones! Ver [DEVELOPMENT.md](./docs/DEVELOPMENT.md) para guías.

### Configuración de Desarrollo

```bash
# Clonar y compilar
git clone https://github.com/kobogithub/knowledge.git
cd knowledge/cli
cargo build

# Ejecutar tests
cargo test

# Verificar formato
cargo fmt --check
cargo clippy
```

### Crear Issues

Usa Beads para seguimiento de issues:

```bash
# Listar issues
bd list

# Crear un nuevo issue
bd create "Título" --type task -p 1 -l rust

# Ver detalles de un issue
bd show task-id
```

---

## 📊 Estado Actual

**Desarrollo del CLI**: 83% completo (5/6 tareas)
- ✅ Inicialización de proyectos
- ✅ Gestión de skills (install, list)
- ✅ Plantillas de Beads (epic, task, bug, feature, chore)
- ✅ Integración MCP
- ✅ Detección mejorada
- 🔄 Soporte para Windows

**Desarrollo Activo**: Agente Rust trabajando en mejoras

---

## 📄 Licencia

Licencia MIT - ver [LICENSE](LICENSE) para detalles.

---

## 🙏 Agradecimientos

- **[Beads](https://github.com/beadlist/beads)** por Steve Yegge - Framework de seguimiento de issues
- **[Dolt](https://doltdb.com)** - Control de versiones tipo Git para datos
- **[agentskills.io](https://agentskills.io)** - Estándar de repositorio de skills
- **Comunidad Rust** - Increíble tooling y ecosistema

---

## 💬 Soporte

- 📧 Issues: [GitHub Issues](https://github.com/kobogithub/knowledge/issues)
- 💬 Discusiones: [GitHub Discussions](https://github.com/kobogithub/knowledge/discussions)
- 📖 Docs: [Documentación](./docs/)

---

**Construido con ❤️ por los contribuidores de Knowledge Framework**
