---
name: bash-best-practices
description: Bash scripting best practices, portable patterns and defensive coding
version: 1.0.0
tags:
  - best-practices
  - bash
  - shell
  - scripting
  - automation
---

# bash-best-practices

Mejores practicas para scripts Bash: coding defensivo, portabilidad, patrones robustos y automatizacion confiable.

## Overview

Bash es el lenguaje de glue por excelencia para:
- **Automatizacion**: Scripts de build, deploy, setup
- **CI/CD**: Pasos de pipeline en GitHub Actions, Jenkins
- **DevOps**: Provisionamiento, health checks, backups
- **Developer tooling**: Wrappers, helpers, git hooks

## Shebang y Modo Estricto

### Siempre iniciar con modo estricto

```bash
#!/usr/bin/env bash
set -euo pipefail

# -e: Salir inmediatamente si un comando falla
# -u: Error si se usa una variable no definida
# -o pipefail: El pipeline falla si cualquier comando falla (no solo el ultimo)
```

### IFS seguro (opcional pero recomendado)

```bash
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
# Evita problemas con espacios en nombres de archivo
```

## Variables

### Declaracion y quoting

```bash
# SIEMPRE hacer quote de variables
readonly APP_NAME="myapp"
readonly VERSION="${1:-1.0.0}"  # Default value si no se pasa argumento

# Mal - sin quotes, se rompe con espacios
cp $file $destination

# Bien - con quotes, seguro
cp "$file" "$destination"

# Arrays
declare -a FILES=("file1.txt" "file2.txt" "file with spaces.txt")
for f in "${FILES[@]}"; do
  echo "Processing: $f"
done
```

### Variables con defaults

```bash
# ${var:-default} - usa default si var no esta definida o esta vacia
DB_HOST="${DB_HOST:-localhost}"
DB_PORT="${DB_PORT:-5432}"

# ${var:?message} - error si var no esta definida
DB_PASSWORD="${DB_PASSWORD:?ERROR: DB_PASSWORD no esta definida}"

# ${var:+value} - usa value solo si var ESTA definida
VERBOSE_FLAG="${VERBOSE:+--verbose}"
```

### Scope y readonly

```bash
# Constantes como readonly
readonly CONFIG_DIR="/etc/myapp"
readonly LOG_FILE="/var/log/myapp.log"

# Variables locales en funciones
my_function() {
  local result=""
  local -r max_retries=3  # local + readonly
  result="computed value"
  echo "$result"
}
```

## Funciones

### Patron estandar

```bash
# Documentar parametros y return
# Usar local para todas las variables
# Preferir echo/printf para retornar valores (no return con numeros)

##
# Verifica si un comando existe en el PATH.
# Args: $1 - nombre del comando
# Returns: 0 si existe, 1 si no
##
command_exists() {
  local cmd="$1"
  command -v "$cmd" &>/dev/null
}

##
# Crea un directorio si no existe, con logging.
# Args: $1 - path del directorio
##
ensure_dir() {
  local dir="$1"
  if [[ ! -d "$dir" ]]; then
    mkdir -p "$dir"
    log "Created directory: $dir"
  fi
}
```

### Logging estandar

```bash
# Colores (solo si stdout es un terminal)
if [[ -t 1 ]]; then
  RED='\033[0;31m'
  GREEN='\033[0;32m'
  YELLOW='\033[0;33m'
  BLUE='\033[0;34m'
  NC='\033[0m'  # No Color
else
  RED='' GREEN='' YELLOW='' BLUE='' NC=''
fi

log()   { echo -e "${BLUE}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*" >&2; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }
die()   { error "$*"; exit 1; }

# Uso
log "Starting deployment..."
warn "Config file not found, using defaults"
die "Cannot connect to database"
```

## Control de Flujo

### Condicionales robustos

```bash
# Usar [[ ]] en vez de [ ] (mas seguro, soporta regex y globbing)
if [[ -f "$config_file" ]]; then
  source "$config_file"
fi

# Comparacion de strings
if [[ "$env" == "production" ]]; then
  log "Production mode"
fi

# Regex matching
if [[ "$email" =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$ ]]; then
  log "Valid email"
fi

# Verificar si variable esta vacia
if [[ -z "${MY_VAR:-}" ]]; then
  die "MY_VAR is required"
fi

# Verificar si comando existe
if ! command_exists "docker"; then
  die "Docker is not installed"
fi
```

### Loops seguros

```bash
# Iterar sobre archivos (maneja espacios en nombres)
while IFS= read -r -d '' file; do
  echo "Processing: $file"
done < <(find /path -name "*.txt" -print0)

# Iterar sobre lineas de un archivo
while IFS= read -r line; do
  echo "Line: $line"
done < "$input_file"

# Iterar sobre output de un comando
while IFS= read -r container; do
  docker stop "$container"
done < <(docker ps -q)

# NUNCA hacer esto (se rompe con espacios y caracteres especiales):
# for file in $(find . -name "*.txt"); do ...
```

## Manejo de Errores

### Trap para cleanup

```bash
# Cleanup automatico al salir (exito o error)
cleanup() {
  local exit_code=$?
  log "Cleaning up..."
  [[ -f "$TEMP_FILE" ]] && rm -f "$TEMP_FILE"
  [[ -d "$TEMP_DIR" ]] && rm -rf "$TEMP_DIR"
  exit "$exit_code"
}
trap cleanup EXIT

# Crear recursos temporales seguros
TEMP_FILE="$(mktemp)"
TEMP_DIR="$(mktemp -d)"
```

### Retry pattern

```bash
##
# Reintenta un comando N veces con backoff exponencial.
# Args: $1 - max intentos, $@ - comando a ejecutar
##
retry() {
  local max_attempts="$1"
  shift
  local attempt=1

  while (( attempt <= max_attempts )); do
    if "$@"; then
      return 0
    fi

    if (( attempt == max_attempts )); then
      error "Command failed after $max_attempts attempts: $*"
      return 1
    fi

    local wait_time=$(( 2 ** (attempt - 1) ))
    warn "Attempt $attempt/$max_attempts failed. Retrying in ${wait_time}s..."
    sleep "$wait_time"
    (( attempt++ ))
  done
}

# Uso
retry 3 curl -sf "https://api.example.com/health"
retry 5 docker push "myimage:latest"
```

### Validacion de argumentos

```bash
usage() {
  cat <<EOF
Usage: $(basename "$0") [OPTIONS] <command>

Options:
  -e, --env ENV     Environment (dev|staging|production)
  -v, --verbose     Enable verbose output
  -h, --help        Show this help

Commands:
  deploy            Deploy the application
  rollback          Rollback to previous version

Examples:
  $(basename "$0") --env production deploy
  $(basename "$0") -e staging -v deploy
EOF
  exit "${1:-0}"
}

# Parsing de argumentos
parse_args() {
  local env="" verbose=false

  while [[ $# -gt 0 ]]; do
    case "$1" in
      -e|--env)
        env="$2"
        shift 2
        ;;
      -v|--verbose)
        verbose=true
        shift
        ;;
      -h|--help)
        usage 0
        ;;
      --)
        shift
        break
        ;;
      -*)
        die "Unknown option: $1"
        ;;
      *)
        break
        ;;
    esac
  done

  # Validar argumentos requeridos
  [[ -z "$env" ]] && die "Missing required option: --env"
  [[ "$env" =~ ^(dev|staging|production)$ ]] || die "Invalid env: $env"

  readonly ENV="$env"
  readonly VERBOSE="$verbose"
}
```

## Archivos y Paths

### Operaciones seguras

```bash
# Siempre verificar antes de operar
[[ -f "$file" ]] || die "File not found: $file"
[[ -d "$dir" ]] || die "Directory not found: $dir"
[[ -r "$file" ]] || die "File not readable: $file"
[[ -w "$file" ]] || die "File not writable: $file"
[[ -x "$script" ]] || die "Script not executable: $script"

# Paths absolutos con realpath
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Archivos temporales seguros
TEMP_FILE="$(mktemp "${TMPDIR:-/tmp}/myapp.XXXXXXXXXX")"
```

### Heredocs

```bash
# Para generar archivos de configuracion
cat > "$CONFIG_FILE" <<EOF
host=$DB_HOST
port=$DB_PORT
database=$DB_NAME
EOF

# Heredoc sin interpolacion (comillas en el delimitador)
cat > "$SCRIPT_FILE" <<'EOF'
#!/bin/bash
echo "This $variable won't be expanded"
EOF
```

## Patrones Comunes

### Script de setup/bootstrap

```bash
#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source helpers
source "$SCRIPT_DIR/lib/logging.sh"

main() {
  log "Setting up project..."

  check_dependencies
  setup_env
  install_tools

  log "Setup complete!"
}

check_dependencies() {
  local deps=("docker" "git" "curl" "jq")
  for dep in "${deps[@]}"; do
    command -v "$dep" &>/dev/null || die "Missing dependency: $dep"
  done
}

setup_env() {
  if [[ ! -f "$PROJECT_ROOT/.env" ]]; then
    cp "$PROJECT_ROOT/.env.example" "$PROJECT_ROOT/.env"
    log "Created .env from .env.example"
  fi
}

install_tools() {
  if ! command -v "bd" &>/dev/null; then
    log "Installing bd..."
    # install bd
  fi
}

main "$@"
```

### Health check script

```bash
#!/usr/bin/env bash
set -euo pipefail

check_service() {
  local name="$1" url="$2" expected_status="${3:-200}"
  local status

  status=$(curl -sf -o /dev/null -w "%{http_code}" "$url" 2>/dev/null) || status="000"

  if [[ "$status" == "$expected_status" ]]; then
    log "OK: $name ($url) -> $status"
    return 0
  else
    error "FAIL: $name ($url) -> $status (expected $expected_status)"
    return 1
  fi
}

main() {
  local failed=0

  check_service "API" "http://localhost:8000/health" || (( failed++ ))
  check_service "Frontend" "http://localhost:4321" || (( failed++ ))

  if (( failed > 0 )); then
    die "$failed service(s) unhealthy"
  fi

  log "All services healthy"
}

main "$@"
```

### Git hook

```bash
#!/usr/bin/env bash
set -euo pipefail

# .git/hooks/pre-commit
# Verificar que no se commiteen secrets ni archivos prohibidos

readonly FORBIDDEN_PATTERNS=(
  '\.env$'
  'credentials\.json'
  '\.pem$'
  'password\s*='
)

main() {
  local found=false

  for pattern in "${FORBIDDEN_PATTERNS[@]}"; do
    if git diff --cached --name-only | grep -qE "$pattern"; then
      error "Forbidden file pattern detected: $pattern"
      found=true
    fi
  done

  if [[ "$found" == true ]]; then
    die "Remove forbidden files before committing"
  fi
}

main
```

## Portabilidad

### Compatibilidad entre sistemas

```bash
# Detectar OS
detect_os() {
  case "$(uname -s)" in
    Linux*)   echo "linux" ;;
    Darwin*)  echo "macos" ;;
    MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
    *)        echo "unknown" ;;
  esac
}

# Comandos con diferencias entre OS
if [[ "$(detect_os)" == "macos" ]]; then
  SED_INPLACE="sed -i ''"
  DATE_CMD="gdate"  # brew install coreutils
else
  SED_INPLACE="sed -i"
  DATE_CMD="date"
fi

# Verificar version minima de bash
if (( BASH_VERSINFO[0] < 4 )); then
  die "Bash 4+ is required (found: $BASH_VERSION)"
fi
```

## ShellCheck

### Siempre ejecutar ShellCheck

```bash
# Instalar
brew install shellcheck          # macOS
apt-get install shellcheck       # Debian/Ubuntu

# Ejecutar
shellcheck script.sh
shellcheck -x script.sh          # Seguir source/includes
shellcheck -s bash script.sh     # Forzar shell dialect

# En CI
shellcheck scripts/*.sh

# Ignorar un warning especifico (con justificacion)
# shellcheck disable=SC2086  # Word splitting intencional aqui
echo $variable
```

### Directivas inline

```bash
# Desactivar regla para una linea
# shellcheck disable=SC2155
export MY_VAR="$(compute_value)"

# Desactivar regla para todo el archivo (al inicio)
# shellcheck disable=SC2034  # Variables usadas en source
```

## Testing

### Patron basico de test

```bash
#!/usr/bin/env bash
set -euo pipefail

# test_mylib.sh
source "$(dirname "$0")/../lib/mylib.sh"

test_count=0
fail_count=0

assert_eq() {
  local expected="$1" actual="$2" msg="${3:-}"
  (( test_count++ ))
  if [[ "$expected" == "$actual" ]]; then
    echo "  PASS: $msg"
  else
    echo "  FAIL: $msg (expected='$expected', got='$actual')"
    (( fail_count++ ))
  fi
}

assert_exit_code() {
  local expected="$1"
  shift
  (( test_count++ ))
  if "$@" >/dev/null 2>&1; then
    local actual=0
  else
    local actual=$?
  fi
  if [[ "$expected" == "$actual" ]]; then
    echo "  PASS: exit code $expected for: $*"
  else
    echo "  FAIL: expected exit $expected, got $actual for: $*"
    (( fail_count++ ))
  fi
}

echo "=== Running tests ==="

# Tests aqui
assert_eq "hello" "$(my_function "hello")" "my_function returns input"
assert_exit_code 0 command_exists "bash"
assert_exit_code 1 command_exists "nonexistent_command_xyz"

echo ""
echo "Results: $test_count tests, $fail_count failures"
exit "$fail_count"
```

## Mejores Practicas

### DO

- Usar `set -euo pipefail` en todos los scripts
- Hacer quote de TODAS las variables: `"$var"` nunca `$var`
- Usar `[[ ]]` en vez de `[ ]` para condicionales
- Declarar variables como `local` dentro de funciones
- Usar `readonly` para constantes
- Implementar trap EXIT para cleanup
- Ejecutar ShellCheck en CI
- Documentar funciones con comentarios
- Usar `mktemp` para archivos temporales
- Preferir `printf` sobre `echo` para output portable

### DON'T

- Parsear output de `ls` — usar globbing o `find -print0`
- Usar backticks — usar `$(command)` en su lugar
- Evaluar input del usuario con `eval` — inyeccion de codigo
- Hardcodear paths absolutos — usar variables y `dirname`
- Ignorar exit codes — verificar o usar `set -e`
- Hacer `cd` sin verificar exito — `cd "$dir" || die "..."`
- Usar `cat file | grep` — usar `grep pattern file` directo
- Omitir el shebang `#!/usr/bin/env bash`
- Escribir scripts largos sin funciones — modularizar
- Commitear scripts sin ejecutar ShellCheck

## Recursos

- [ShellCheck](https://www.shellcheck.net/)
- [Google Shell Style Guide](https://google.github.io/styleguide/shellguide.html)
- [Bash Strict Mode](http://redsymbol.net/articles/unofficial-bash-strict-mode/)
- [Pure Bash Bible](https://github.com/dylanaraps/pure-bash-bible)
- [Advanced Bash-Scripting Guide](https://tldp.org/LDP/abs/html/)
