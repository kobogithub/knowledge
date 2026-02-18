---
name: terraform-best-practices
description: Infrastructure as Code con Terraform siguiendo estándares enterprise
version: 1.0.0
author: Knowledge Framework
tags: [terraform, iac, infrastructure, devops, security]
---

# Terraform Best Practices

Guía completa de mejores prácticas para gestionar infraestructura como código con Terraform, siguiendo estándares enterprise de organización, seguridad y mantenibilidad.

## Priority Levels

- **Critical**: Impacto directo en seguridad, costos o disponibilidad
- **High**: Mejora significativa en mantenibilidad y escalabilidad
- **Medium**: Optimización y mejores prácticas recomendadas
- **Low**: Mejoras menores y conveniencia

---

## 1. Estructura de Proyecto (Critical)

### Organización con Environments & Modules

**Estructura estándar:**

```
terraform/
├── environments/
│   ├── dev/
│   │   ├── README.md           # Documentación del environment
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   ├── outputs.tf
│   │   ├── terraform.tfvars
│   │   └── backend.tf
│   ├── staging/
│   │   └── ...
│   └── prod/
│       └── ...
└── modules/
    ├── vpc/
    │   ├── README.md           # ¡OBLIGATORIO! Documenta el módulo
    │   ├── main.tf
    │   ├── variables.tf
    │   ├── outputs.tf
    │   ├── locals.tf           # Prefijos y tags generales
    │   └── versions.tf
    ├── ec2/
    │   └── ...
    └── rds/
        └── ...
```

### README.md en Modules (Critical)

**Template de README.md para cada módulo:**

```markdown
# [Nombre del Módulo] Module

## Descripción
[Descripción breve de qué hace este módulo]

## Versión
- **Module Version**: 1.2.0
- **Terraform Version**: >= 1.6.0
- **Provider Version**: aws ~> 5.0

## Uso

\`\`\`hcl
module "vpc_prod" {
  source = "../../modules/vpc"
  
  environment = "prod"
  company     = "acme"
  project     = "ecommerce"
  component   = "network"
  service     = "aws"
  
  vpc_cidr = "10.0.0.0/16"
}
\`\`\`

## Variables

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|----------|
| environment | Environment name | string | - | yes |
| vpc_cidr | VPC CIDR block | string | - | yes |

## Outputs

| Name | Description |
|------|-------------|
| vpc_id | ID of the VPC |
| subnet_ids | List of subnet IDs |

## Recursos Creados

- VPC
- Subnets (public/private)
- Internet Gateway
- NAT Gateway
- Route Tables

## Ejemplos

Ver `examples/` directory.
```

### README.md en Environments (High)

**Template para environments:**

```markdown
# [Environment] Environment

## Información
- **Environment**: production
- **Company**: acme
- **Project**: ecommerce
- **Region**: us-east-1
- **Account ID**: 123456789012

## Recursos Desplegados

- VPC con 3 AZs
- RDS PostgreSQL 15 (Multi-AZ)
- EC2 Auto Scaling Group (min: 2, max: 10)
- ALB con SSL/TLS

## Costos Estimados

~$500/mes

## Deployment

\`\`\`bash
cd environments/prod
terraform init
terraform plan
terraform apply
\`\`\`

## Contacto

- Owner: DevOps Team
- Slack: #devops-prod
```

---

## 2. Naming Convention (Critical)

### Estándar de Nomenclatura

**Formato obligatorio:**
```
{entorno}-{empresa}-{proyecto}-{componente}-{servicio}
```

**Ejemplos:**
```
prod-acme-ecommerce-vpc-aws
staging-acme-ecommerce-rds-aws
dev-acme-analytics-gcs-gcp
prod-contoso-webapp-storage-azure
```

**Implementación en `locals.tf`:**

```hcl
# modules/vpc/locals.tf

locals {
  # Naming convention
  prefix = "${var.environment}-${var.company}-${var.project}-${var.component}-${var.service}"
  
  # Common tags
  common_tags = {
    Environment       = var.environment
    Company          = var.company
    Project          = var.project
    Component        = var.component
    Service          = var.service
    ManagedBy        = "Terraform"
    TerraformVersion = "1.6.0"
    ModuleVersion    = "1.2.0"
    Owner            = var.owner
    CostCenter       = var.cost_center
    CreatedAt        = timestamp()
  }
  
  # Merge custom tags with common tags
  tags = merge(local.common_tags, var.additional_tags)
}
```

**Uso en recursos:**

```hcl
resource "aws_vpc" "main" {
  cidr_block           = var.vpc_cidr
  enable_dns_hostnames = true
  enable_dns_support   = true
  
  tags = merge(
    local.tags,
    {
      Name = "${local.prefix}-vpc"
    }
  )
}

resource "aws_subnet" "public" {
  count = length(var.public_subnet_cidrs)
  
  vpc_id            = aws_vpc.main.id
  cidr_block        = var.public_subnet_cidrs[count.index]
  availability_zone = var.availability_zones[count.index]
  
  tags = merge(
    local.tags,
    {
      Name = "${local.prefix}-public-subnet-${count.index + 1}"
      Type = "Public"
    }
  )
}
```

---

## 3. locals.tf - Configuration Hub (High)

### Estructura de locals.tf

Cada módulo debe tener un `locals.tf` para centralizar configuración:

```hcl
# modules/rds/locals.tf

locals {
  # ==========================================
  # Naming & Tagging
  # ==========================================
  prefix = "${var.environment}-${var.company}-${var.project}-${var.component}-${var.service}"
  
  common_tags = {
    Environment       = var.environment
    Company          = var.company
    Project          = var.project
    Component        = var.component
    Service          = var.service
    ManagedBy        = "Terraform"
    TerraformVersion = var.terraform_version
    ModuleVersion    = var.module_version
    Owner            = var.owner
    CostCenter       = var.cost_center
    Compliance       = var.compliance_standard  # e.g., "PCI-DSS", "HIPAA"
  }
  
  tags = merge(local.common_tags, var.additional_tags)
  
  # ==========================================
  # Computed Values
  # ==========================================
  db_identifier = "${local.prefix}-db"
  
  # Port mapping
  port_map = {
    postgres = 5432
    mysql    = 3306
    mariadb  = 3306
  }
  
  db_port = local.port_map[var.engine]
  
  # ==========================================
  # Security & Networking
  # ==========================================
  allowed_cidr_blocks = concat(
    var.vpc_cidr_blocks,
    var.additional_cidr_blocks
  )
  
  # ==========================================
  # Backup & Maintenance
  # ==========================================
  backup_window      = var.environment == "prod" ? "03:00-04:00" : "02:00-03:00"
  maintenance_window = var.environment == "prod" ? "sun:04:00-sun:05:00" : "sun:03:00-sun:04:00"
  
  # ==========================================
  # Conditional Features
  # ==========================================
  enable_multi_az           = var.environment == "prod" ? true : false
  enable_deletion_protection = var.environment == "prod" ? true : false
  backup_retention_days     = var.environment == "prod" ? 30 : 7
}
```

### Variables Requeridas (variables.tf)

```hcl
# variables.tf - Naming convention variables

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be dev, staging, or prod."
  }
}

variable "company" {
  description = "Company name"
  type        = string
}

variable "project" {
  description = "Project name"
  type        = string
}

variable "component" {
  description = "Component/service being deployed (e.g., vpc, rds, ec2)"
  type        = string
}

variable "service" {
  description = "Cloud service (aws, gcp, azure)"
  type        = string
  default     = "aws"
}

variable "owner" {
  description = "Team or person responsible"
  type        = string
}

variable "cost_center" {
  description = "Cost center for billing"
  type        = string
}

variable "terraform_version" {
  description = "Terraform version used"
  type        = string
  default     = "1.6.0"
}

variable "module_version" {
  description = "Module version"
  type        = string
}

variable "additional_tags" {
  description = "Additional tags to merge with common tags"
  type        = map(string)
  default     = {}
}
```

---

## 4. Security Scanning con Checkov (Critical)

### Instalación y Configuración

```bash
# Instalar Checkov
pip install checkov

# O con brew (macOS)
brew install checkov
```

### Pre-commit Hook

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/bridgecrewio/checkov
    rev: 3.1.0
    hooks:
      - id: checkov
        args: [
          '--framework', 'terraform',
          '--quiet',
          '--compact',
          '--output', 'cli',
          '--output', 'json',
          '--output-file-path', 'checkov-results'
        ]
```

### CI/CD Integration

```yaml
# .github/workflows/terraform-security.yml
name: Terraform Security Scan

on:
  pull_request:
    paths:
      - 'terraform/**'
      - '.github/workflows/terraform-security.yml'

jobs:
  checkov:
    name: Checkov Security Scan
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Run Checkov
        uses: bridgecrewio/checkov-action@master
        with:
          directory: terraform/
          framework: terraform
          output_format: cli,sarif
          output_file_path: console,results.sarif
          soft_fail: false  # Fail build on security issues
          skip_check: CKV_AWS_126  # Skip specific checks if needed
      
      - name: Upload SARIF results
        uses: github/codeql-action/upload-sarif@v2
        if: always()
        with:
          sarif_file: results.sarif
```

### Configuración de Checkov

```yaml
# .checkov.yml
branch: main
framework:
  - terraform
download-external-modules: true
evaluate-variables: true
external-modules-download-path: .external_modules
compact: true

# Skip specific checks
skip-check:
  - CKV_AWS_126  # Example: Skip specific check with justification
  
# Exclude paths
exclude-paths:
  - terraform/modules/legacy/
  
# Custom policies directory
external-checks-dir:
  - custom-policies/
```

### Ejemplo de Remediación

**Antes (Inseguro):**

```hcl
resource "aws_s3_bucket" "data" {
  bucket = "${local.prefix}-data"
  
  # ❌ Missing encryption
  # ❌ Missing versioning
  # ❌ Public access not blocked
}
```

**Después (Seguro):**

```hcl
resource "aws_s3_bucket" "data" {
  bucket = "${local.prefix}-data"
  
  tags = local.tags
}

# ✅ Enable versioning
resource "aws_s3_bucket_versioning" "data" {
  bucket = aws_s3_bucket.data.id
  
  versioning_configuration {
    status = "Enabled"
  }
}

# ✅ Enable encryption
resource "aws_s3_bucket_server_side_encryption_configuration" "data" {
  bucket = aws_s3_bucket.data.id
  
  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

# ✅ Block public access
resource "aws_s3_bucket_public_access_block" "data" {
  bucket = aws_s3_bucket.data.id
  
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

# ✅ Enable logging
resource "aws_s3_bucket_logging" "data" {
  bucket = aws_s3_bucket.data.id
  
  target_bucket = aws_s3_bucket.logs.id
  target_prefix = "s3-access-logs/"
}
```

---

## 5. Backend Configuration (Critical)

### Remote State con S3 + DynamoDB

```hcl
# environments/prod/backend.tf

terraform {
  backend "s3" {
    bucket         = "prod-acme-terraform-state-aws"
    key            = "ecommerce/prod/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    kms_key_id     = "arn:aws:kms:us-east-1:123456789012:key/abc123"
    dynamodb_table = "prod-acme-terraform-locks-aws"
    
    # Enhanced security
    acl = "private"
    
    # Versioning enabled on bucket (configured separately)
  }
}
```

### State Bucket Setup

```hcl
# bootstrap/state-bucket.tf

resource "aws_s3_bucket" "terraform_state" {
  bucket = "${var.environment}-${var.company}-terraform-state-${var.service}"
  
  tags = local.tags
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
      sse_algorithm     = "aws:kms"
      kms_master_key_id = aws_kms_key.terraform.arn
    }
  }
}

resource "aws_s3_bucket_public_access_block" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id
  
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_dynamodb_table" "terraform_locks" {
  name           = "${var.environment}-${var.company}-terraform-locks-${var.service}"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "LockID"
  
  attribute {
    name = "LockID"
    type = "S"
  }
  
  tags = local.tags
}
```

---

## 6. Module Versioning (High)

### versions.tf en cada módulo

```hcl
# modules/vpc/versions.tf

terraform {
  required_version = ">= 1.6.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# Module metadata
locals {
  module_version = "1.2.0"
  module_name    = "vpc"
  module_author  = "DevOps Team"
  last_updated   = "2026-02-18"
}
```

### Versionado Semántico

Sigue [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.x.x): Breaking changes
- **MINOR** (x.1.x): New features (backward compatible)
- **PATCH** (x.x.1): Bug fixes

**Ejemplo de CHANGELOG.md:**

```markdown
# Changelog - VPC Module

## [1.2.0] - 2026-02-18

### Added
- Support for VPC Flow Logs
- NAT Gateway high availability option

### Changed
- Default CIDR block validation

### Fixed
- Route table association bug

## [1.1.0] - 2026-01-15

### Added
- Multi-AZ subnet support
```

---

## 7. Code Quality (High)

### Terraform Formatting

```bash
# Format all .tf files
terraform fmt -recursive

# Check formatting (CI)
terraform fmt -check -recursive -diff
```

### Validation

```bash
# Validate configuration
terraform validate

# Enhanced validation
terraform validate -json
```

### Terraform Docs

Generar documentación automática:

```bash
# Instalar terraform-docs
brew install terraform-docs

# Generar README.md
terraform-docs markdown table --output-file README.md ./modules/vpc/
```

**Configuración `.terraform-docs.yml`:**

```yaml
formatter: markdown table
output:
  file: README.md
  mode: inject
  template: |-
    <!-- BEGIN_TF_DOCS -->
    {{ .Content }}
    <!-- END_TF_DOCS -->
    
sort:
  enabled: true
  by: required
  
settings:
  anchor: true
  color: true
  default: true
  description: false
  escape: true
  hide-empty: false
  html: true
  indent: 2
  lockfile: true
  read-comments: true
  required: true
  sensitive: true
  type: true
```

---

## 8. Testing (Medium)

### Terratest (Go)

```go
// test/vpc_test.go
package test

import (
	"testing"
	
	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestVPCModule(t *testing.T) {
	terraformOptions := &terraform.Options{
		TerraformDir: "../modules/vpc",
		
		Vars: map[string]interface{}{
			"environment": "test",
			"company":     "acme",
			"project":     "ecommerce",
			"component":   "vpc",
			"service":     "aws",
			"vpc_cidr":    "10.0.0.0/16",
		},
	}
	
	defer terraform.Destroy(t, terraformOptions)
	
	terraform.InitAndApply(t, terraformOptions)
	
	vpcID := terraform.Output(t, terraformOptions, "vpc_id")
	assert.NotEmpty(t, vpcID)
}
```

### Terraform Compliance

```bash
# Instalar
pip install terraform-compliance

# Ejecutar tests
terraform-compliance -f tests/ -p terraform/plan.json
```

**Ejemplo de test:**

```gherkin
# tests/security.feature
Feature: Security compliance
  
  Scenario: Ensure S3 buckets have encryption
    Given I have aws_s3_bucket defined
    Then it must have aws_s3_bucket_server_side_encryption_configuration
```

---

## 9. Secrets Management (Critical)

### NUNCA hardcodear secretos

❌ **MAL:**
```hcl
resource "aws_db_instance" "main" {
  username = "admin"
  password = "SuperSecret123!"  # ❌ NUNCA
}
```

✅ **BIEN: Usar AWS Secrets Manager**

```hcl
data "aws_secretsmanager_secret_version" "db_password" {
  secret_id = "${local.prefix}-db-password"
}

resource "aws_db_instance" "main" {
  username = "admin"
  password = jsondecode(data.aws_secretsmanager_secret_version.db_password.secret_string)["password"]
  
  tags = local.tags
}
```

✅ **BIEN: Usar variables de entorno**

```bash
export TF_VAR_db_password="SecretFromVault"
terraform apply
```

---

## 10. Best Practices Checklist

### Antes de cada commit

- [ ] `terraform fmt -recursive` ejecutado
- [ ] `terraform validate` pasa
- [ ] `checkov` scan completado sin errores críticos
- [ ] README.md actualizado en módulos modificados
- [ ] Versión del módulo incrementada apropiadamente
- [ ] Tags estandarizados aplicados a todos los recursos
- [ ] Naming convention seguida: `{env}-{company}-{project}-{component}-{service}`
- [ ] `locals.tf` tiene prefix y common_tags configurados
- [ ] No hay secretos hardcodeados
- [ ] Backend configurado para remote state

### Antes de merge a main

- [ ] `terraform plan` revisado
- [ ] Security scan aprobado (Checkov, tfsec)
- [ ] Code review completado
- [ ] Tests automáticos pasando (si aplica)
- [ ] CHANGELOG.md actualizado
- [ ] Documentación actualizada

### Antes de apply en producción

- [ ] Backup de state actual creado
- [ ] Plan guardado y revisado: `terraform plan -out=plan.tfplan`
- [ ] Ventana de mantenimiento programada
- [ ] Rollback plan documentado
- [ ] Stakeholders notificados
- [ ] Monitoring activo durante apply

---

## 11. Ejemplo Completo de Módulo

### Estructura del Módulo VPC

```
modules/vpc/
├── README.md              # Documentación completa
├── main.tf               # Recursos principales
├── variables.tf          # Input variables
├── outputs.tf            # Output values
├── locals.tf             # Naming, tags, computed values
├── versions.tf           # Terraform & provider versions
├── data.tf               # Data sources (optional)
└── examples/
    └── complete/
        ├── main.tf
        └── terraform.tfvars
```

### locals.tf completo

```hcl
locals {
  # Naming convention
  prefix = "${var.environment}-${var.company}-${var.project}-${var.component}-${var.service}"
  
  # Common tags
  common_tags = {
    Environment       = var.environment
    Company          = var.company
    Project          = var.project
    Component        = var.component
    Service          = var.service
    ManagedBy        = "Terraform"
    TerraformVersion = "1.6.0"
    ModuleVersion    = "1.2.0"
    Owner            = var.owner
    CostCenter       = var.cost_center
    CreatedBy        = "DevOps Team"
    Repository       = "github.com/company/terraform-modules"
  }
  
  tags = merge(local.common_tags, var.additional_tags)
  
  # Computed values
  azs = slice(data.aws_availability_zones.available.names, 0, var.az_count)
}
```

### Uso desde environment

```hcl
# environments/prod/main.tf

module "vpc" {
  source = "../../modules/vpc"
  
  # Naming convention
  environment = "prod"
  company     = "acme"
  project     = "ecommerce"
  component   = "network"
  service     = "aws"
  
  # VPC configuration
  vpc_cidr = "10.0.0.0/16"
  az_count = 3
  
  # Metadata
  owner       = "platform-team"
  cost_center = "engineering"
  
  # Additional tags
  additional_tags = {
    Compliance = "PCI-DSS"
    Backup     = "daily"
  }
}
```

---

## Recursos

- [Terraform Documentation](https://www.terraform.io/docs)
- [Checkov Documentation](https://www.checkov.io/)
- [Terraform Best Practices by Anton Babenko](https://www.terraform-best-practices.com/)
- [AWS Provider Documentation](https://registry.terraform.io/providers/hashicorp/aws/latest/docs)
- [Terraform Module Registry](https://registry.terraform.io/)

---

**Última actualización**: 2026-02-18  
**Mantenido por**: DevOps Team
