---
name: jsonnet-best-practices
description: Jsonnet templating for Kubernetes manifests, replacing complex YAML and Helm logic
version: 1.0.0
tags:
  - best-practices
  - jsonnet
  - kubernetes
  - templating
  - kustomize
  - manifests
---

# jsonnet-best-practices

Mejores practicas para Jsonnet: generar manifests de Kubernetes reduciendo la repeticion de YAML, eliminando la logica complicada de Helm, y compilando a JSON/YAML para kubectl apply.

## Overview

Jsonnet es un lenguaje de datos que extiende JSON con:
- **Funciones y variables**: Logica real en vez de hacks de YAML
- **Imports**: Reutilizar codigo entre archivos sin copiar/pegar
- **Mixins y herencia**: Componer objetos complejos con `+` y `+:`
- **Generacion de multiples manifests**: Un archivo genera N recursos
- **Compilar a JSON/YAML**: Output directo para `kubectl apply -f -`

### Por que Jsonnet en vez de Helm o Kustomize puro

- **Helm**: Los templates de Go son dificiles de debuggear, la logica condicional se vuelve spaghetti, y mantener charts complejos es un dolor
- **Kustomize**: Bueno para patches simples, pero para logica condicional compleja (loops, funciones, defaults) se queda corto
- **Jsonnet**: Es un lenguaje real — tiene funciones, imports, herencia, validacion. Escribis logica de verdad en vez de pelear con `{{ if .Values.something }}`

El workflow es: **escribir en Jsonnet -> compilar -> pipe a kubectl**.

## Instalacion

```bash
# macOS
brew install jsonnet

# Arch/Manjaro
pacman -S jsonnet

# Go install
go install github.com/google/go-jsonnet/cmd/jsonnet@latest
go install github.com/google/go-jsonnet/cmd/jsonnetfmt@latest

# Verificar
jsonnet --version
jsonnetfmt --version
```

### Herramientas complementarias

```bash
# jsonnet-bundler — package manager para jsonnet (como npm)
go install github.com/jsonnet-bundler/jsonnet-bundler/cmd/jb@latest

# kubecfg — alternativa a jsonnet CLI con soporte nativo de k8s
go install github.com/kubecfg/kubecfg@latest
```

## Estructura de Proyecto

### Layout recomendado

```
k8s/
├── lib/                         # Librerias compartidas
│   ├── k8s.libsonnet            # Helpers de Kubernetes
│   ├── defaults.libsonnet       # Defaults del proyecto
│   └── utils.libsonnet          # Funciones utilitarias
├── components/                  # Componentes reutilizables
│   ├── deployment.libsonnet
│   ├── service.libsonnet
│   ├── ingress.libsonnet
│   ├── configmap.libsonnet
│   ├── secret.libsonnet
│   └── hpa.libsonnet
├── apps/                        # Aplicaciones concretas
│   ├── api.jsonnet
│   ├── worker.jsonnet
│   └── frontend.jsonnet
├── environments/                # Valores por ambiente
│   ├── dev.libsonnet
│   ├── staging.libsonnet
│   └── production.libsonnet
├── vendor/                      # Dependencias (via jb)
│   └── ...
├── jsonnetfile.json             # Dependencias (como package.json)
├── jsonnetfile.lock.json
└── Makefile                     # Comandos de compilacion y deploy
```

## Fundamentos de Jsonnet

### Sintaxis basica

```jsonnet
// comentarios con //

// Variables locales
local name = "api";
local replicas = 3;

// Objetos (como JSON pero con trailing commas y sin quotes en keys)
{
  apiVersion: "apps/v1",
  kind: "Deployment",
  metadata: {
    name: name,
    namespace: "myapp",
  },
  spec: {
    replicas: replicas,
  },
}
```

### Funciones

```jsonnet
// Funcion que genera labels
local labels(app, version="v1") = {
  app: app,
  version: version,
  "app.kubernetes.io/name": app,
  "app.kubernetes.io/version": version,
  "app.kubernetes.io/managed-by": "jsonnet",
};

// Usar
{
  metadata: {
    labels: labels("api", "v2"),
  },
}
```

### Herencia con + (merge profundo)

```jsonnet
// Base
local base = {
  metadata: {
    namespace: "default",
    labels: { team: "platform" },
  },
};

// Override — + hace merge profundo, +: hace merge del campo
base + {
  metadata+: {
    namespace: "production",    // override
    labels+: {                  // merge (mantiene team, agrega env)
      env: "production",
    },
  },
}

// Resultado:
// {
//   metadata: {
//     namespace: "production",
//     labels: { team: "platform", env: "production" },
//   },
// }
```

### Imports

```jsonnet
// Importar libsonnet
local k = import 'lib/k8s.libsonnet';
local defaults = import 'environments/production.libsonnet';

// Importar JSON/YAML existente
local config = import 'config.json';

// Importar texto plano
local readme = importstr 'README.md';
```

### Condicionales y loops

```jsonnet
// Condicional
local config = {
  replicas: if env == "production" then 3 else 1,
  resources: if env == "production" then {
    requests: { cpu: "500m", memory: "512Mi" },
    limits: { cpu: "1", memory: "1Gi" },
  } else {
    requests: { cpu: "100m", memory: "128Mi" },
    limits: { cpu: "200m", memory: "256Mi" },
  },
};

// Array comprehension
local ports = [
  { name: svc, port: port }
  for [svc, port] in [["http", 80], ["grpc", 9090], ["metrics", 9100]]
];

// Object comprehension
local env_vars = {
  [key]: { name: key, value: values[key] }
  for key in std.objectFields(values)
};

// Filtrar
local production_services = [
  svc
  for svc in all_services
  if svc.tier == "production"
];
```

### Hidden fields (::) y late binding

```jsonnet
// Hidden fields no aparecen en el output pero se pueden referenciar
{
  _config:: {        // :: = hidden
    name: "api",
    port: 8000,
  },

  deployment: {
    metadata: { name: $._config.name },  // $ = root object
    spec: {
      template: {
        spec: {
          containers: [{
            name: $._config.name,
            ports: [{ containerPort: $._config.port }],
          }],
        },
      },
    },
  },
}
// Output solo tiene "deployment", no "_config"
```

## Componentes para Kubernetes

### lib/k8s.libsonnet — Helpers base

```jsonnet
// lib/k8s.libsonnet
{
  // Metadata estandar
  metadata(name, namespace, labels={}):: {
    metadata: {
      name: name,
      namespace: namespace,
      labels: {
        app: name,
        "app.kubernetes.io/name": name,
        "app.kubernetes.io/managed-by": "jsonnet",
      } + labels,
    },
  },

  // Container con defaults seguros
  container(name, image, port=8000):: {
    name: name,
    image: image,
    imagePullPolicy: "IfNotPresent",
    ports: [{ name: "http", containerPort: port, protocol: "TCP" }],
    securityContext: {
      allowPrivilegeEscalation: false,
      readOnlyRootFilesystem: true,
      capabilities: { drop: ["ALL"] },
    },
    volumeMounts: [{ name: "tmp", mountPath: "/tmp" }],
  },

  // Probes
  probes(path="/health", port="http"):: {
    startupProbe: {
      httpGet: { path: path, port: port },
      initialDelaySeconds: 5,
      periodSeconds: 5,
      failureThreshold: 30,
    },
    livenessProbe: {
      httpGet: { path: path, port: port },
      periodSeconds: 20,
      timeoutSeconds: 5,
      failureThreshold: 3,
    },
    readinessProbe: {
      httpGet: { path: path + "/ready", port: port },
      periodSeconds: 10,
      timeoutSeconds: 3,
      failureThreshold: 3,
    },
  },

  // Resources con presets
  resources(preset="small")::
    local presets = {
      small:  { requests: { cpu: "100m", memory: "128Mi" }, limits: { cpu: "500m", memory: "512Mi" } },
      medium: { requests: { cpu: "250m", memory: "256Mi" }, limits: { cpu: "1",    memory: "1Gi"   } },
      large:  { requests: { cpu: "500m", memory: "512Mi" }, limits: { cpu: "2",    memory: "2Gi"   } },
    };
    presets[preset],

  // Security context de Pod
  podSecurity(user=1000):: {
    runAsNonRoot: true,
    runAsUser: user,
    runAsGroup: user,
    fsGroup: user,
    seccompProfile: { type: "RuntimeDefault" },
  },
}
```

### components/deployment.libsonnet

```jsonnet
// components/deployment.libsonnet
local k = import '../lib/k8s.libsonnet';

function(config) {
  apiVersion: "apps/v1",
  kind: "Deployment",
  metadata: k.metadata(config.name, config.namespace, config.labels).metadata,
  spec: {
    replicas: config.replicas,
    revisionHistoryLimit: 5,
    strategy: {
      type: "RollingUpdate",
      rollingUpdate: { maxSurge: 1, maxUnavailable: 0 },
    },
    selector: {
      matchLabels: { app: config.name },
    },
    template: {
      metadata: {
        labels: { app: config.name } + config.labels,
        annotations: std.get(config, "annotations", {}),
      },
      spec: {
        serviceAccountName: config.name + "-sa",
        securityContext: k.podSecurity(),
        containers: [
          k.container(config.name, config.image, config.port)
          + k.probes(std.get(config, "healthPath", "/health"))
          + { resources: k.resources(std.get(config, "size", "small")) }
          + { env: std.get(config, "env", []) }
          + { envFrom: std.get(config, "envFrom", []) },
        ],
        volumes: [{ name: "tmp", emptyDir: {} }],
        topologySpreadConstraints: [{
          maxSkew: 1,
          topologyKey: "kubernetes.io/hostname",
          whenUnsatisfiable: "DoNotSchedule",
          labelSelector: { matchLabels: { app: config.name } },
        }],
      },
    },
  },
}
```

### components/service.libsonnet

```jsonnet
// components/service.libsonnet
local k = import '../lib/k8s.libsonnet';

function(config) {
  apiVersion: "v1",
  kind: "Service",
  metadata: k.metadata(config.name, config.namespace).metadata,
  spec: {
    type: std.get(config, "serviceType", "ClusterIP"),
    selector: { app: config.name },
    ports: [{
      name: "http",
      port: std.get(config, "servicePort", 80),
      targetPort: "http",
      protocol: "TCP",
    }],
  },
}
```

### components/ingress.libsonnet

```jsonnet
// components/ingress.libsonnet
local k = import '../lib/k8s.libsonnet';

function(config) {
  apiVersion: "networking.k8s.io/v1",
  kind: "Ingress",
  metadata: k.metadata(config.name + "-ingress", config.namespace).metadata + {
    annotations: {
      "nginx.ingress.kubernetes.io/ssl-redirect": "true",
      "cert-manager.io/cluster-issuer": "letsencrypt-prod",
    } + std.get(config, "ingressAnnotations", {}),
  },
  spec: {
    ingressClassName: "nginx",
    tls: [{
      hosts: [config.host],
      secretName: config.name + "-tls",
    }],
    rules: [{
      host: config.host,
      http: {
        paths: [{
          path: std.get(config, "ingressPath", "/"),
          pathType: "Prefix",
          backend: {
            service: {
              name: config.name,
              port: { name: "http" },
            },
          },
        }],
      },
    }],
  },
}
```

### components/hpa.libsonnet

```jsonnet
// components/hpa.libsonnet
function(config) {
  apiVersion: "autoscaling/v2",
  kind: "HorizontalPodAutoscaler",
  metadata: {
    name: config.name + "-hpa",
    namespace: config.namespace,
  },
  spec: {
    scaleTargetRef: {
      apiVersion: "apps/v1",
      kind: "Deployment",
      name: config.name,
    },
    minReplicas: std.get(config, "minReplicas", 2),
    maxReplicas: std.get(config, "maxReplicas", 10),
    metrics: [
      {
        type: "Resource",
        resource: {
          name: "cpu",
          target: { type: "Utilization", averageUtilization: 70 },
        },
      },
      {
        type: "Resource",
        resource: {
          name: "memory",
          target: { type: "Utilization", averageUtilization: 80 },
        },
      },
    ],
    behavior: {
      scaleUp: {
        stabilizationWindowSeconds: 60,
        policies: [{ type: "Pods", value: 2, periodSeconds: 60 }],
      },
      scaleDown: {
        stabilizationWindowSeconds: 300,
        policies: [{ type: "Pods", value: 1, periodSeconds: 120 }],
      },
    },
  },
}
```

## Environments — Valores por Ambiente

### environments/dev.libsonnet

```jsonnet
// environments/dev.libsonnet
{
  environment: "dev",
  namespace: "dev",
  replicas: 1,
  size: "small",
  domain: "dev.example.com",
  labels: {
    env: "dev",
    team: "platform",
  },
}
```

### environments/production.libsonnet

```jsonnet
// environments/production.libsonnet
{
  environment: "production",
  namespace: "production",
  replicas: 3,
  size: "large",
  domain: "example.com",
  labels: {
    env: "production",
    team: "platform",
  },
}
```

## Apps — Generar Manifests Completos

### apps/api.jsonnet

```jsonnet
// apps/api.jsonnet
// Genera TODOS los manifests del servicio API
local env = import '../environments/' + std.extVar('env') + '.libsonnet';
local deployment = import '../components/deployment.libsonnet';
local service = import '../components/service.libsonnet';
local ingress = import '../components/ingress.libsonnet';
local hpa = import '../components/hpa.libsonnet';

local config = env + {
  name: "api",
  image: "ghcr.io/org/api:" + std.extVar('tag'),
  port: 8000,
  host: "api." + env.domain,

  env: [
    { name: "LOG_LEVEL", value: if env.environment == "production" then "WARNING" else "DEBUG" },
    { name: "DATABASE_URL", valueFrom: { secretKeyRef: { name: "api-secrets", key: "database-url" } } },
  ],
  envFrom: [
    { configMapRef: { name: "api-config" } },
  ],

  annotations: {
    "prometheus.io/scrape": "true",
    "prometheus.io/port": "8000",
  },

  // HPA solo en production
  minReplicas: if env.environment == "production" then 3 else 1,
  maxReplicas: if env.environment == "production" then 20 else 3,
};

// Multi-document output — cada key se convierte en un manifest
{
  deployment: deployment(config),
  service: service(config),
  ingress: ingress(config),
  [if env.environment != "dev" then "hpa"]: hpa(config),
}
```

### apps/worker.jsonnet

```jsonnet
// apps/worker.jsonnet
local env = import '../environments/' + std.extVar('env') + '.libsonnet';
local deployment = import '../components/deployment.libsonnet';

local config = env + {
  name: "worker",
  image: "ghcr.io/org/worker:" + std.extVar('tag'),
  port: 9090,
  healthPath: "/healthz",
  replicas: if env.environment == "production" then 5 else 1,
  size: if env.environment == "production" then "medium" else "small",

  env: [
    { name: "QUEUE_URL", valueFrom: { secretKeyRef: { name: "worker-secrets", key: "queue-url" } } },
    { name: "CONCURRENCY", value: if env.environment == "production" then "10" else "2" },
  ],
};

// Worker solo necesita deployment (no service/ingress)
{
  deployment: deployment(config),
}
```

## Compilacion y Deploy

### Comandos basicos

```bash
# Compilar a JSON (default)
jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=v1.2.3 apps/api.jsonnet

# Compilar a YAML (mas legible)
jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=v1.2.3 apps/api.jsonnet \
  | yq -P

# Compilar multi-document y aplicar directo
jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=v1.2.3 -m /dev/stdout apps/api.jsonnet \
  | kubectl apply -f -

# Compilar a archivos separados
jsonnet -J vendor -J lib --ext-str env=staging --ext-str tag=v1.2.3 -m output/ apps/api.jsonnet

# Compilar y hacer diff antes de aplicar
jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=v1.2.3 -m /dev/stdout apps/api.jsonnet \
  | kubectl diff -f -
```

### Compilar multi-document YAML (stream)

```bash
# Convertir output JSON con multiples keys a un YAML stream separado por ---
# Esto es lo que kubectl espera para multi-resource apply
jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=v1.2.3 apps/api.jsonnet \
  | jq -r '.[]' | yq -P '.' \
  | kubectl apply -f -

# O usando kubecfg (entiende jsonnet nativo)
kubecfg update --ext-str env=production --ext-str tag=v1.2.3 apps/api.jsonnet
```

### Makefile

```makefile
# Makefile
JSONNET := jsonnet
JSONNET_FLAGS := -J vendor -J lib
ENV ?= dev
TAG ?= latest

# Compilar un app
.PHONY: compile
compile:
	$(JSONNET) $(JSONNET_FLAGS) --ext-str env=$(ENV) --ext-str tag=$(TAG) apps/$(APP).jsonnet

# Compilar y aplicar
.PHONY: deploy
deploy:
	$(JSONNET) $(JSONNET_FLAGS) --ext-str env=$(ENV) --ext-str tag=$(TAG) apps/$(APP).jsonnet \
		| jq -r '.[]' \
		| kubectl apply -f -

# Diff antes de aplicar
.PHONY: diff
diff:
	$(JSONNET) $(JSONNET_FLAGS) --ext-str env=$(ENV) --ext-str tag=$(TAG) apps/$(APP).jsonnet \
		| jq -r '.[]' \
		| kubectl diff -f -

# Compilar todo
.PHONY: compile-all
compile-all:
	@for app in apps/*.jsonnet; do \
		echo "=== Compiling $$(basename $$app .jsonnet) ==="; \
		$(JSONNET) $(JSONNET_FLAGS) --ext-str env=$(ENV) --ext-str tag=$(TAG) $$app; \
	done

# Deploy todo
.PHONY: deploy-all
deploy-all:
	@for app in apps/*.jsonnet; do \
		echo "=== Deploying $$(basename $$app .jsonnet) ==="; \
		$(JSONNET) $(JSONNET_FLAGS) --ext-str env=$(ENV) --ext-str tag=$(TAG) $$app \
			| jq -r '.[]' \
			| kubectl apply -f -; \
	done

# Formatear
.PHONY: fmt
fmt:
	find . -name '*.jsonnet' -o -name '*.libsonnet' | xargs jsonnetfmt -i

# Lint (verificar que compila)
.PHONY: lint
lint:
	@for app in apps/*.jsonnet; do \
		echo "Checking $$app..."; \
		$(JSONNET) $(JSONNET_FLAGS) --ext-str env=dev --ext-str tag=test $$app > /dev/null; \
	done
	@echo "All files OK"

# Instalar dependencias
.PHONY: deps
deps:
	jb install
```

### Uso del Makefile

```bash
# Compilar la API para staging
make compile APP=api ENV=staging TAG=v1.2.3

# Diff antes de deploy
make diff APP=api ENV=production TAG=v1.2.3

# Deploy
make deploy APP=api ENV=production TAG=v1.2.3

# Deploy todo para dev
make deploy-all ENV=dev TAG=latest

# Formatear todo
make fmt

# Verificar que todo compila
make lint
```

## Jsonnet-bundler (jb) — Dependencias

```bash
# Inicializar proyecto
jb init

# Instalar libreria (ej: ksonnet-lib para k8s helpers)
jb install github.com/jsonnet-libs/k8s-libsonnet/1.30@main

# Instalar libreria custom
jb install github.com/org/jsonnet-libs@main

# Actualizar dependencias
jb update

# El directorio vendor/ se llena con las librerias
# NUNCA editar vendor/ — se regenera con jb install
```

### jsonnetfile.json

```json
{
  "version": 1,
  "dependencies": [
    {
      "source": {
        "git": {
          "remote": "https://github.com/jsonnet-libs/k8s-libsonnet",
          "subdir": "1.30"
        }
      },
      "version": "main"
    }
  ],
  "legacyImports": true
}
```

## CI/CD con GitHub Actions

```yaml
name: K8s Deploy
on:
  push:
    branches: [main]
    paths: ["k8s/**"]
  pull_request:
    paths: ["k8s/**"]

permissions:
  id-token: write
  contents: read
  pull-requests: write

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install jsonnet
        run: |
          go install github.com/google/go-jsonnet/cmd/jsonnet@latest
          go install github.com/google/go-jsonnet/cmd/jsonnetfmt@latest
          go install github.com/jsonnet-bundler/jsonnet-bundler/cmd/jb@latest

      - name: Install dependencies
        run: jb install
        working-directory: k8s

      - name: Check formatting
        run: |
          find . -name '*.jsonnet' -o -name '*.libsonnet' | xargs jsonnetfmt --test
        working-directory: k8s

      - name: Validate compilation
        run: |
          for app in apps/*.jsonnet; do
            echo "Validating $(basename $app)..."
            jsonnet -J vendor -J lib --ext-str env=dev --ext-str tag=test "$app" > /dev/null
          done
        working-directory: k8s

  diff:
    if: github.event_name == 'pull_request'
    needs: validate
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install tools
        run: |
          go install github.com/google/go-jsonnet/cmd/jsonnet@latest
          go install github.com/jsonnet-bundler/jsonnet-bundler/cmd/jb@latest

      - uses: azure/setup-kubectl@v3

      - name: Configure kubeconfig
        run: echo "${{ secrets.KUBECONFIG }}" > /tmp/kubeconfig
        env:
          KUBECONFIG: /tmp/kubeconfig

      - name: Diff
        run: |
          jb install
          for app in apps/*.jsonnet; do
            echo "=== $(basename $app .jsonnet) ==="
            jsonnet -J vendor -J lib --ext-str env=staging --ext-str tag=${{ github.sha }} "$app" \
              | jq -r '.[]' \
              | kubectl diff -f - || true
          done
        working-directory: k8s

  deploy:
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    needs: validate
    runs-on: ubuntu-latest
    environment: production
    steps:
      - uses: actions/checkout@v4

      - name: Install tools
        run: |
          go install github.com/google/go-jsonnet/cmd/jsonnet@latest
          go install github.com/jsonnet-bundler/jsonnet-bundler/cmd/jb@latest

      - uses: azure/setup-kubectl@v3

      - name: Configure kubeconfig
        run: echo "${{ secrets.KUBECONFIG }}" > /tmp/kubeconfig
        env:
          KUBECONFIG: /tmp/kubeconfig

      - name: Deploy
        run: |
          jb install
          for app in apps/*.jsonnet; do
            echo "=== Deploying $(basename $app .jsonnet) ==="
            jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=${{ github.sha }} "$app" \
              | jq -r '.[]' \
              | kubectl apply -f -
          done
        working-directory: k8s
```

## Patrones Avanzados

### Mixin pattern — componer funcionalidad

```jsonnet
// Un mixin es un objeto parcial que se mergea con +
local monitoringMixin = {
  spec+: {
    template+: {
      metadata+: {
        annotations+: {
          "prometheus.io/scrape": "true",
          "prometheus.io/port": std.toString(self._port),
        },
      },
    },
  },
};

local loggingMixin = {
  spec+: {
    template+: {
      spec+: {
        containers: [
          super.containers[0] + {
            env+: [
              { name: "LOG_FORMAT", value: "json" },
              { name: "LOG_LEVEL", value: "info" },
            ],
          },
        ],
      },
    },
  },
};

// Componer
deployment(config) + monitoringMixin + loggingMixin
```

### Generar multiples apps desde una config

```jsonnet
// apps/all.jsonnet — generar todos los servicios
local env = import '../environments/' + std.extVar('env') + '.libsonnet';
local deployment = import '../components/deployment.libsonnet';
local service = import '../components/service.libsonnet';

local apps = {
  api:      { image: "ghcr.io/org/api",      port: 8000, size: "large"  },
  worker:   { image: "ghcr.io/org/worker",    port: 9090, size: "medium" },
  frontend: { image: "ghcr.io/org/frontend",  port: 3000, size: "small"  },
};

{
  [name + "-deployment"]: deployment(env + apps[name] + {
    name: name,
    image: apps[name].image + ":" + std.extVar('tag'),
  })
  for name in std.objectFields(apps)
} + {
  [name + "-service"]: service(env + apps[name] + { name: name })
  for name in std.objectFields(apps)
  if name != "worker"  // worker no necesita service
}
```

### Funciones de validacion

```jsonnet
// lib/utils.libsonnet
{
  // Validar que un campo exista
  require(config, field)::
    if !std.objectHas(config, field)
    then error "Required field '" + field + "' is missing"
    else config[field],

  // Validar que el port esta en rango
  validatePort(port)::
    if port < 1 || port > 65535
    then error "Invalid port: " + std.toString(port)
    else port,

  // Validar image tiene tag
  validateImage(image)::
    if std.length(std.findSubstr(":", image)) == 0
    then error "Image must include tag: " + image
    else image,
}
```

### Generar ConfigMaps desde archivos

```jsonnet
// Leer config files y empaquetarlos en un ConfigMap
{
  apiVersion: "v1",
  kind: "ConfigMap",
  metadata: {
    name: "nginx-config",
    namespace: "myapp",
  },
  data: {
    "nginx.conf": importstr '../configs/nginx.conf',
    "default.conf": importstr '../configs/default.conf',
  },
}
```

## Formateo

```bash
# Formatear un archivo
jsonnetfmt -i apps/api.jsonnet

# Formatear todo
find . -name '*.jsonnet' -o -name '*.libsonnet' | xargs jsonnetfmt -i

# Verificar formato (CI)
find . -name '*.jsonnet' -o -name '*.libsonnet' | xargs jsonnetfmt --test

# Opciones de formato
jsonnetfmt --indent 2 --max-blank-lines 1 --string-style d -i file.jsonnet
```

## Mejores Practicas

### DO

- Compilar Jsonnet y pipear a `kubectl apply -f -` en vez de escribir YAML a mano
- Usar funciones para encapsular patrones repetitivos (deployment, service, etc.)
- Separar environments en archivos `.libsonnet` con valores por ambiente
- Usar `std.extVar()` para valores que cambian por deploy (tag, env)
- Usar hidden fields (`::`) para config interna que no debe aparecer en output
- Usar `+:` para merge profundo de campos heredados
- Formatear con `jsonnetfmt` siempre — en CI y en pre-commit
- Validar que todos los archivos compilan en CI antes de deploy
- Usar `jb` para manejar dependencias de librerias
- Hacer `kubectl diff -f -` antes de apply para revisar cambios
- Mantener componentes genericos — la logica de negocio va en los `apps/*.jsonnet`
- Commitear `jsonnetfile.lock.json` para reproducibilidad

### DON'T

- Escribir YAML de Kubernetes a mano cuando Jsonnet puede generarlo
- Usar Helm para logica compleja — los templates de Go se vuelven ilegibles
- Mezclar Jsonnet con Kustomize patches — elegir uno como fuente de verdad
- Hardcodear valores que varian por ambiente — usar environments/
- Editar archivos en `vendor/` — se regeneran con `jb install`
- Usar `std.extVar` para todo — solo para valores que cambian por deploy
- Omitir validacion de compilacion en CI
- Generar manifests sin tag fijo de imagen — nunca `:latest`
- Ignorar el diff antes de apply en produccion
- Crear componentes demasiado genericos — si tiene 20 parametros, partirlo

## Recursos

- [Jsonnet Language Reference](https://jsonnet.org/ref/language.html)
- [Jsonnet Standard Library](https://jsonnet.org/ref/stdlib.html)
- [Jsonnet Tutorial](https://jsonnet.org/learning/tutorial.html)
- [jsonnet-bundler (jb)](https://github.com/jsonnet-bundler/jsonnet-bundler)
- [kubecfg](https://github.com/kubecfg/kubecfg) — Herramienta de deploy que entiende Jsonnet
- [k8s-libsonnet](https://github.com/jsonnet-libs/k8s-libsonnet) — Libreria de tipos K8s para Jsonnet
- [Tanka (Grafana)](https://tanka.dev/) — Framework de deploy K8s basado en Jsonnet
