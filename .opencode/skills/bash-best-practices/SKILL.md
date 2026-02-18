---
name: bash-best-practices
description: Bash scripting best practices for automation and DevOps
version: 1.0.0
author: Knowledge Framework
tags: [bash, shell, scripting, automation, devops]
---

# Bash Best Practices

Expert guidelines for writing robust, maintainable shell scripts.

## Core Principles

### 1. Script Headers (Critical)

```bash
#!/usr/bin/env bash
set -euo pipefail  # Exit on error, undefined vars, pipe failures
IFS=$'\n\t'        # Safe word splitting

# Script metadata
# Description: Deploy application to production
# Usage: ./deploy.sh [environment]
# Author: Your Name
# Version: 1.0.0
```

### 2. Error Handling (Critical)

```bash
# ✅ GOOD: Check exit codes
if ! command_that_might_fail; then
    echo "Error: Command failed" >&2
    exit 1
fi

# ✅ GOOD: Trap errors
trap 'echo "Error on line $LINENO" >&2' ERR
trap 'cleanup' EXIT

cleanup() {
    # Remove temp files, close connections, etc.
    rm -f /tmp/my-script.$$.*
}

# ❌ BAD: Ignoring errors
command_that_might_fail
do_something_else
```

### 3. Variables & Quoting (Critical)

```bash
# ✅ GOOD: Quote variables
file="my document.txt"
rm "$file"              # Works with spaces
echo "${USER:-nobody}"  # Default value

# ✅ GOOD: Uppercase for env/constants
readonly API_KEY="${API_KEY:-}"
readonly MAX_RETRIES=3

# ✅ GOOD: Lowercase for local vars
local temp_file="/tmp/data.$$"

# ❌ BAD: Unquoted (word splitting!)
rm $file  # Breaks with spaces
```

### 4. Functions (High)

```bash
# ✅ GOOD: Descriptive functions with locals
deploy_application() {
    local env="${1:?Environment required}"
    local version="${2:-latest}"
    
    echo "Deploying $version to $env..."
    
    if ! build_image "$version"; then
        log_error "Build failed"
        return 1
    fi
    
    push_to_registry "$version"
}

# Call with error checking
if ! deploy_application "production" "v1.2.3"; then
    exit 1
fi
```

### 5. Command Substitution (High)

```bash
# ✅ GOOD: $() is preferred
current_user=$(whoami)
file_count=$(find . -type f | wc -l)

# ❌ BAD: Backticks (harder to nest)
current_user=`whoami`

# ✅ Read command output into array
mapfile -t files < <(find . -name "*.txt")
for file in "${files[@]}"; do
    process "$file"
done
```

### 6. Conditionals (High)

```bash
# ✅ GOOD: [[ ]] for conditionals (bash-specific, more features)
if [[ -f "$file" ]]; then
    echo "File exists"
fi

if [[ "$string" =~ ^[0-9]+$ ]]; then
    echo "Is number"
fi

if [[ "$var" == "value" ]]; then
    echo "Match"
fi

# File tests
[[ -f file ]]  # Regular file
[[ -d dir ]]   # Directory
[[ -x bin ]]   # Executable
[[ -z "$var" ]]  # Empty string
[[ -n "$var" ]]  # Non-empty string

# ❌ BAD: [ ] for new scripts (use in POSIX sh only)
if [ -f $file ]; then  # Also missing quotes!
    echo "Bad"
fi
```

### 7. Loops (High)

```bash
# ✅ GOOD: While read loop
while IFS= read -r line; do
    process "$line"
done < file.txt

# ✅ GOOD: For loop over array
files=( *.txt )
for file in "${files[@]}"; do
    echo "Processing: $file"
done

# ✅ GOOD: C-style loop
for ((i=0; i<10; i++)); do
    echo "$i"
done

# ❌ BAD: Parsing ls output
for file in $(ls *.txt); do  # Breaks with spaces!
    echo "$file"
done
```

### 8. Input Validation (High)

```bash
# ✅ GOOD: Validate arguments
main() {
    local action="${1:?Action required (start|stop|restart)}"
    local service="${2:?Service name required}"
    
    case "$action" in
        start|stop|restart)
            "${action}_service" "$service"
            ;;
        *)
            echo "Error: Invalid action '$action'" >&2
            usage
            return 1
            ;;
    esac
}

usage() {
    cat <<EOF
Usage: $0 <action> <service>

Actions:
    start       Start the service
    stop        Stop the service
    restart     Restart the service

Examples:
    $0 start nginx
    $0 restart postgresql
EOF
}

main "$@"
```

### 9. Logging (Medium)

```bash
# ✅ GOOD: Logging functions
readonly LOG_FILE="/var/log/myapp.log"

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

log_error() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $*" | tee -a "$LOG_FILE" >&2
}

log_debug() {
    if [[ "${DEBUG:-0}" == "1" ]]; then
        echo "[$(date +'%Y-%m-%d %H:%M:%S')] DEBUG: $*" | tee -a "$LOG_FILE"
    fi
}

# Usage
log "Starting deployment"
log_error "Failed to connect to database"
log_debug "Connection string: $DB_URL"
```

### 10. Best Practices Checklist (High)

```bash
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# Constants
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly SCRIPT_NAME="$(basename "$0")"
readonly TEMP_DIR="$(mktemp -d)"

# Cleanup on exit
trap cleanup EXIT INT TERM

cleanup() {
    rm -rf "$TEMP_DIR"
}

# Main function
main() {
    local env="${1:?Environment required}"
    
    # Validate input
    if [[ ! "$env" =~ ^(dev|staging|prod)$ ]]; then
        echo "Error: Invalid environment" >&2
        return 1
    fi
    
    # Do work
    log "Deploying to $env"
    deploy "$env"
}

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*"
}

deploy() {
    local env="$1"
    # Implementation
}

# Run main if executed (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
```

## Common Patterns

### Retry Logic
```bash
retry() {
    local max_attempts="$1"
    shift
    local attempt=1
    
    until "$@"; do
        if ((attempt >= max_attempts)); then
            echo "Failed after $max_attempts attempts" >&2
            return 1
        fi
        echo "Attempt $attempt failed, retrying..." >&2
        sleep $((attempt * 2))
        ((attempt++))
    done
}

retry 5 curl -f https://api.example.com/health
```

### Parallel Execution
```bash
# Process files in parallel
parallel_process() {
    local max_jobs=4
    local job_count=0
    
    for file in *.txt; do
        process_file "$file" &
        ((job_count++))
        
        if ((job_count >= max_jobs)); then
            wait -n  # Wait for any job to finish
            ((job_count--))
        fi
    done
    
    wait  # Wait for remaining jobs
}
```

### Config File Parsing
```bash
# Read key=value config
while IFS='=' read -r key value; do
    # Skip comments and empty lines
    [[ "$key" =~ ^[[:space:]]*# ]] && continue
    [[ -z "$key" ]] && continue
    
    # Trim whitespace
    key="${key//[[:space:]]/}"
    value="${value//[[:space:]]/}"
    
    declare "$key=$value"
done < config.ini
```

## Common Pitfalls

❌ **Unquoted variables**: Word splitting breaks with spaces  
✅ Always quote: `"$var"`, `"${array[@]}"`

❌ **Parsing ls**: Breaks with special characters  
✅ Use glob patterns: `for file in *.txt`

❌ **Missing set -euo pipefail**: Silent failures  
✅ Always use at script top

❌ **Global variables everywhere**: Hard to maintain  
✅ Use `local` in functions, `readonly` for constants

❌ **No error handling**: Scripts fail silently  
✅ Check exit codes, use `trap`

❌ **cat | grep | awk**: Useless use of cat  
✅ Direct: `grep pattern file | awk ...`

## Useful Commands

```bash
# Check if command exists
command -v docker >/dev/null 2>&1 || { echo "Docker required" >&2; exit 1; }

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp file/directory
temp_file="$(mktemp)"
temp_dir="$(mktemp -d)"

# Read password securely
read -rsp "Password: " password
echo

# Parse JSON with jq
value=$(jq -r '.key' data.json)

# Find and process files
find . -name "*.log" -mtime +7 -delete

# Monitor command output
watch -n 5 'kubectl get pods'

# Parallel processing with xargs
find . -name "*.txt" | xargs -P 4 -I {} process {}
```

## ShellCheck Integration

```bash
# Install ShellCheck
sudo apt-get install shellcheck  # Debian/Ubuntu
brew install shellcheck          # macOS

# Check script
shellcheck script.sh

# Disable specific warnings
# shellcheck disable=SC2086
var=$unquoted_on_purpose

# Check all scripts
find . -name "*.sh" -exec shellcheck {} +
```

## Resources

- [ShellCheck](https://www.shellcheck.net/)
- [Bash Guide](https://mywiki.wooledge.org/BashGuide)
- [Google Shell Style Guide](https://google.github.io/styleguide/shellguide.html)
- [Bash Pitfalls](https://mywiki.wooledge.org/BashPitfalls)
