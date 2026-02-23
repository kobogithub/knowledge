---
name: terraform-best-practices
description: Terraform IaC best practices, modules, state management and CI/CD patterns
version: 1.0.0
tags:
  - best-practices
  - terraform
  - infrastructure
  - iac
  - devops
  - aws
---

# terraform-best-practices

Mejores practicas para Terraform: estructura de proyecto, modulos reutilizables, state management, seguridad y CI/CD para infraestructura como codigo.

## Overview

Terraform se usa para:
- **Infrastructure as Code**: Definir infra reproducible y versionada
- **Multi-cloud**: AWS, GCP, Azure, Cloudflare con la misma herramienta
- **State management**: Tracking de recursos creados y sus relaciones
- **Modulos reutilizables**: Componentes de infra compartidos entre proyectos
- **CI/CD**: Plan y apply automatizados con review

## Estructura de Proyecto

### Proyecto simple

```
terraform/
├── main.tf              # Recursos principales
├── variables.tf         # Declaracion de variables
├── outputs.tf           # Valores de salida
├── providers.tf         # Configuracion de providers
├── versions.tf          # Version constraints
├── terraform.tfvars     # Valores de variables (NO commitear secrets)
├── locals.tf            # Valores locales calculados
└── data.tf              # Data sources
```

### Proyecto con environments

```
terraform/
├── modules/
│   ├── networking/
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   └── outputs.tf
│   ├── database/
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   └── outputs.tf
│   └── app/
│       ├── main.tf
│       ├── variables.tf
│       └── outputs.tf
├── environments/
│   ├── dev/
│   │   ├── main.tf
│   │   ├── backend.tf
│   │   └── terraform.tfvars
│   ├── staging/
│   │   ├── main.tf
│   │   ├── backend.tf
│   │   └── terraform.tfvars
│   └── production/
│       ├── main.tf
│       ├── backend.tf
│       └── terraform.tfvars
└── README.md
```

## Configuracion Base

### versions.tf

```hcl
terraform {
  required_version = ">= 1.7"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.6"
    }
  }
}
```

### providers.tf

```hcl
provider "aws" {
  region = var.aws_region

  default_tags {
    tags = {
      Environment      = var.environment
      Customer         = var.customer
      Project          = var.project
      Component        = var.component
      Service          = var.service
      Version          = var.version_app
      VersionTerraform = "1.7"
      ManagedBy        = "terraform"
    }
  }
}

# Provider para otro region (ej: CloudFront certificates)
provider "aws" {
  alias  = "us_east_1"
  region = "us-east-1"

  default_tags {
    tags = {
      Environment      = var.environment
      Customer         = var.customer
      Project          = var.project
      Component        = var.component
      Service          = var.service
      Version          = var.version_app
      VersionTerraform = "1.7"
      ManagedBy        = "terraform"
    }
  }
}
```

### backend.tf (Remote State)

```hcl
# SIEMPRE usar remote state — nunca local en equipo
terraform {
  backend "s3" {
    bucket         = "myproject-terraform-state"
    key            = "environments/production/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "terraform-state-lock"

    # Prevenir borrado accidental
    # lifecycle { prevent_destroy = true }
  }
}
```

### Crear el backend de S3

```hcl
# bootstrap/main.tf — ejecutar una sola vez para crear el state bucket
resource "aws_s3_bucket" "terraform_state" {
  bucket = "myproject-terraform-state"

  lifecycle {
    prevent_destroy = true
  }
}

resource "aws_s3_bucket_versioning" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id
  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "aws:kms"
    }
  }
}

resource "aws_s3_bucket_public_access_block" "terraform_state" {
  bucket                  = aws_s3_bucket.terraform_state.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_dynamodb_table" "terraform_lock" {
  name         = "terraform-state-lock"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "LockID"

  attribute {
    name = "LockID"
    type = "S"
  }
}
```

## Variables

### Declaracion con validacion

```hcl
# variables.tf
# Naming convention: {environment}-{customer}-{project}-{component}-{service}

variable "environment" {
  description = "Deployment environment"
  type        = string

  validation {
    condition     = contains(["dev", "staging", "production"], var.environment)
    error_message = "Environment must be dev, staging, or production."
  }
}

variable "customer" {
  description = "Customer or client name"
  type        = string

  validation {
    condition     = length(var.customer) > 0 && can(regex("^[a-z0-9-]+$", var.customer))
    error_message = "Customer must be lowercase alphanumeric with hyphens only."
  }
}

variable "project" {
  description = "Project name"
  type        = string

  validation {
    condition     = length(var.project) > 0 && can(regex("^[a-z0-9-]+$", var.project))
    error_message = "Project must be lowercase alphanumeric with hyphens only."
  }
}

variable "component" {
  description = "Component name (e.g., api, frontend, worker, database)"
  type        = string

  validation {
    condition     = length(var.component) > 0
    error_message = "Component must not be empty."
  }
}

variable "service" {
  description = "Service name within the component"
  type        = string

  validation {
    condition     = length(var.service) > 0
    error_message = "Service must not be empty."
  }
}

variable "version_app" {
  description = "Application version (semver)"
  type        = string
  default     = "0.0.0"
}

variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "db_instance_class" {
  description = "RDS instance class"
  type        = string
  default     = "db.t3.micro"
}

variable "enable_monitoring" {
  description = "Enable CloudWatch monitoring"
  type        = bool
  default     = true
}

variable "allowed_cidr_blocks" {
  description = "CIDR blocks allowed to access the service"
  type        = list(string)
  default     = []
}

# NUNCA poner defaults para secrets — forzar que se pasen
variable "db_password" {
  description = "Database master password"
  type        = string
  sensitive   = true
}
```

### Locals

```hcl
# locals.tf
locals {
  # Naming convention: {environment}-{customer}-{project}-{component}-{service}
  name_prefix = "${var.environment}-${var.customer}-${var.project}-${var.component}-${var.service}"

  # Tags comunes — se mergean con el tag Name de cada recurso y tags custom por recurso
  common_tags = {
    Environment      = var.environment
    Customer         = var.customer
    Project          = var.project
    Component        = var.component
    Service          = var.service
    Version          = var.version_app
    VersionTerraform = "1.7"
    ManagedBy        = "terraform"
  }

  # Valores calculados
  is_production = var.environment == "production"
  db_multi_az   = local.is_production
  min_instances = local.is_production ? 2 : 1
  max_instances = local.is_production ? 10 : 3
}

# Uso en recursos — mergear common_tags + Name + tags custom del recurso
resource "aws_vpc" "main" {
  cidr_block = var.vpc_cidr

  tags = merge(local.common_tags, {
    Name = "${local.name_prefix}-vpc"
  })
}

resource "aws_instance" "app" {
  ami           = data.aws_ami.amazon_linux.id
  instance_type = "t3.micro"

  tags = merge(local.common_tags, {
    Name = "${local.name_prefix}-app"
    Role = "application"  # tag custom por recurso
  })
}
```

### terraform.tfvars

```hcl
# environments/production/terraform.tfvars
environment = "production"
customer    = "acme"
project     = "webapp"
component   = "api"
service     = "main"
version_app = "1.2.0"
aws_region  = "us-east-1"

db_instance_class   = "db.r6g.large"
enable_monitoring   = true
allowed_cidr_blocks = ["10.0.0.0/8"]
```

## Modulos

### Estructura de un modulo

```hcl
# modules/networking/main.tf
resource "aws_vpc" "main" {
  cidr_block           = var.vpc_cidr
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name = "${var.name_prefix}-vpc"
  }
}

resource "aws_subnet" "public" {
  count = length(var.availability_zones)

  vpc_id            = aws_vpc.main.id
  cidr_block        = cidrsubnet(var.vpc_cidr, 8, count.index)
  availability_zone = var.availability_zones[count.index]

  map_public_ip_on_launch = true

  tags = {
    Name = "${var.name_prefix}-public-${var.availability_zones[count.index]}"
    Tier = "public"
  }
}

resource "aws_subnet" "private" {
  count = length(var.availability_zones)

  vpc_id            = aws_vpc.main.id
  cidr_block        = cidrsubnet(var.vpc_cidr, 8, count.index + length(var.availability_zones))
  availability_zone = var.availability_zones[count.index]

  tags = {
    Name = "${var.name_prefix}-private-${var.availability_zones[count.index]}"
    Tier = "private"
  }
}
```

```hcl
# modules/networking/variables.tf
variable "name_prefix" {
  description = "Prefix for resource names"
  type        = string
}

variable "vpc_cidr" {
  description = "CIDR block for the VPC"
  type        = string
  default     = "10.0.0.0/16"
}

variable "availability_zones" {
  description = "List of availability zones"
  type        = list(string)
}
```

```hcl
# modules/networking/outputs.tf
output "vpc_id" {
  description = "ID of the VPC"
  value       = aws_vpc.main.id
}

output "public_subnet_ids" {
  description = "IDs of public subnets"
  value       = aws_subnet.public[*].id
}

output "private_subnet_ids" {
  description = "IDs of private subnets"
  value       = aws_subnet.private[*].id
}
```

### Usar el modulo

```hcl
# environments/production/main.tf
module "networking" {
  source = "../../modules/networking"

  name_prefix        = local.name_prefix
  vpc_cidr           = "10.0.0.0/16"
  availability_zones = ["us-east-1a", "us-east-1b", "us-east-1c"]
}

module "database" {
  source = "../../modules/database"

  name_prefix      = local.name_prefix
  vpc_id           = module.networking.vpc_id
  subnet_ids       = module.networking.private_subnet_ids
  instance_class   = var.db_instance_class
  master_password  = var.db_password
  multi_az         = local.is_production
}

module "app" {
  source = "../../modules/app"

  name_prefix    = local.name_prefix
  vpc_id         = module.networking.vpc_id
  subnet_ids     = module.networking.public_subnet_ids
  db_endpoint    = module.database.endpoint
  min_instances  = local.min_instances
  max_instances  = local.max_instances
}
```

## Data Sources

```hcl
# Obtener datos existentes sin crearlos
data "aws_caller_identity" "current" {}

data "aws_region" "current" {}

data "aws_availability_zones" "available" {
  state = "available"
}

# AMI mas reciente
data "aws_ami" "amazon_linux" {
  most_recent = true
  owners      = ["amazon"]

  filter {
    name   = "name"
    values = ["al2023-ami-*-x86_64"]
  }
}

# SSM parameter (secrets)
data "aws_ssm_parameter" "db_password" {
  name = "/${var.environment}/${var.customer}/${var.project}/db/password"
}

# Usar en recursos
resource "aws_instance" "app" {
  ami           = data.aws_ami.amazon_linux.id
  instance_type = "t3.micro"
  # ...
}
```

## Outputs

```hcl
# outputs.tf
output "vpc_id" {
  description = "ID of the VPC"
  value       = module.networking.vpc_id
}

output "db_endpoint" {
  description = "Database endpoint"
  value       = module.database.endpoint
}

output "app_url" {
  description = "Application URL"
  value       = "https://${module.app.domain_name}"
}

# Marcar outputs sensibles
output "db_password" {
  description = "Database password"
  value       = var.db_password
  sensitive   = true
}
```

## Lifecycle y State

### Lifecycle rules

```hcl
resource "aws_instance" "app" {
  ami           = data.aws_ami.amazon_linux.id
  instance_type = "t3.micro"

  lifecycle {
    # Prevenir borrado accidental
    prevent_destroy = true

    # Crear nuevo antes de destruir viejo (zero downtime)
    create_before_destroy = true

    # Ignorar cambios que se hacen fuera de terraform
    ignore_changes = [
      tags["LastModified"],
      user_data,
    ]
  }
}
```

### State management

```bash
# Listar recursos en state
terraform state list

# Ver detalles de un recurso
terraform state show aws_instance.app

# Mover recurso (renombrar sin destruir)
terraform state mv aws_instance.app aws_instance.web

# Importar recurso existente
terraform import aws_instance.app i-1234567890abcdef0

# Remover del state (sin destruir)
terraform state rm aws_instance.legacy

# Refresh state
terraform refresh
```

## Patrones Comunes

### for_each vs count

```hcl
# Preferir for_each sobre count — las keys son estables
# Si usas count y cambias el orden, terraform destruye y recrea

# Bien — for_each con map
variable "services" {
  type = map(object({
    port     = number
    protocol = string
  }))
  default = {
    http  = { port = 80, protocol = "HTTP" }
    https = { port = 443, protocol = "HTTPS" }
  }
}

resource "aws_security_group_rule" "ingress" {
  for_each = var.services

  type              = "ingress"
  security_group_id = aws_security_group.main.id
  from_port         = each.value.port
  to_port           = each.value.port
  protocol          = "tcp"
  cidr_blocks       = ["0.0.0.0/0"]

  description = "Allow ${each.key} traffic"
}

# Bien — for_each con set
resource "aws_iam_user" "developers" {
  for_each = toset(var.developer_names)

  name = each.value
}

# Evitar count para recursos con identidad
# count es OK para copias identicas
resource "aws_subnet" "public" {
  count = 3
  # ...
}
```

### Dynamic blocks

```hcl
resource "aws_security_group" "main" {
  name_prefix = "${local.name_prefix}-sg"
  vpc_id      = var.vpc_id

  dynamic "ingress" {
    for_each = var.ingress_rules
    content {
      from_port   = ingress.value.port
      to_port     = ingress.value.port
      protocol    = ingress.value.protocol
      cidr_blocks = ingress.value.cidr_blocks
      description = ingress.value.description
    }
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
```

### Conditional resources

```hcl
# Crear recurso solo si condicion es true
resource "aws_cloudwatch_metric_alarm" "cpu" {
  count = var.enable_monitoring ? 1 : 0

  alarm_name  = "${local.name_prefix}-cpu-high"
  # ...
}

# Referencia condicional
output "alarm_arn" {
  value = var.enable_monitoring ? aws_cloudwatch_metric_alarm.cpu[0].arn : null
}
```

## Comandos

```bash
# Init — descargar providers y configurar backend
terraform init

# Re-init con upgrade de providers
terraform init -upgrade

# Plan — ver que cambiaria
terraform plan
terraform plan -out=plan.tfplan     # Guardar plan

# Apply — aplicar cambios
terraform apply
terraform apply plan.tfplan         # Aplicar plan guardado
terraform apply -auto-approve       # Sin confirmacion (SOLO en CI)

# Destroy — borrar todo
terraform destroy

# Format
terraform fmt -recursive

# Validate
terraform validate

# Graph (Graphviz)
terraform graph | dot -Tpng -o graph.png

# Output
terraform output
terraform output -json
```

## CI/CD con GitHub Actions

```yaml
name: Terraform
on:
  push:
    branches: [main]
    paths: ["terraform/**"]
  pull_request:
    paths: ["terraform/**"]

permissions:
  id-token: write     # Para OIDC
  contents: read
  pull-requests: write # Para comentar el plan

jobs:
  plan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: hashicorp/setup-terraform@v3
        with:
          terraform_version: "1.7"

      - uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: ${{ vars.AWS_ROLE_ARN }}
          aws-region: us-east-1

      - name: Terraform Init
        run: terraform init
        working-directory: terraform/environments/production

      - name: Terraform Format
        run: terraform fmt -check -recursive
        working-directory: terraform

      - name: Terraform Validate
        run: terraform validate
        working-directory: terraform/environments/production

      - name: Terraform Plan
        id: plan
        run: terraform plan -no-color -out=plan.tfplan
        working-directory: terraform/environments/production

      - name: Comment PR with Plan
        if: github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            const plan = `${{ steps.plan.outputs.stdout }}`;
            const body = `## Terraform Plan\n\`\`\`hcl\n${plan.substring(0, 60000)}\n\`\`\``;
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: body
            });

  apply:
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    needs: plan
    runs-on: ubuntu-latest
    environment: production  # Requiere approval
    steps:
      - uses: actions/checkout@v4
      - uses: hashicorp/setup-terraform@v3
      - uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: ${{ vars.AWS_ROLE_ARN }}
          aws-region: us-east-1

      - name: Terraform Init & Apply
        run: |
          terraform init
          terraform apply -auto-approve
        working-directory: terraform/environments/production
```

## Seguridad

### Secrets

```hcl
# NUNCA hardcodear secrets
# Mal
variable "db_password" {
  default = "mypassword123"  # NUNCA
}

# Bien — marcar como sensitive
variable "db_password" {
  type      = string
  sensitive = true
  # Sin default — se pasa via TF_VAR_db_password o -var
}

# Bien — leer de SSM/Secrets Manager
data "aws_ssm_parameter" "db_password" {
  name            = "/${var.environment}/${var.customer}/${var.project}/db/password"
  with_decryption = true
}

resource "aws_db_instance" "main" {
  password = data.aws_ssm_parameter.db_password.value
  # ...
}
```

### Permisos minimos

```hcl
# IAM policies con least privilege
data "aws_iam_policy_document" "app" {
  statement {
    effect = "Allow"
    actions = [
      "s3:GetObject",
      "s3:PutObject",
    ]
    resources = [
      "${aws_s3_bucket.uploads.arn}/*",
    ]
  }

  statement {
    effect = "Allow"
    actions = [
      "sqs:SendMessage",
    ]
    resources = [
      aws_sqs_queue.processing.arn,
    ]
  }
}
```

### Checkov — Escaneo de Seguridad

[Checkov](https://github.com/bridgecrewio/checkov) analiza codigo Terraform para detectar configuraciones inseguras, compliance violations, y misconfigurations antes de aplicar.

#### Instalacion

```bash
# pip
pip install checkov

# pipx (recomendado — aislado)
pipx install checkov

# brew
brew install checkov

# Docker
docker run --rm -v "$(pwd):/tf" bridgecrew/checkov -d /tf
```

#### Uso basico

```bash
# Escanear directorio completo
checkov -d terraform/

# Escanear archivos especificos
checkov -f terraform/main.tf

# Escanear un plan (mas preciso — evalua valores reales)
terraform plan -out=plan.tfplan
terraform show -json plan.tfplan > plan.json
checkov -f plan.json --framework terraform_plan

# Output en JSON
checkov -d terraform/ -o json > checkov-report.json

# Output en JUnit (para CI)
checkov -d terraform/ -o junitxml > checkov-report.xml

# Solo errores (sin passed checks)
checkov -d terraform/ --compact
```

#### Checks comunes para AWS

```bash
# Checks criticos que checkov valida:
# CKV_AWS_18  - S3 bucket sin logging habilitado
# CKV_AWS_19  - S3 bucket sin server-side encryption
# CKV_AWS_21  - S3 bucket sin versionado
# CKV_AWS_53  - S3 bucket con block public access deshabilitado
# CKV_AWS_145 - S3 bucket sin KMS encryption
# CKV_AWS_23  - Security group con descripcion vacia
# CKV_AWS_24  - Security group permite ingress 0.0.0.0/0 a port 22
# CKV_AWS_25  - Security group permite ingress 0.0.0.0/0 a port 3389
# CKV_AWS_8   - RDS instance sin encryption habilitada
# CKV_AWS_16  - RDS instance sin encryption at rest
# CKV_AWS_17  - RDS instance sin logging habilitado
# CKV_AWS_118 - RDS enhanced monitoring deshabilitado
# CKV_AWS_126 - RDS instance no es multi-az
# CKV_AWS_157 - RDS instance sin auto minor version upgrade
# CKV_AWS_161 - RDS instance con IAM auth deshabilitado
# CKV_AWS_226 - EC2 instance sin detailed monitoring
# CKV_AWS_79  - EC2 instance metadata v2 no requerido
# CKV_AWS_88  - EC2 instance con IP publica
# CKV_AWS_260 - Security group permite ingress 0.0.0.0/0
# CKV2_AWS_5  - Security group no asociado a ningun recurso
```

#### Suprimir checks (cuando es intencional)

```hcl
# En el recurso — inline skip con justificacion
resource "aws_s3_bucket" "public_assets" {
  bucket = "${local.name_prefix}-public-assets"

  #checkov:skip=CKV_AWS_53:Bucket es intencionalmente publico para assets estaticos
  #checkov:skip=CKV_AWS_19:Encryption no necesaria para assets publicos
}
```

```yaml
# .checkov.yaml — configuracion del proyecto
soft-fail: false
compact: true
framework:
  - terraform
skip-check:
  - CKV_AWS_53   # Si tu caso requiere buckets publicos
check:
  # O listar solo los checks que quieres
  - CKV_AWS_18
  - CKV_AWS_19
  - CKV_AWS_21
```

#### Checkov en CI/CD (GitHub Actions)

```yaml
# Agregar step en el job de plan
- name: Checkov Security Scan
  uses: bridgecrewio/checkov-action@v12
  with:
    directory: terraform/
    framework: terraform
    output_format: cli,sarif
    output_file_path: console,checkov-results.sarif
    soft_fail: false   # true para no bloquear el pipeline al inicio
    quiet: true        # Solo mostrar failures
    skip_check: ""     # CKV_AWS_53,CKV_AWS_19 si necesitas skipear

# Subir resultados como SARIF (aparece en Security tab)
- name: Upload SARIF
  if: always()
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: checkov-results.sarif
```

#### Pre-commit hook

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/bridgecrewio/checkov
    rev: "3.2.0"
    hooks:
      - id: checkov
        args: ["--compact", "--quiet"]
```

## MCP aws-terraform

El MCP (Model Context Protocol) server `aws-terraform` permite a los agentes de IA interactuar con Terraform y AWS directamente. Util para:
- Consultar estado de infraestructura
- Generar configuraciones Terraform
- Validar y planificar cambios
- Consultar precios y documentacion de AWS

### Configuracion del MCP

```json
// .opencode/config.json o settings del editor
{
  "mcpServers": {
    "aws-terraform": {
      "command": "uvx",
      "args": ["awslabs.aws-terraform-mcp-server@latest"],
      "env": {
        "AWS_PROFILE": "default",
        "AWS_REGION": "us-east-1",
        "FASTMCP_LOG_LEVEL": "ERROR"
      }
    }
  }
}
```

### Herramientas disponibles

El MCP aws-terraform expone las siguientes tools:

```
# Terraform operations
terraform_init         — Inicializar directorio terraform
terraform_plan         — Ejecutar terraform plan
terraform_apply        — Aplicar cambios (con approval)
terraform_validate     — Validar configuracion
terraform_show         — Ver estado actual
terraform_output       — Listar outputs

# AWS operations
aws_sts_get_caller     — Verificar identidad AWS actual
aws_s3_list_buckets    — Listar buckets S3
aws_ec2_describe       — Describir instancias EC2
aws_rds_describe       — Describir instancias RDS

# Documentation
aws_pricing            — Consultar precios de servicios
aws_docs               — Buscar documentacion de AWS
```

### Flujo de trabajo con el MCP

```
# 1. Verificar identidad AWS
> Usa aws_sts_get_caller para verificar que cuenta y role estamos usando

# 2. Consultar estado actual
> Usa terraform_show para ver los recursos actuales
> Usa terraform_output para ver outputs existentes

# 3. Planificar cambios
> Despues de editar .tf files, usa terraform_validate y terraform_plan

# 4. Aplicar (con review)
> Revisa el plan y usa terraform_apply si todo se ve bien

# 5. Verificar
> Usa terraform_output para confirmar los nuevos valores
```

### Ejemplo: Pedir al agente que cree infra

```
Usuario: "Crea un bucket S3 con versionado y encryption para el proyecto"

El agente:
1. Usa aws_sts_get_caller para verificar identidad
2. Genera el codigo HCL con la naming convention correcta
3. Usa terraform_validate para verificar sintaxis
4. Usa terraform_plan para mostrar que va a crear
5. Pide confirmacion antes de terraform_apply
```

## Mejores Practicas

### DO

- Usar remote state con locking (S3 + DynamoDB)
- Usar modulos para componentes reutilizables
- Validar variables con `validation` blocks
- Usar `for_each` sobre `count` para recursos con identidad
- Marcar variables como `sensitive = true` para secrets
- Usar `default_tags` en providers para tagging consistente
- Mergear `local.common_tags` con tag `Name` en cada recurso
- Usar `lifecycle.prevent_destroy` para recursos criticos
- Hacer `terraform plan` antes de cada `apply`
- Correr Checkov en CI antes de apply para detectar misconfigurations
- Usar OIDC en CI/CD, no access keys
- Pinear versiones de providers (`~> 5.0`, no `>= 5.0`)
- Formatear con `terraform fmt -recursive`
- Separar state por environment (un backend por env)
- Seguir naming convention: `{environment}-{customer}-{project}-{component}-{service}`

### DON'T

- Guardar state localmente — usar remote backend
- Hardcodear secrets en `.tf` o `.tfvars` — usar SSM/Secrets Manager
- Usar `terraform apply -auto-approve` manualmente — solo en CI
- Editar recursos creados por Terraform manualmente — causa drift
- Usar `count` para recursos que pueden cambiar de orden
- Omitir `description` en variables y outputs
- Compartir state entre environments — un state por env
- Ignorar `terraform plan` output — revisar antes de apply
- Usar `*` en IAM policies — least privilege siempre
- Commitear `.terraform/` o `.tfstate` — agregar a `.gitignore`
- Hacer `terraform destroy` en produccion sin review
- Usar `depends_on` cuando las dependencias son implicitas

## .gitignore

```gitignore
# Terraform
.terraform/
*.tfstate
*.tfstate.*
*.tfplan
*.tfvars          # Si contiene secrets
!*.tfvars.example
.terraform.lock.hcl  # Commitear SI para reproducibilidad
crash.log
override.tf
override.tf.json
```

## Recursos

- [Terraform Docs](https://developer.hashicorp.com/terraform/docs)
- [Terraform Registry](https://registry.terraform.io/)
- [AWS Provider Docs](https://registry.terraform.io/providers/hashicorp/aws/latest/docs)
- [Terraform Best Practices](https://www.terraform-best-practices.com/)
- [Terraform Style Guide](https://developer.hashicorp.com/terraform/language/style)
- [tflint](https://github.com/terraform-linters/tflint) — Linter para Terraform
- [checkov](https://github.com/bridgecrewio/checkov) — Security scanning para IaC
- [Checkov Checks Reference](https://www.checkov.io/5.Policy%20Index/terraform.html) — Lista completa de checks
- [infracost](https://github.com/infracost/infracost) — Estimacion de costos
- [MCP aws-terraform](https://github.com/awslabs/aws-terraform-mcp-server) — MCP server para Terraform + AWS
