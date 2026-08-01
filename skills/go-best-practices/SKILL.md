---
name: go-best-practices
description: Go best practices — project layout, error handling, concurrency, interfaces and testing
version: 1.0.0
tags:
  - best-practices
  - go
  - golang
  - cli
  - backend
---

# go-best-practices

Mejores prácticas para desarrollo en Go: layout de proyecto, manejo de errores,
concurrencia con goroutines/channels, interfaces pequeñas y testing idiomático.

## Overview

Go es ideal para:
- **CLIs** y binarios estáticos distribuibles sin runtime
- **Servicios/APIs** concurrentes de alto rendimiento
- **Herramientas de infra** y workers

## Project Structure

```
myapp/
├── cmd/
│   └── myapp/
│       └── main.go         # entrypoint fino
├── internal/               # código privado (no importable fuera del módulo)
│   ├── config/
│   ├── server/
│   └── store/
├── pkg/                    # código público reutilizable (opcional)
├── go.mod
├── go.sum
└── Makefile
```

- `internal/` para todo lo que no querés exponer como API pública.
- `cmd/<bin>/main.go` mínimo: parsea flags y delega.
- Un módulo por repo (`go.mod`); nombres de paquete cortos y en minúscula.

## Error Handling

```go
func loadConfig(path string) (*Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("read config %s: %w", path, err)
    }
    // ...
    return cfg, nil
}
```

- Envolvé con `fmt.Errorf("...: %w", err)` para preservar la cadena.
- Inspeccioná con `errors.Is` (valor centinela) y `errors.As` (tipo).
- Definí errores centinela con `errors.New`; errores ricos como structs que
  implementan `error`.
- No ignores errores (`_ = f()`) salvo que sea deliberado y comentado.

```go
var ErrNotFound = errors.New("not found")

if errors.Is(err, ErrNotFound) { /* 404 */ }
```

## Interfaces

- **Interfaces pequeñas**, definidas por el consumidor, no por el proveedor.
- Aceptá interfaces, devolvé structs concretos.

```go
type Store interface {
    Get(ctx context.Context, id string) (*Item, error)
}

func NewHandler(s Store) *Handler { return &Handler{store: s} }
```

## Concurrencia

```go
func fetchAll(ctx context.Context, urls []string) ([]Result, error) {
    g, ctx := errgroup.WithContext(ctx)
    results := make([]Result, len(urls))
    sem := make(chan struct{}, 10) // límite de concurrencia

    for i, u := range urls {
        i, u := i, u
        g.Go(func() error {
            sem <- struct{}{}
            defer func() { <-sem }()
            r, err := fetch(ctx, u)
            if err != nil {
                return err
            }
            results[i] = r
            return nil
        })
    }
    return results, g.Wait()
}
```

- Pasá `context.Context` como primer parámetro para cancelación/timeouts.
- "Don't communicate by sharing memory; share memory by communicating" — usá channels.
- Protegé estado compartido con `sync.Mutex`; verificá con `go test -race`.
- Evitá goroutine leaks: toda goroutine debe tener una salida clara (ctx/close).

## Testing

```go
func TestParse(t *testing.T) {
    tests := []struct {
        name    string
        in      string
        want    int
        wantErr bool
    }{
        {"valid", "42", 42, false},
        {"empty", "", 0, true},
    }
    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got, err := Parse(tt.in)
            if (err != nil) != tt.wantErr {
                t.Fatalf("err=%v wantErr=%v", err, tt.wantErr)
            }
            if got != tt.want {
                t.Errorf("got %d want %d", got, tt.want)
            }
        })
    }
}
```

- **Table-driven tests** + subtests con `t.Run`.
- `go test ./... -race -cover`.
- `testing.T.TempDir()` para archivos temporales; `t.Cleanup` para teardown.

## Tooling

```bash
go build ./...
go test ./... -race -cover
go vet ./...
gofmt -l -w .        # o `go fmt ./...`
```

- Formateo obligatorio con `gofmt`/`goimports` (no hay debate de estilo).
- `go vet` + `staticcheck`/`golangci-lint` en CI.
- Fijá versión con la directiva `go 1.xx` en `go.mod`.

## DO / DON'T

**DO**
- Envolver errores con `%w` y chequear con `errors.Is/As`.
- Pasar `context.Context` para I/O y cancelación.
- Interfaces chicas del lado del consumidor.
- Correr tests con `-race`.

**DON'T**
- Ignorar errores silenciosamente.
- Usar `panic` para control de flujo (reservalo para lo irrecuperable).
- Sobre-abstraer con interfaces grandes/anticipadas.
- Lanzar goroutines sin una condición de salida.

## Recursos

- [Effective Go](https://go.dev/doc/effective_go)
- [Go Code Review Comments](https://go.dev/wiki/CodeReviewComments)
- [Standard Go Project Layout](https://github.com/golang-standards/project-layout)
- [Uber Go Style Guide](https://github.com/uber-go/guide/blob/master/style.md)
