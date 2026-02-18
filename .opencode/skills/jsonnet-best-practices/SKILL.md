---
name: jsonnet-best-practices
description: Jsonnet templating language best practices for configuration management
version: 1.0.0
author: Knowledge Framework
tags: [jsonnet, configuration, templating, kubernetes, devops]
---

# Jsonnet Best Practices

Expert guidelines for using Jsonnet as a data templating language for configuration management, especially for Kubernetes manifests and infrastructure as code.

## Core Principles

### 1. Basic Syntax (Critical)

**Objects and Fields**
```jsonnet
// Basic object
{
  name: 'myapp',
  version: '1.0.0',
  replicas: 3,
  enabled: true,
}

// Nested objects
{
  metadata: {
    name: 'myapp',
    namespace: 'production',
    labels: {
      app: 'myapp',
      version: 'v1',
    },
  },
}

// Arrays
{
  ports: [80, 443, 8080],
  envVars: [
    { name: 'PORT', value: '8080' },
    { name: 'ENV', value: 'production' },
  ],
}
```

**String Concatenation**
```jsonnet
{
  // Simple concatenation
  fullName: 'my' + 'app',
  
  // With variables
  local name = 'myapp',
  local version = '1.0.0',
  image: name + ':' + version,
  
  // Text blocks
  config: |||
    server {
      listen 80;
      server_name example.com;
    }
  |||,
}
```

**Variables (local)**
```jsonnet
local appName = 'myapp';
local namespace = 'production';
local version = '1.0.0';

{
  metadata: {
    name: appName,
    namespace: namespace,
  },
  spec: {
    image: appName + ':' + version,
  },
}
```

### 2. Functions (High)

**Basic Functions**
```jsonnet
local createPort(port, name='http') = {
  name: name,
  port: port,
  targetPort: port,
};

{
  ports: [
    createPort(80, 'http'),
    createPort(443, 'https'),
    createPort(9090, 'metrics'),
  ],
}
```

**Functions with Default Parameters**
```jsonnet
local createContainer(
  name,
  image,
  port=8080,
  replicas=3,
  resources={}
) = {
  name: name,
  image: image,
  ports: [{ containerPort: port }],
  replicas: replicas,
} + if std.length(resources) > 0 then { resources: resources } else {};

{
  container1: createContainer('app', 'myapp:1.0'),
  container2: createContainer('api', 'myapi:2.0', port=3000, replicas=5),
}
```

**Higher-Order Functions**
```jsonnet
local map(fn, arr) = [fn(x) for x in arr];
local filter(fn, arr) = [x for x in arr if fn(x)];

local envVars = [
  { name: 'PORT', value: '8080' },
  { name: 'DEBUG', value: 'false' },
  { name: 'LOG_LEVEL', value: 'info' },
];

{
  // Extract just the names
  names: map(function(e) e.name, envVars),
  
  // Filter production vars
  prodVars: filter(function(e) e.name != 'DEBUG', envVars),
}
```

### 3. Imports and Libraries (Critical)

**File Imports**
```jsonnet
// config.libsonnet
{
  namespace: 'production',
  replicas: 3,
  image: {
    repository: 'myapp',
    tag: '1.0.0',
  },
}
```

```jsonnet
// deployment.jsonnet
local config = import 'config.libsonnet';

{
  apiVersion: 'apps/v1',
  kind: 'Deployment',
  metadata: {
    name: 'myapp',
    namespace: config.namespace,
  },
  spec: {
    replicas: config.replicas,
    template: {
      spec: {
        containers: [{
          name: 'app',
          image: config.image.repository + ':' + config.image.tag,
        }],
      },
    },
  },
}
```

**Library Structure (libsonnet)**
```jsonnet
// k8s.libsonnet - Kubernetes helpers library
{
  // Create a deployment
  deployment(name, namespace, image, replicas=3): {
    apiVersion: 'apps/v1',
    kind: 'Deployment',
    metadata: {
      name: name,
      namespace: namespace,
    },
    spec: {
      replicas: replicas,
      selector: {
        matchLabels: { app: name },
      },
      template: {
        metadata: {
          labels: { app: name },
        },
        spec: {
          containers: [{
            name: name,
            image: image,
          }],
        },
      },
    },
  },
  
  // Create a service
  service(name, namespace, port, targetPort=port): {
    apiVersion: 'v1',
    kind: 'Service',
    metadata: {
      name: name,
      namespace: namespace,
    },
    spec: {
      selector: { app: name },
      ports: [{
        port: port,
        targetPort: targetPort,
      }],
    },
  },
  
  // Create a namespace
  namespace(name): {
    apiVersion: 'v1',
    kind: 'Namespace',
    metadata: {
      name: name,
    },
  },
}
```

**Using the Library**
```jsonnet
// app.jsonnet
local k8s = import 'k8s.libsonnet';

[
  k8s.namespace('production'),
  k8s.deployment('myapp', 'production', 'myapp:1.0.0', replicas=5),
  k8s.service('myapp', 'production', 80, 8080),
]
```

### 4. Standard Library (High)

**String Functions**
```jsonnet
{
  // String manipulation
  upper: std.asciiUpper('hello'),           // 'HELLO'
  lower: std.asciiLower('WORLD'),           // 'world'
  split: std.split('a,b,c', ','),           // ['a', 'b', 'c']
  join: std.join('.', ['a', 'b', 'c']),     // 'a.b.c'
  
  // String formatting
  formatted: std.format('Hello %s!', 'World'),
  
  // String testing
  startsWith: std.startsWith('hello world', 'hello'),  // true
  endsWith: std.endsWith('hello.txt', '.txt'),         // true
}
```

**Array Functions**
```jsonnet
local arr = [1, 2, 3, 4, 5];

{
  length: std.length(arr),                    // 5
  slice: arr[1:3],                            // [2, 3]
  first: std.head(arr),                       // 1
  last: std.tail(arr),                        // [2, 3, 4, 5]
  reverse: std.reverse(arr),                  // [5, 4, 3, 2, 1]
  sort: std.sort([3, 1, 4, 1, 5]),           // [1, 1, 3, 4, 5]
  uniq: std.uniq([1, 2, 2, 3, 3, 3]),        // [1, 2, 3]
  
  // Functional
  map: std.map(function(x) x * 2, arr),       // [2, 4, 6, 8, 10]
  filter: std.filter(function(x) x > 2, arr), // [3, 4, 5]
  fold: std.foldl(function(a, b) a + b, arr, 0), // 15 (sum)
}
```

**Object Functions**
```jsonnet
local obj = { a: 1, b: 2, c: 3 };

{
  fields: std.objectFields(obj),                    // ['a', 'b', 'c']
  values: std.objectValues(obj),                    // [1, 2, 3]
  has: std.objectHas(obj, 'a'),                     // true
  
  // Object manipulation
  merged: obj + { d: 4 },                           // { a: 1, b: 2, c: 3, d: 4 }
  removed: std.prune({ a: 1, b: null, c: 3 }),     // { a: 1, c: 3 }
}
```

**Type Checking**
```jsonnet
{
  isString: std.isString('hello'),       // true
  isNumber: std.isNumber(42),            // true
  isBoolean: std.isBoolean(true),        // true
  isArray: std.isArray([1, 2, 3]),       // true
  isObject: std.isObject({ a: 1 }),      // true
  isFunction: std.isFunction(function(x) x), // true
}
```

### 5. Conditionals and Comprehensions (High)

**If-Then-Else**
```jsonnet
local env = 'production';

{
  replicas: if env == 'production' then 5 else 2,
  
  resources: if env == 'production' then {
    requests: { cpu: '1', memory: '2Gi' },
    limits: { cpu: '2', memory: '4Gi' },
  } else {
    requests: { cpu: '100m', memory: '256Mi' },
  },
}
```

**Array Comprehensions**
```jsonnet
{
  // Simple comprehension
  squares: [x * x for x in [1, 2, 3, 4, 5]],
  
  // With condition
  evens: [x for x in std.range(1, 10) if x % 2 == 0],
  
  // Multiple iterations
  pairs: [[x, y] for x in [1, 2, 3] for y in ['a', 'b']],
  
  // Generate env vars
  envVars: [
    { name: key, value: value }
    for [key, value] in std.objectKeysValues({
      PORT: '8080',
      DEBUG: 'false',
      LOG_LEVEL: 'info',
    })
  ],
}
```

**Object Comprehensions**
```jsonnet
{
  // Create object from array
  portsObj: {
    [name]: port
    for [name, port] in [
      ['http', 80],
      ['https', 443],
      ['metrics', 9090],
    ]
  },
  
  // Transform object
  uppercase: {
    [std.asciiUpper(key)]: value
    for [key, value] in std.objectKeysValues({ a: 1, b: 2 })
  },
}
```

### 6. Composition and Mixins (Critical)

**Object Composition**
```jsonnet
local base = {
  apiVersion: 'apps/v1',
  kind: 'Deployment',
  metadata: {
    name: 'myapp',
  },
};

local withReplicas(replicas) = {
  spec+: {
    replicas: replicas,
  },
};

local withImage(image) = {
  spec+: {
    template+: {
      spec+: {
        containers: [{
          name: 'app',
          image: image,
        }],
      },
    },
  },
};

// Compose
base + withReplicas(5) + withImage('myapp:1.0.0')
```

**Mixins Pattern**
```jsonnet
// mixins.libsonnet
{
  // Mixin for adding resources
  withResources(cpu, memory): {
    spec+: {
      template+: {
        spec+: {
          containers: [
            c + {
              resources: {
                requests: { cpu: cpu, memory: memory },
                limits: { cpu: cpu, memory: memory },
              },
            }
            for c in super.containers
          ],
        },
      },
    },
  },
  
  // Mixin for adding labels
  withLabels(labels): {
    metadata+: {
      labels+: labels,
    },
    spec+: {
      template+: {
        metadata+: {
          labels+: labels,
        },
      },
    },
  },
  
  // Mixin for adding annotations
  withAnnotations(annotations): {
    metadata+: {
      annotations+: annotations,
    },
  },
}
```

**Using Mixins**
```jsonnet
local k8s = import 'k8s.libsonnet';
local mixins = import 'mixins.libsonnet';

k8s.deployment('myapp', 'production', 'myapp:1.0.0')
+ mixins.withResources('500m', '512Mi')
+ mixins.withLabels({ team: 'backend', env: 'prod' })
+ mixins.withAnnotations({ 'prometheus.io/scrape': 'true' })
```

### 7. Multi-File Projects (High)

**Project Structure**
```
jsonnet/
├── lib/
│   ├── k8s.libsonnet      # Kubernetes primitives
│   ├── mixins.libsonnet   # Reusable mixins
│   └── utils.libsonnet    # Utility functions
├── environments/
│   ├── dev.libsonnet      # Dev config
│   ├── staging.libsonnet  # Staging config
│   └── prod.libsonnet     # Prod config
└── apps/
    ├── frontend.jsonnet   # Frontend app
    ├── backend.jsonnet    # Backend app
    └── database.jsonnet   # Database
```

**Environment Configuration**
```jsonnet
// environments/prod.libsonnet
{
  namespace: 'production',
  replicas: 5,
  resources: {
    requests: { cpu: '1', memory: '2Gi' },
    limits: { cpu: '2', memory: '4Gi' },
  },
  domain: 'example.com',
  tls: true,
}
```

```jsonnet
// apps/frontend.jsonnet
local env = import '../environments/prod.libsonnet';
local k8s = import '../lib/k8s.libsonnet';
local mixins = import '../lib/mixins.libsonnet';

local app = k8s.deployment('frontend', env.namespace, 'frontend:1.0.0')
  + mixins.withResources(env.resources.requests.cpu, env.resources.requests.memory)
  + { spec+: { replicas: env.replicas } };

local svc = k8s.service('frontend', env.namespace, 80, 3000);

{
  deployment: app,
  service: svc,
}
```

### 8. External Variables (High)

**Top-Level Arguments (TLA)**
```jsonnet
// app.jsonnet
function(env='dev', replicas=3, version='latest') {
  apiVersion: 'apps/v1',
  kind: 'Deployment',
  metadata: {
    name: 'myapp-' + env,
  },
  spec: {
    replicas: replicas,
    template: {
      spec: {
        containers: [{
          name: 'app',
          image: 'myapp:' + version,
        }],
      },
    },
  },
}
```

**Using TLA**
```bash
# From command line
jsonnet app.jsonnet \
  --tla-str env=production \
  --tla-code replicas=10 \
  --tla-str version=1.0.0

# From file
jsonnet app.jsonnet \
  --tla-code-file params.json
```

**External Variables (extVar)**
```jsonnet
// Use external variables
{
  namespace: std.extVar('namespace'),
  image: std.extVar('image'),
  replicas: std.parseInt(std.extVar('replicas')),
}
```

```bash
jsonnet app.jsonnet \
  --ext-str namespace=production \
  --ext-str image=myapp:1.0.0 \
  --ext-str replicas=5
```

### 9. Testing and Validation (Medium)

**Assertions**
```jsonnet
local replicas = 10;

{
  // Assert conditions
  assert replicas > 0 : 'Replicas must be positive',
  assert replicas <= 100 : 'Too many replicas',
  
  spec: {
    replicas: replicas,
  },
}
```

**Type Validation**
```jsonnet
local validateConfig(config) =
  assert std.isObject(config) : 'Config must be an object';
  assert std.objectHas(config, 'name') : 'Config must have name';
  assert std.isString(config.name) : 'Name must be string';
  assert std.isNumber(config.replicas) : 'Replicas must be number';
  config;

validateConfig({
  name: 'myapp',
  replicas: 3,
})
```

**Unit Testing Pattern**
```jsonnet
// tests/test_k8s.jsonnet
local k8s = import '../lib/k8s.libsonnet';

local tests = {
  test_deployment_has_apiVersion:
    local d = k8s.deployment('test', 'default', 'test:1.0');
    assert d.apiVersion == 'apps/v1' : 'API version mismatch';
    true,
  
  test_deployment_has_replicas:
    local d = k8s.deployment('test', 'default', 'test:1.0', replicas=5);
    assert d.spec.replicas == 5 : 'Replicas mismatch';
    true,
};

// Run all tests
std.foldl(
  function(acc, test) acc && tests[test],
  std.objectFields(tests),
  true
)
```

### 10. Kubernetes-Specific Patterns (Critical)

**Multi-Resource Manifests**
```jsonnet
local k8s = import 'k8s.libsonnet';

// Return array of resources
[
  k8s.namespace('myapp'),
  k8s.deployment('frontend', 'myapp', 'frontend:1.0'),
  k8s.service('frontend', 'myapp', 80),
  k8s.deployment('backend', 'myapp', 'backend:1.0'),
  k8s.service('backend', 'myapp', 8080),
]
```

**ConfigMap Generator**
```jsonnet
local createConfigMap(name, namespace, data) = {
  apiVersion: 'v1',
  kind: 'ConfigMap',
  metadata: {
    name: name,
    namespace: namespace,
  },
  data: data,
};

createConfigMap('app-config', 'production', {
  'app.properties': |||
    server.port=8080
    logging.level=INFO
  |||,
  LOG_LEVEL: 'info',
  DATABASE_HOST: 'postgres.default.svc.cluster.local',
})
```

**Secret Generator**
```jsonnet
local createSecret(name, namespace, data) = {
  apiVersion: 'v1',
  kind: 'Secret',
  metadata: {
    name: name,
    namespace: namespace,
  },
  type: 'Opaque',
  stringData: data,
};

createSecret('app-secrets', 'production', {
  DATABASE_PASSWORD: std.extVar('db_password'),
  API_KEY: std.extVar('api_key'),
})
```

### 11. Common Patterns (High)

**DRY Principle with Inheritance**
```jsonnet
local baseDeployment = {
  apiVersion: 'apps/v1',
  kind: 'Deployment',
  spec: {
    replicas: 3,
    template: {
      spec: {
        securityContext: {
          runAsNonRoot: true,
          runAsUser: 1000,
        },
        containers: [],
      },
    },
  },
};

// Extend base for specific apps
{
  frontend: baseDeployment + {
    metadata: { name: 'frontend' },
    spec+: {
      template+: {
        spec+: {
          containers: [{ name: 'frontend', image: 'frontend:1.0' }],
        },
      },
    },
  },
  
  backend: baseDeployment + {
    metadata: { name: 'backend' },
    spec+: {
      replicas: 5,  // Override
      template+: {
        spec+: {
          containers: [{ name: 'backend', image: 'backend:1.0' }],
        },
      },
    },
  },
}
```

**Factory Functions**
```jsonnet
local createApp(name, image, port, replicas=3) = {
  deployment: {
    apiVersion: 'apps/v1',
    kind: 'Deployment',
    metadata: { name: name },
    spec: {
      replicas: replicas,
      selector: { matchLabels: { app: name } },
      template: {
        metadata: { labels: { app: name } },
        spec: {
          containers: [{
            name: name,
            image: image,
            ports: [{ containerPort: port }],
          }],
        },
      },
    },
  },
  
  service: {
    apiVersion: 'v1',
    kind: 'Service',
    metadata: { name: name },
    spec: {
      selector: { app: name },
      ports: [{ port: 80, targetPort: port }],
    },
  },
};

// Use factory
{
  frontend: createApp('frontend', 'frontend:1.0', 3000),
  backend: createApp('backend', 'backend:1.0', 8080, replicas=5),
}
```

## Advanced Techniques

### Polymorphism with Functions
```jsonnet
local createResource(kind) = {
  create(name, namespace, spec): {
    apiVersion: 'v1',
    kind: kind,
    metadata: {
      name: name,
      namespace: namespace,
    },
    spec: spec,
  },
};

local serviceFactory = createResource('Service');
local configMapFactory = createResource('ConfigMap');

{
  svc: serviceFactory.create('myapp', 'default', {
    selector: { app: 'myapp' },
    ports: [{ port: 80 }],
  }),
}
```

### Lazy Evaluation
```jsonnet
// Define expensive computation lazily
local expensiveComputation = {
  result:: std.foldl(
    function(a, b) a + b,
    std.range(1, 1000000),
    0
  ),
};

{
  // Only computed if accessed
  value: expensiveComputation.result,
}
```

### Self-Reference with `self` and `super`
```jsonnet
{
  name: 'myapp',
  version: '1.0.0',
  
  // self refers to current object
  fullName: self.name + '-' + self.version,
  
  container: {
    image: $.name + ':' + $.version,  // $ refers to root
  },
}
```

## Tanka Integration

**Tanka Project Structure**
```
myproject/
├── environments/
│   ├── default/
│   │   ├── main.jsonnet
│   │   └── spec.json
│   └── production/
│       ├── main.jsonnet
│       └── spec.json
├── lib/
│   └── myapp.libsonnet
└── vendor/
    └── github.com/
        └── grafana/
            └── jsonnet-libs/
```

**Tanka Environment**
```jsonnet
// environments/production/main.jsonnet
local tanka = import 'github.com/grafana/jsonnet-libs/tanka-util/main.libsonnet';
local k = import 'github.com/grafana/jsonnet-libs/ksonnet-util/kausal.libsonnet';

{
  _config:: {
    namespace: 'production',
    cluster: 'prod-cluster',
  },
  
  myapp: (import '../../lib/myapp.libsonnet') {
    _config+:: $._config,
  },
}
```

## Common Anti-Patterns to Avoid

### ❌ Don't Do This:
1. **Deep nesting** - Use functions and composition instead
2. **Magic numbers** - Define constants at top
3. **String concatenation hell** - Use functions
4. **Copy-paste code** - Create reusable libraries
5. **No error handling** - Use assertions
6. **Inconsistent naming** - Follow conventions
7. **Mixing data and logic** - Separate concerns
8. **No documentation** - Add comments for complex logic
9. **Hardcoded values** - Use external variables
10. **No testing** - Write validation functions

## Best Practices Checklist

- [ ] Use `.libsonnet` extension for libraries
- [ ] Prefix private fields with `::` (hidden fields)
- [ ] Use `local` for variables that shouldn't appear in output
- [ ] Validate inputs with assertions
- [ ] Document complex functions with comments
- [ ] Use meaningful variable names
- [ ] Extract common patterns to libraries
- [ ] Keep files under 300 lines
- [ ] Use external variables for environment-specific values
- [ ] Test your Jsonnet code with assertions
- [ ] Use version control for libraries
- [ ] Follow consistent formatting (use `jsonnetfmt`)

## Tools and Workflow

**Formatting**
```bash
# Format a file
jsonnetfmt -i myfile.jsonnet

# Format all files
find . -name '*.jsonnet' -o -name '*.libsonnet' | xargs jsonnetfmt -i
```

**Evaluation**
```bash
# Basic eval
jsonnet myfile.jsonnet

# Multi-file output
jsonnet -m output/ myfile.jsonnet

# YAML output
jsonnet -y myfile.jsonnet

# With external variables
jsonnet --ext-str env=prod myfile.jsonnet
```

**Linting**
```bash
# Check syntax
jsonnet lint myfile.jsonnet

# With Tanka
tk lint environments/production
```

**IDE Support**
- VS Code: Jsonnet Language Server
- IntelliJ: Jsonnet Plugin
- Vim/Neovim: vim-jsonnet

## Additional Resources

- [Jsonnet Documentation](https://jsonnet.org/learning/tutorial.html)
- [Jsonnet Standard Library](https://jsonnet.org/ref/stdlib.html)
- [Tanka Documentation](https://tanka.dev/)
- [ksonnet-lib](https://github.com/ksonnet/ksonnet-lib)
- [jsonnet-libs by Grafana](https://github.com/grafana/jsonnet-libs)

---

**Use this skill when:**
- Writing Kubernetes manifests with Jsonnet
- Creating configuration templates
- Building reusable infrastructure libraries
- Implementing DRY principles in configs
- Managing multi-environment deployments
- Working with Tanka or similar tools
