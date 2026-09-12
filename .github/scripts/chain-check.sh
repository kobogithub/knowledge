#!/usr/bin/env bash
#
# chain-check — hace duras las compuertas de la cadena de producto.
#
# El criterio de éxito 2 de docs/product/PROJECT.md pide que ningún agente derive un
# artefacto sobre otro sin firmar, y la mitigación del riesgo "las compuertas se saltean"
# es que la verifique el agente y no la memoria del humano. Un agente se deja convencer;
# esto no.
#
# Verifica dos cosas (las otras dos del plan quedan para EPIC-02):
#
#   1. FIRMAS COHERENTES — todo artefacto con Estado "Aprobado" tiene "Firmado por" y
#      "Fecha de firma" con contenido. Un "Aprobado" sin firmante es una compuerta que
#      dice que pasó algo que no pasó.
#
#   3. TRAZABILIDAD story <-> spec — todo US-NN que un spec referencia existe como
#      archivo, y toda story de la épica del spec aparece referenciada en él. Es el
#      riesgo #1 registrado en PROJECT.md: que las dos capas diverjan en silencio.
#
# Uso:
#   bash .github/scripts/chain-check.sh
#
# Sale 0 si la cadena es consistente, 1 si no. En GitHub Actions emite anotaciones
# ::error:: para que la falla aparezca sobre el archivo en el diff del PR.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

ERRORES=0
CHEQUEADOS=0

# Anotación de error. En Actions usa el formato de workflow command para que GitHub la
# ancle al archivo; fuera de Actions imprime una línea legible.
error() {
    local archivo="$1" mensaje="$2"
    if [ -n "${GITHUB_ACTIONS:-}" ]; then
        printf '::error file=%s::%s\n' "$archivo" "$mensaje"
    fi
    printf '  ✗ %s: %s\n' "$archivo" "$mensaje" >&2
    ERRORES=$((ERRORES + 1))
}

ok() {
    printf '  ✓ %s\n' "$1"
}

# Devuelve el valor de un campo del encabezado (**Campo**: valor), sin espacios al borde.
# Vacío si el campo no está o no tiene contenido.
campo() {
    local archivo="$1" nombre="$2"
    grep -m1 "^\*\*${nombre}\*\*:" "$archivo" 2>/dev/null \
        | sed -e "s/^\*\*${nombre}\*\*:[[:space:]]*//" -e 's/[[:space:]]*$//' \
        || true
}

# ─────────────────────────────────────────────────────────────────────────────
# 1. Firmas coherentes
# ─────────────────────────────────────────────────────────────────────────────
#
# Alcance: los artefactos de la cadena en docs/product/ (brief, PRD, stories) y todo
# specs/*/spec.md que declare un encabezado de compuerta. Los specs anteriores a la capa
# de producto no lo declaran y quedan fuera sin marcar falso positivo.
#
# Quedan fuera a propósito:
#   - templates/  : son las plantillas, su "Estado" es un ejemplo
#   - discovery/  : input inmutable del cliente, no lleva firma
#   - README.md   : documenta la cadena, no es un artefacto de ella

chequear_firma() {
    local f="$1"
    local estado firmante fecha

    estado="$(campo "$f" 'Estado')"
    [ -n "$estado" ] || return 0

    CHEQUEADOS=$((CHEQUEADOS + 1))

    # "Estado" se compara por contenido, no por igualdad: "Borrador — pendiente de firma"
    # es un borrador. Misma regla que usa el skill product-gate.
    case "$estado" in
        *Aprobado*) ;;
        *) ok "$f — Estado: $estado (sin firma exigible)"; return 0 ;;
    esac

    firmante="$(campo "$f" 'Firmado por')"
    fecha="$(campo "$f" 'Fecha de firma')"

    if [ -z "$firmante" ]; then
        error "$f" 'dice Aprobado pero "Firmado por" está vacío o no existe. Una firma que no nombra a nadie no es una firma.'
    fi
    if [ -z "$fecha" ]; then
        error "$f" 'dice Aprobado pero "Fecha de firma" está vacía o no existe.'
    elif ! printf '%s' "$fecha" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}'; then
        error "$f" "\"Fecha de firma\" es \"$fecha\"; se espera YYYY-MM-DD."
    fi

    if [ -n "$firmante" ] && [ -n "$fecha" ]; then
        ok "$f — Aprobado, firmado por $firmante el ${fecha%% *}"
    fi
}

echo "1. Firmas coherentes"
echo

if [ -d docs/product ]; then
    while IFS= read -r f; do
        chequear_firma "$f"
    done < <(find docs/product -name '*.md' \
        -not -path 'docs/product/templates/*' \
        -not -path 'docs/product/discovery/*' \
        -not -name 'README.md' | sort)
else
    echo "  (no hay docs/product/ en este repo — nada que verificar)"
fi

while IFS= read -r f; do
    chequear_firma "$f"
done < <(find specs -mindepth 2 -maxdepth 2 -name 'spec.md' 2>/dev/null | sort)

echo
echo "2. Trazabilidad story <-> spec"
echo

# ─────────────────────────────────────────────────────────────────────────────
# 3. Trazabilidad story <-> spec
# ─────────────────────────────────────────────────────────────────────────────
#
# Un spec declara de qué épica se deriva con "**Épica**: EPIC-NN". Para esos specs:
#   a. todo US-NN que el spec menciona existe como docs/product/stories/EPIC-NN/US-NN.md
#   b. toda story de esa carpeta aparece mencionada en el spec
#
# El sentido (b) es el que atrapa el caso caro: una story firmada que el spec se olvidó
# de bajar, y que por lo tanto nadie va a implementar ni QA va a validar.

specs_con_epica=0

while IFS= read -r spec; do
    epica="$(campo "$spec" 'Épica' | grep -oE 'EPIC-[0-9]+' | head -1 || true)"
    [ -n "$epica" ] || continue

    specs_con_epica=$((specs_con_epica + 1))
    dir_stories="docs/product/stories/$epica"

    if [ ! -d "$dir_stories" ]; then
        error "$spec" "declara $epica pero no existe $dir_stories/. Un spec sin stories no tiene de dónde derivarse."
        continue
    fi

    # (a) IDs referenciados que no existen como archivo
    huerfanos=0
    while IFS= read -r id; do
        [ -n "$id" ] || continue
        if [ ! -f "$dir_stories/$id.md" ]; then
            error "$spec" "referencia $id, que no existe como $dir_stories/$id.md."
            huerfanos=$((huerfanos + 1))
        fi
    done < <(grep -oE 'US-[0-9]{2,}' "$spec" | sort -u)

    # (b) stories de la épica que el spec no referencia
    sin_bajar=0
    while IFS= read -r archivo; do
        [ -n "$archivo" ] || continue
        id="$(basename "$archivo" .md)"
        if ! grep -qE "\\b$id\\b" "$spec"; then
            error "$archivo" "es una story de $epica que $spec no referencia. O el spec la bajó y no la nombró, o la story quedó sin implementar."
            sin_bajar=$((sin_bajar + 1))
        fi
    done < <(find "$dir_stories" -name 'US-*.md' | sort)

    if [ "$huerfanos" -eq 0 ] && [ "$sin_bajar" -eq 0 ]; then
        total="$(find "$dir_stories" -name 'US-*.md' | wc -l | tr -d ' ')"
        ok "$spec <-> $epica ($total stories, correspondencia completa)"
    fi
done < <(find specs -mindepth 2 -maxdepth 2 -name 'spec.md' 2>/dev/null | sort)

if [ "$specs_con_epica" -eq 0 ]; then
    echo "  (ningún spec declara épica todavía — nada que trazar)"
fi

echo
if [ "$ERRORES" -gt 0 ]; then
    printf 'chain-check: %d problema(s) en la cadena. %d artefacto(s) con encabezado revisados.\n' \
        "$ERRORES" "$CHEQUEADOS" >&2
    exit 1
fi

printf 'chain-check: cadena consistente. %d artefacto(s) con encabezado revisados, %d spec(s) trazados.\n' \
    "$CHEQUEADOS" "$specs_con_epica"
