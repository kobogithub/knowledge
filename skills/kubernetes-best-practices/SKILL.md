---
name: kubernetes-best-practices
description: Kubernetes operations, monitoring, deployments, secrets and troubleshooting
version: 2.0.0
tags:
  - best-practices
  - kubernetes
  - k8s
  - operations
  - monitoring
  - deployments
---

# kubernetes-best-practices

Mejores practicas operativas para Kubernetes: despliegues, monitoreo, secrets, troubleshooting, scaling y gestion de clusters en produccion.

> **Nota**: Para generar manifests de Kubernetes, ver la skill [jsonnet-best-practices](../jsonnet-best-practices/SKILL.md). Usamos Jsonnet para compilar manifests y `kubectl apply -f -` en vez de escribir YAML a mano o usar Helm.

## Overview

Esta skill cubre lo **operativo** de Kubernetes:
- **Despliegues**: Rolling updates, rollbacks, canary, blue-green
- **Monitoreo**: Prometheus, alertas, metricas, dashboards
- **Secrets**: Gestion segura con External Secrets, SOPS, sealed secrets
- **Troubleshooting**: Debugging de pods, nodos, networking, storage
- **Scaling**: HPA, VPA, cluster autoscaler, capacity planning
- **Seguridad operativa**: RBAC, network policies, pod security

## Despliegues

### Flujo de deploy con Jsonnet

```bash
# 1. Compilar manifests desde Jsonnet
jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=v1.2.3 apps/api.jsonnet

# 2. Diff — ver que cambiaria antes de aplicar
jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=v1.2.3 apps/api.jsonnet \
  | jq -r '.[]' \
  | kubectl diff -f -

# 3. Aplicar
jsonnet -J vendor -J lib --ext-str env=production --ext-str tag=v1.2.3 apps/api.jsonnet \
  | jq -r '.[]' \
  | kubectl apply -f -

# 4. Verificar
kubectl rollout status deployment/api -n production --timeout=120s
```

### Rolling Updates

```bash
# Ver estado del rollout
kubectl rollout status deployment/api -n myapp

# Ver historial de revisiones
kubectl rollout history deployment/api -n myapp

# Ver detalles de una revision especifica
kubectl rollout history deployment/api -n myapp --revision=3

# Pausar rollout (para canary manual)
kubectl rollout pause deployment/api -n myapp

# Resumir rollout
kubectl rollout resume deployment/api -n myapp

# Restart pods (rolling restart sin cambiar manifest)
kubectl rollout restart deployment/api -n myapp
```

### Rollbacks

```bash
# Rollback a la revision anterior
kubectl rollout undo deployment/api -n myapp

# Rollback a una revision especifica
kubectl rollout undo deployment/api -n myapp --to-revision=3

# Ver que version esta corriendo
kubectl get deployment api -n myapp -o jsonpath='{.spec.template.spec.containers[0].image}'

# Rollback con verificacion completa
kubectl rollout undo deployment/api -n myapp && \
  kubectl rollout status deployment/api -n myapp --timeout=120s && \
  kubectl get pods -n myapp -l app=api
```

### Estrategias de deploy

```bash
# Rolling update (default) — gradual, zero downtime
# Configurado en el manifest: maxSurge: 1, maxUnavailable: 0

# Blue-green — dos deployments, switch via service
kubectl apply -f deployment-green.yaml
kubectl rollout status deployment/api-green -n myapp --timeout=120s
# Verificar que green funciona
kubectl port-forward svc/api-green 8000:80 -n myapp
# Switch: apuntar el service al green
kubectl patch service api -n myapp -p '{"spec":{"selector":{"version":"green"}}}'
# Si falla, revertir al blue
kubectl patch service api -n myapp -p '{"spec":{"selector":{"version":"blue"}}}'

# Canary — deploy parcial para validar con trafico real
# Reducir replicas del canary y monitorear metricas
kubectl scale deployment/api-canary -n myapp --replicas=1
# Verificar metricas del canary vs stable
# Si todo OK, promover el canary a stable
```

## Monitoreo

### kubectl para monitoreo rapido

```bash
# Estado de pods con metricas
kubectl top pods -n myapp --sort-by=memory
kubectl top pods -n myapp --sort-by=cpu
kubectl top nodes

# Ver pods con problemas
kubectl get pods -n myapp --field-selector=status.phase!=Running
kubectl get pods -A --field-selector=status.phase=Failed

# Pods en CrashLoopBackOff
kubectl get pods -A | grep -E "CrashLoop|Error|OOMKilled"

# Ver eventos recientes (problemas de scheduling, OOM, etc.)
kubectl get events -n myapp --sort-by=.metadata.creationTimestamp --field-selector type=Warning

# Estado de nodos
kubectl get nodes -o wide
kubectl describe node NODE-NAME | grep -A 5 "Conditions"
kubectl describe node NODE-NAME | grep -A 10 "Allocated resources"
```

### Prometheus — metricas y alertas

```yaml
# ServiceMonitor — decirle a Prometheus que scrapee tu app
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: api-monitor
  namespace: myapp
  labels:
    release: prometheus  # Debe coincidir con el label del Prometheus operator
spec:
  selector:
    matchLabels:
      app: api
  endpoints:
    - port: http
      path: /metrics
      interval: 30s
      scrapeTimeout: 10s
```

### Alertas con PrometheusRule

```yaml
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: api-alerts
  namespace: myapp
  labels:
    release: prometheus
spec:
  groups:
    - name: api.rules
      rules:
        # Alta tasa de errores
        - alert: HighErrorRate
          expr: |
            sum(rate(http_requests_total{job="api",status=~"5.."}[5m]))
            / sum(rate(http_requests_total{job="api"}[5m])) > 0.05
          for: 5m
          labels:
            severity: critical
          annotations:
            summary: "API error rate above 5%"
            description: "{{ $value | humanizePercentage }} of requests are failing"

        # Latencia alta
        - alert: HighLatency
          expr: |
            histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket{job="api"}[5m])) by (le))
            > 1.0
          for: 5m
          labels:
            severity: warning
          annotations:
            summary: "API p95 latency above 1s"

        # Pod restarts
        - alert: PodCrashLooping
          expr: |
            increase(kube_pod_container_status_restarts_total{namespace="myapp"}[1h]) > 3
          for: 5m
          labels:
            severity: critical
          annotations:
            summary: "Pod {{ $labels.pod }} is crash looping"

        # Memory alta
        - alert: HighMemoryUsage
          expr: |
            container_memory_working_set_bytes{namespace="myapp"}
            / container_spec_memory_limit_bytes{namespace="myapp"} > 0.85
          for: 10m
          labels:
            severity: warning
          annotations:
            summary: "Pod {{ $labels.pod }} using >85% memory limit"

        # Pods no ready
        - alert: PodNotReady
          expr: |
            kube_pod_status_ready{namespace="myapp", condition="true"} == 0
          for: 5m
          labels:
            severity: warning
          annotations:
            summary: "Pod {{ $labels.pod }} not ready for 5 minutes"
```

### Queries utiles de PromQL

```promql
# Request rate por segundo
sum(rate(http_requests_total{namespace="myapp"}[5m])) by (service)

# Error rate como porcentaje
sum(rate(http_requests_total{status=~"5.."}[5m])) by (service)
/ sum(rate(http_requests_total[5m])) by (service) * 100

# Latencia p50, p95, p99
histogram_quantile(0.50, sum(rate(http_request_duration_seconds_bucket[5m])) by (le, service))
histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le, service))
histogram_quantile(0.99, sum(rate(http_request_duration_seconds_bucket[5m])) by (le, service))

# CPU usage vs request
sum(rate(container_cpu_usage_seconds_total{namespace="myapp"}[5m])) by (pod)
/ sum(kube_pod_container_resource_requests{namespace="myapp", resource="cpu"}) by (pod)

# Memory usage vs limit
sum(container_memory_working_set_bytes{namespace="myapp"}) by (pod)
/ sum(container_spec_memory_limit_bytes{namespace="myapp"}) by (pod)

# Pods restarts en la ultima hora
sum(increase(kube_pod_container_status_restarts_total{namespace="myapp"}[1h])) by (pod)

# Deployments con replicas no disponibles
kube_deployment_status_replicas_unavailable{namespace="myapp"} > 0
```

### Logging

```bash
# Ver logs de un pod
kubectl logs pod/api-abc123 -n myapp -f --tail=100

# Ver logs del container anterior (si crasheo)
kubectl logs pod/api-abc123 -n myapp --previous

# Ver logs de todos los pods de un deployment
kubectl logs -l app=api -n myapp --tail=50

# Ver logs de un container especifico (pod multi-container)
kubectl logs pod/api-abc123 -n myapp -c sidecar

# Ver logs con timestamps
kubectl logs -l app=api -n myapp --timestamps --since=1h

# Buscar errores en logs
kubectl logs -l app=api -n myapp --since=1h | grep -i error

# Stern — tail de multiples pods (mejor que kubectl logs)
stern api -n myapp --since=30m
stern "api|worker" -n myapp --output json
```

## Secrets

### kubectl para secrets

```bash
# Crear secret desde literal
kubectl create secret generic api-secrets \
  --namespace myapp \
  --from-literal=database-url='postgresql://user:pass@db:5432/myapp' \
  --from-literal=api-key='sk-abc123'

# Crear secret desde archivo
kubectl create secret generic tls-cert \
  --namespace myapp \
  --from-file=tls.crt=./cert.pem \
  --from-file=tls.key=./key.pem

# Ver secrets (sin decodificar)
kubectl get secrets -n myapp
kubectl describe secret api-secrets -n myapp

# Decodificar un secret
kubectl get secret api-secrets -n myapp -o jsonpath='{.data.database-url}' | base64 -d

# Actualizar un secret
kubectl create secret generic api-secrets \
  --namespace myapp \
  --from-literal=database-url='postgresql://new-user:new-pass@db:5432/myapp' \
  --dry-run=client -o yaml | kubectl apply -f -

# Borrar secret
kubectl delete secret api-secrets -n myapp
```

### External Secrets Operator (ESO)

```yaml
# ClusterSecretStore — conexion al proveedor de secrets
apiVersion: external-secrets.io/v1beta1
kind: ClusterSecretStore
metadata:
  name: aws-secrets
spec:
  provider:
    aws:
      service: SecretsManager
      region: us-east-1
      auth:
        jwt:
          serviceAccountRef:
            name: external-secrets-sa
            namespace: external-secrets
---
# ExternalSecret — mapear secret externo a K8s secret
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: api-secrets
  namespace: myapp
spec:
  refreshInterval: 1h
  secretStoreRef:
    kind: ClusterSecretStore
    name: aws-secrets
  target:
    name: api-secrets
    creationPolicy: Owner
  data:
    - secretKey: database-url
      remoteRef:
        key: production/api/database-url
    - secretKey: api-key
      remoteRef:
        key: production/api/api-key
```

### SOPS — Secrets encriptados en Git

```bash
# Instalar SOPS
brew install sops

# Crear .sops.yaml en la raiz del repo
cat > .sops.yaml << 'EOF'
creation_rules:
  - path_regex: secrets/.*\.yaml$
    kms: arn:aws:kms:us-east-1:123456789:key/abc-123
    encrypted_regex: "^(data|stringData)$"
EOF

# Encriptar un secret
sops -e secrets/api-secrets.yaml > secrets/api-secrets.enc.yaml

# Desencriptar y aplicar
sops -d secrets/api-secrets.enc.yaml | kubectl apply -f -

# Editar secret encriptado in-place
sops secrets/api-secrets.enc.yaml
```

### Sealed Secrets (Bitnami)

```bash
# Instalar kubeseal CLI
brew install kubeseal

# Crear un SealedSecret desde un secret regular
kubectl create secret generic api-secrets \
  --namespace myapp \
  --from-literal=database-url='postgresql://...' \
  --dry-run=client -o yaml \
  | kubeseal --format yaml > sealed-api-secrets.yaml

# El SealedSecret se puede commitear a git — solo el cluster lo puede desencriptar
kubectl apply -f sealed-api-secrets.yaml
```

## Troubleshooting

### Diagnostico de pods

```bash
# Pod no arranca — ver eventos
kubectl describe pod POD-NAME -n myapp | tail -30

# Motivos comunes:
# ImagePullBackOff — imagen no existe o no hay permisos
# CrashLoopBackOff — la app crashea al arrancar
# Pending — no hay nodos con recursos suficientes
# OOMKilled — se excedio el memory limit

# Ver estado detallado
kubectl get pod POD-NAME -n myapp -o jsonpath='{.status.containerStatuses[0].state}'

# Ver reason del ultimo restart
kubectl get pod POD-NAME -n myapp -o jsonpath='{.status.containerStatuses[0].lastState.terminated.reason}'

# Ejecutar shell en pod para debugging
kubectl exec -it POD-NAME -n myapp -- /bin/sh

# Debug container (si el pod no tiene shell)
kubectl debug POD-NAME -n myapp --image=busybox:1.36 -it -- /bin/sh
kubectl debug POD-NAME -n myapp --image=nicolaka/netshoot -it -- /bin/bash

# Copiar archivos desde/hacia pod
kubectl cp myapp/POD-NAME:/app/logs/error.log ./error.log
kubectl cp ./debug-script.sh myapp/POD-NAME:/tmp/debug.sh
```

### Diagnostico de networking

```bash
# Verificar que el service resuelve
kubectl exec -it POD-NAME -n myapp -- nslookup api.myapp.svc.cluster.local

# Test de conectividad a otro servicio
kubectl exec -it POD-NAME -n myapp -- wget -qO- http://api.myapp.svc:80/health

# Ver endpoints de un service (pods reales detras)
kubectl get endpoints api -n myapp

# Si endpoints esta vacio, el selector del service no matchea ningun pod
kubectl get pods -n myapp -l app=api --show-labels

# Verificar network policies
kubectl get networkpolicies -n myapp
kubectl describe networkpolicy api-network-policy -n myapp

# DNS debugging
kubectl run dns-test --image=busybox:1.36 --rm -it --restart=Never -- nslookup api.myapp.svc.cluster.local

# Port-forward para acceder a un servicio directamente
kubectl port-forward svc/api 8000:80 -n myapp
kubectl port-forward pod/POD-NAME 8000:8000 -n myapp
```

### Diagnostico de storage

```bash
# Ver PVCs y su estado
kubectl get pvc -n myapp
kubectl describe pvc data-pvc -n myapp

# Ver PVs
kubectl get pv

# PVC Pending — no hay PV disponible o StorageClass no existe
kubectl get storageclass

# Ver uso de disco en pods
kubectl exec -it POD-NAME -n myapp -- df -h

# Expandir PVC (si el StorageClass lo soporta)
kubectl patch pvc data-pvc -n myapp -p '{"spec":{"resources":{"requests":{"storage":"20Gi"}}}}'
```

### Diagnostico de nodos

```bash
# Ver estado de nodos
kubectl get nodes -o wide
kubectl describe node NODE-NAME

# Ver taints (por que un pod no se schedula en un nodo)
kubectl get nodes -o custom-columns=NAME:.metadata.name,TAINTS:.spec.taints

# Ver capacidad vs uso
kubectl describe node NODE-NAME | grep -A 5 "Allocated resources"

# Cordon/Uncordon — evitar que se schedulen pods nuevos
kubectl cordon NODE-NAME
kubectl uncordon NODE-NAME

# Drain — sacar pods del nodo (para mantenimiento)
kubectl drain NODE-NAME --ignore-daemonsets --delete-emptydir-data

# Ver pods en un nodo especifico
kubectl get pods -A --field-selector spec.nodeName=NODE-NAME
```

### Checklist de troubleshooting

```
Pod no arranca:
  1. kubectl describe pod — ver eventos
  2. kubectl logs --previous — ver logs del crash anterior
  3. Verificar image, imagePullSecrets
  4. Verificar resources (requests vs nodo disponible)
  5. Verificar securityContext (runAsNonRoot, readOnlyRootFilesystem)

Pod arranca pero no recibe trafico:
  1. kubectl get endpoints — el service tiene endpoints?
  2. Labels del pod matchean el selector del service?
  3. readinessProbe esta pasando?
  4. Network policies bloquean el trafico?

Pod recibe trafico pero da errores:
  1. kubectl logs -f — ver errores en tiempo real
  2. kubectl exec -- env — verificar env vars y secrets
  3. kubectl exec -- wget internal-service — verificar conectividad a dependencias
  4. kubectl top pod — verificar CPU/memory (throttling?)
```

## Scaling

### HPA (Horizontal Pod Autoscaler)

```bash
# Ver HPAs
kubectl get hpa -n myapp

# Descripcion detallada (metricas actuales vs target)
kubectl describe hpa api-hpa -n myapp

# Crear HPA rapido desde CLI
kubectl autoscale deployment api -n myapp --min=2 --max=10 --cpu-percent=70

# Ver eventos del HPA (scale up/down decisions)
kubectl get events -n myapp --field-selector involvedObject.kind=HorizontalPodAutoscaler

# Scale manual (override temporal del HPA)
kubectl scale deployment/api -n myapp --replicas=5
# NOTA: el HPA lo va a sobreescribir si esta activo
```

### VPA (Vertical Pod Autoscaler)

```yaml
# VPA ajusta requests/limits automaticamente basado en uso real
apiVersion: autoscaling.k8s.io/v1
kind: VerticalPodAutoscaler
metadata:
  name: api-vpa
  namespace: myapp
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api
  updatePolicy:
    updateMode: "Off"  # "Off" = solo recomendar, "Auto" = aplicar
  resourcePolicy:
    containerPolicies:
      - containerName: api
        minAllowed:
          cpu: 50m
          memory: 64Mi
        maxAllowed:
          cpu: 2
          memory: 2Gi
```

```bash
# Ver recomendaciones del VPA
kubectl get vpa api-vpa -n myapp -o jsonpath='{.status.recommendation}'
```

### PodDisruptionBudget

```yaml
# Garantizar minimo de pods durante upgrades/mantenimiento
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: api-pdb
  namespace: myapp
spec:
  minAvailable: 2         # O usar maxUnavailable: 1
  selector:
    matchLabels:
      app: api
```

```bash
# Ver PDBs
kubectl get pdb -n myapp
kubectl describe pdb api-pdb -n myapp
```

## Seguridad Operativa

### RBAC — verificar permisos

```bash
# Que puede hacer un ServiceAccount?
kubectl auth can-i --list --as=system:serviceaccount:myapp:api-sa -n myapp

# Puede un SA acceder a secrets?
kubectl auth can-i get secrets --as=system:serviceaccount:myapp:api-sa -n myapp

# Quien puede hacer que en un namespace?
kubectl auth can-i --list -n myapp

# Ver roles y bindings
kubectl get roles,rolebindings -n myapp
kubectl get clusterroles,clusterrolebindings | grep -v system

# Describir un role para ver los permisos exactos
kubectl describe role api-role -n myapp
```

### Pod Security Standards

```bash
# Ver labels de Pod Security en namespaces
kubectl get ns --show-labels | grep pod-security

# Aplicar Pod Security Standard a un namespace
kubectl label namespace myapp \
  pod-security.kubernetes.io/enforce=restricted \
  pod-security.kubernetes.io/audit=restricted \
  pod-security.kubernetes.io/warn=restricted

# Dry-run — verificar que pods cumplen antes de aplicar
kubectl label namespace myapp \
  pod-security.kubernetes.io/enforce=restricted \
  --dry-run=server --overwrite
```

### Network Policies — verificar

```bash
# Listar network policies
kubectl get networkpolicies -n myapp

# Verificar que la policy esta activa
kubectl describe networkpolicy api-network-policy -n myapp

# Test de conectividad (desde un pod de prueba)
kubectl run nettest --image=nicolaka/netshoot --rm -it --restart=Never -n myapp -- \
  curl -s -o /dev/null -w "%{http_code}" http://api.myapp.svc:80/health

# Si no hay policies, todo el trafico esta permitido por default
# Aplicar deny-all como baseline:
kubectl apply -f - <<EOF
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: deny-all
  namespace: myapp
spec:
  podSelector: {}
  policyTypes:
    - Ingress
    - Egress
EOF
```

## Mantenimiento del Cluster

### Upgrades

```bash
# Ver version del cluster
kubectl version --short

# Ver version de los nodos
kubectl get nodes -o custom-columns=NAME:.metadata.name,VERSION:.status.nodeInfo.kubeletVersion

# Cordon + Drain antes de upgrade de nodo
kubectl cordon NODE-NAME
kubectl drain NODE-NAME --ignore-daemonsets --delete-emptydir-data --timeout=120s
# ... upgrade del nodo ...
kubectl uncordon NODE-NAME
```

### Limpieza

```bash
# Borrar pods completados/fallidos
kubectl delete pods -n myapp --field-selector=status.phase==Succeeded
kubectl delete pods -n myapp --field-selector=status.phase==Failed

# Borrar jobs completados
kubectl delete jobs -n myapp --field-selector=status.successful=1

# Borrar eventos viejos
kubectl delete events -n myapp --all

# Ver imagenes en uso (para saber cuales purgar del registry)
kubectl get pods -A -o jsonpath='{range .items[*]}{.spec.containers[*].image}{"\n"}{end}' | sort -u

# Resources sin uso
kubectl get configmaps -n myapp
kubectl get secrets -n myapp
kubectl get pvc -n myapp
```

### Backups

```bash
# Backup de todos los recursos de un namespace
kubectl get all -n myapp -o yaml > backup-myapp-$(date +%Y%m%d).yaml

# Backup solo de ciertos tipos
kubectl get deployments,services,configmaps,secrets -n myapp -o yaml > backup.yaml

# Backup del etcd (para clusters self-managed)
ETCDCTL_API=3 etcdctl snapshot save backup.db \
  --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key
```

## Comandos Utiles

### kubectl shortcuts

```bash
# Aliases recomendados en .bashrc/.zshrc
alias k='kubectl'
alias kn='kubectl -n'
alias kg='kubectl get'
alias kd='kubectl describe'
alias kl='kubectl logs'
alias ke='kubectl exec -it'
alias kgp='kubectl get pods'
alias kgs='kubectl get svc'
alias kgn='kubectl get nodes'

# Cambiar namespace default
kubectl config set-context --current --namespace=myapp

# Cambiar cluster
kubectl config use-context production-cluster

# Ver contextos disponibles
kubectl config get-contexts

# Port-forward
kubectl port-forward svc/api 8000:80 -n myapp

# Kustomize (cuando no usas Jsonnet para algo simple)
kubectl apply -k overlays/production/
kubectl diff -k overlays/production/
```

### kubectl con JSON path

```bash
# Obtener IPs de todos los pods
kubectl get pods -n myapp -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.status.podIP}{"\n"}{end}'

# Ver todas las imagenes en un namespace
kubectl get pods -n myapp -o jsonpath='{range .items[*]}{.spec.containers[*].image}{"\n"}{end}'

# Obtener nodo donde corre un pod
kubectl get pod POD-NAME -n myapp -o jsonpath='{.spec.nodeName}'

# Ver resource requests totales en un namespace
kubectl get pods -n myapp -o json | jq '[.items[].spec.containers[].resources.requests.cpu // "0"] | map(gsub("m";"") | tonumber) | add'
```

## Mejores Practicas

### DO

- Usar Jsonnet para generar manifests y `kubectl apply -f -` para aplicarlos
- Hacer `kubectl diff` antes de cada apply en produccion
- Configurar alertas para error rate, latencia, OOM, crash loops
- Usar External Secrets Operator o SOPS — nunca secrets en YAML plano en git
- Definir PodDisruptionBudgets para servicios criticos
- Usar namespaces para separar ambientes y equipos
- Monitorear con Prometheus + alertas en Slack/PagerDuty
- Hacer drain de nodos antes de mantenimiento
- Configurar Pod Security Standards en namespaces
- Revisar RBAC periodicamente — least privilege
- Usar `stern` para ver logs de multiples pods
- Tener runbooks para alertas comunes

### DON'T

- Escribir YAML a mano para manifests complejos — usar Jsonnet
- Usar `kubectl edit` en produccion — cambios no rastreables
- Ignorar alertas de OOMKilled o CrashLoopBackOff
- Guardar secrets en ConfigMaps o YAML plano
- Omitir resource requests — scheduling impredecible
- Deploy manual con `kubectl apply` directo — usar CI/CD pipeline
- Usar `:latest` tag — no es reproducible ni auditable
- Ignorar Network Policies — por default todo el trafico esta permitido
- Crear todo en el namespace `default`
- Hacer `kubectl delete pod` como solucion a un crash loop — arreglar la causa

## Recursos

- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [kubectl Cheat Sheet](https://kubernetes.io/docs/reference/kubectl/cheatsheet/)
- [Prometheus Operator](https://github.com/prometheus-operator/prometheus-operator)
- [External Secrets Operator](https://external-secrets.io/)
- [SOPS](https://github.com/getsops/sops)
- [Sealed Secrets](https://github.com/bitnami-labs/sealed-secrets)
- [stern](https://github.com/stern/stern) — Multi-pod log tailing
- [k9s](https://k9scli.io/) — Terminal UI para Kubernetes
- [Jsonnet Best Practices](../jsonnet-best-practices/SKILL.md) — Generar manifests con Jsonnet
