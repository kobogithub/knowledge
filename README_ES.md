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
# Instalación de una línea (recomendado - URL corta)
curl -fsSL https://kn.foxlabar.online/install | bash

# O usando la URL completa de GitHub
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

El script de instalación automáticamente pobla `~/.kn/` con todos los recursos disponibles:

**Que se instala automaticamente:**
- Todos los agentes en `~/.kn/agents/` (planner, frontend, backend, devops, qa, rust, security, uiux-tester, finanzas)
- Todos los skills en `~/.kn/skills/` (11 skills de mejores practicas)
- Directorio MCP `~/.kn/mcps/` (MCPs se instalan bajo demanda)

**¡No se requiere configuración adicional!** Después de la instalación, puedes ejecutar inmediatamente `kn init` en cualquier proyecto para seleccionar qué recursos usar.

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

### Desinstalar

Si necesitas desinstalar `kn`, proporcionamos un script de desinstalación completo:

#### 🗑️ Desinstalación Básica

Elimina solo el binario `kn` y la configuración del shell:

```bash
curl -fsSL https://raw.githubusercontent.com/kobogithub/knowledge/prod/uninstall.sh | bash
```

O si tienes el repositorio clonado:

```bash
./uninstall.sh
```

#### 🧹 Eliminación Completa

Elimina todo incluyendo todos los recursos globales:

```bash
# Eliminar binario kn + directorio ~/.kn/ (agentes, skills, MCPs)
./uninstall.sh --remove-data

# Eliminar binario kn + configuraciones de proyectos (kn.toml, .opencode/, .gemini/)
./uninstall.sh --remove-config

# Eliminar todo (binario + datos + configuraciones de proyectos)
./uninstall.sh --remove-data --remove-config --yes
```

**Opciones:**
- `--remove-data` - Elimina el directorio `~/.kn/` (todos los agentes, skills, MCPs)
- `--remove-config` - Elimina configuraciones de proyectos en ubicaciones comunes
- `--yes` - Omite todas las confirmaciones
- `--help` - Muestra ayuda detallada

**Qué se elimina:**
- ✅ Binario `kn` de `~/.local/bin/` o `/usr/local/bin/`
- ✅ Entradas de configuración del shell (`.bashrc`, `.zshrc`, etc.)
- ⚠️ Directorio `~/.kn/` (solo con `--remove-data`)
- ⚠️ Configuraciones de proyectos (solo con `--remove-config`)
- ❌ Dependencias (Git, Node.js, bd) NO se eliminan

**Nota:** El script de desinstalación crea respaldos de los archivos de configuración del shell antes de modificarlos.

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
│   │       ├── init.rs       # Inicializacion de proyecto
│   │       ├── skills.rs     # Gestion de skills
│   │       ├── beads.rs      # Plantillas de issues
│   │       ├── mcp.rs        # Gestion de servidores MCP
│   │       ├── doctor.rs     # Verificacion de dependencias
│   │       ├── update.rs     # Auto-actualizacion
│   │       ├── sync.rs       # Sincronizacion de proyecto
│   │       └── agents.rs     # Gestion de agentes
│   ├── Cargo.toml
│   └── README.md
├── agents/                   # Instrucciones por agente (9 agentes)
│   ├── planner/AGENTS.md     # Planner — coordinacion, epics, workflow 5 fases
│   ├── frontend/AGENTS.md    # Frontend — UI/UX, React, componentes
│   ├── backend/AGENTS.md     # Backend — APIs, bases de datos, seguridad
│   ├── rust/AGENTS.md        # Rust — CLI, bibliotecas, programacion de sistemas
│   ├── devops/AGENTS.md      # DevOps — infraestructura, CI/CD, monitoreo
│   ├── qa/AGENTS.md          # QA — testing, aseguramiento de calidad
│   ├── security/AGENTS.md    # Security — AppSec, SAST, escaneo de vulnerabilidades
│   ├── uiux-tester/AGENTS.md # UI/UX Tester — fidelidad visual, accesibilidad
│   └── finanzas/AGENTS.md    # Finanzas — seguimiento de costos, presupuestos
├── .opencode/skills/         # Skills de agentes IA instalados (11 skills)
│   ├── bash-best-practices/
│   ├── bd-best-practices/    # Manual de workflow de 5 fases
│   ├── docker-best-practices/
│   ├── github-actions-best-practices/
│   ├── jsonnet-best-practices/
│   ├── python-best-practices/
│   ├── rust-best-practices/
│   ├── security-gitleaks/
│   ├── security-owasp-zap/
│   ├── security-semgrep/
│   └── security-trivy/
├── .beads/                   # Seguimiento de issues (Beads)
│   ├── issues.jsonl          # Base de datos de issues
│   └── formulas/             # Plantillas de workflow
│       ├── mol-feature.formula.json
│       ├── mol-bugfix.formula.json
│       ├── mol-spike.formula.json
│       └── mol-release.formula.json
├── docs/                     # Documentacion
│   ├── GETTING_STARTED.md
│   ├── ARCHITECTURE.md
│   └── AGENT_MODEL_STRATEGY.md
├── AGENTS.md                 # Coordinacion principal de agentes
├── kn.toml                   # Configuracion del proyecto
└── README.md                 # Este archivo
```

---

## 🤖 Flujo de Trabajo Multi-Agente

Knowledge Framework usa [Beads](https://github.com/beadlist/beads) para seguimiento de issues con 9 agentes IA especializados:

| Agente | ID | Responsabilidades |
|--------|-----|-------------------|
| **Planner** | `knowledge-x6e` | Coordina trabajo, crea epics, asigna tareas |
| **Frontend** | `knowledge-4yh` | UI/UX, React, componentes, lado cliente |
| **Backend** | `knowledge-vlf` | APIs, bases de datos, logica de negocio, seguridad |
| **Rust** | `knowledge-r5t` | Herramientas CLI, bibliotecas, programacion de sistemas |
| **DevOps** | `knowledge-w5p` | Infraestructura, CI/CD, despliegue, monitoreo |
| **QA** | `knowledge-pu1` | Testing, aseguramiento de calidad, automatizacion de pruebas |
| **Security** | `knowledge-s3c` | AppSec, escaneo de vulnerabilidades, SAST, deteccion de secretos |
| **UI/UX Tester** | `knowledge-u7x` | Fidelidad visual, testing de interaccion, accesibilidad (WCAG) |
| **Finanzas** | `knowledge-f1n` | Seguimiento de costos OpenRouter, reportes de gasto, controles de presupuesto |

Cada agente:
- Tiene conocimiento y herramientas especializadas
- Cierra sus propias tareas de forma autonoma
- Reporta progreso de forma transparente
- Se coordina con otros agentes via comentarios de bd
- Sigue el flujo de trabajo estructurado de 5 fases

Ver [AGENTS.md](./AGENTS.md) para instrucciones detalladas.

### 🔄 Framework de Workflow en 5 Fases

Todo trabajo significativo sigue un ciclo de vida estructurado en 5 fases gestionado con `bd`:

| Fase | Nombre | Comandos Clave |
|------|--------|---------------|
| 1 | **Exploracion** | `bd create -t decision`, `bd query`, `bd kv`, `bd todo add` |
| 2 | **Especificacion** | `bd formula list`, `bd cook`, `bd lint`, `bd graph` |
| 3 | **Task Planning** | `bd mol pour`, `bd swarm`, `bd slot`, `bd count` |
| 4 | **Implementacion** | `bd agent state`, `bd heartbeat`, `bd merge-slot`, `bd audit` |
| 5 | **Verificacion** | `bd gate resolve`, `bd preflight`, `bd orphans`, `bd epic close-eligible` |

**Plantillas de workflow (formulas)** disponibles para patrones comunes:
- `mol-feature` — Workflow de 8 pasos para features con multiples agentes
- `mol-bugfix` — 4 pasos para bugfix con analisis de causa raiz
- `mol-spike` — 3 pasos para investigacion con tiempo limitado
- `mol-release` — 6 pasos para release con quality gates

---

## 📚 Documentación

- [Documentación del CLI](./cli/README.md) - Referencia completa del CLI
- [Instrucciones de Agentes](./AGENTS.md) - Guía de flujo de trabajo multi-agente
- [Primeros Pasos](./docs/GETTING_STARTED.md) - Tutorial paso a paso
- [Arquitectura](./docs/ARCHITECTURE.md) - Diseño del sistema y decisiones
- [Guía de Desarrollo](./docs/DEVELOPMENT.md) - Guías para contribuir

---

## 🛣️ Hoja de Ruta

### Fase 1: CLI Base (Completada)
- [x] `kn init` - Inicializacion de proyecto con auto-deteccion
- [x] `kn skills install/list` - Gestion de skills
- [x] `kn beads template` - Generacion de plantillas de issues
- [x] `kn mcp add/list/remove` - Configuracion de servidores MCP
- [x] `kn doctor` - Verificacion de dependencias
- [x] `kn update` - Mecanismo de auto-actualizacion

### Fase 2: Mejoras (Completada)
- [x] Deteccion mejorada de proyectos (workspaces, frameworks)
- [x] Soporte de estandar de workspace (OpenCode/Antigravity/Ambos)
- [ ] Soporte cross-platform (symlinks en Windows)
- [ ] Suite de tests para todos los comandos

### Fase 3: Multi-Agente y Workflow (Completada)
- [x] 9 agentes especializados con MCP scoping por agente
- [x] Framework de workflow en 5 fases (Exploracion hasta Verificacion)
- [x] 4 plantillas de workflow formula (feature, bugfix, spike, release)
- [x] Merge-slot para coordinacion serializada de pushes
- [x] Skill `bd-best-practices` como manual completo de 5 fases

### Fase 4: Caracteristicas Avanzadas
- [ ] `kn agent create` - Generacion de agentes personalizados
- [ ] `kn workflow init` - Plantillas de flujos de trabajo
- [ ] `kn sync` - Sincronizacion multi-proyecto
- [ ] Sistema de plugins para extensibilidad

### Fase 5: Ecosistema
- [ ] Repositorio publico de skills (integracion con agentskills.io)
- [ ] Dashboard web para vista general del proyecto
- [ ] Caracteristicas de colaboracion en equipo
- [ ] Analiticas e insights

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

**Version**: 0.5.1

**CLI**: Todos los comandos core implementados (`init`, `skills`, `beads`, `mcp`, `doctor`, `update`, `sync`, `agents`)

**Agentes**: 9 agentes especializados operativos con MCP scoping por agente

**Workflow**: Framework de 5 fases completamente documentado y validado end-to-end

**Formulas**: 4 plantillas de workflow (feature, bugfix, spike, release)

**Skills**: 11 skills de mejores practicas instalados

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
