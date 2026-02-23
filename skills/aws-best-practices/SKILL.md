---
name: aws-best-practices
description: AWS cloud infrastructure best practices for secure and cost-effective deployments
version: 1.0.0
tags:
  - best-practices
  - aws
  - cloud
  - infrastructure
  - iam
---

# aws-best-practices

Mejores practicas para AWS: IAM, networking, compute, storage, seguridad, cost optimization y patrones de arquitectura.

## Overview

AWS se usa para:
- **Compute**: ECS Fargate, Lambda, EC2
- **Storage**: S3, EBS, EFS
- **Database**: RDS, Aurora, DynamoDB, ElastiCache
- **Networking**: VPC, ALB, CloudFront, Route53
- **Security**: IAM, Secrets Manager, KMS, WAF

## IAM (Identity & Access Management)

### Principio de menor privilegio

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowSpecificS3Bucket",
      "Effect": "Allow",
      "Action": [
        "s3:GetObject",
        "s3:PutObject",
        "s3:ListBucket"
      ],
      "Resource": [
        "arn:aws:s3:::my-app-bucket",
        "arn:aws:s3:::my-app-bucket/*"
      ]
    }
  ]
}
```

### Roles vs Users

```bash
# BIEN: Usar roles para aplicaciones (no access keys)
# - ECS Task Role
# - Lambda Execution Role
# - EC2 Instance Profile

# MAL: Crear IAM users con access keys para aplicaciones

# Para CI/CD: Usar OIDC con GitHub Actions
# (sin access keys estaticas)
```

### OIDC para GitHub Actions

```yaml
# .github/workflows/deploy.yml
permissions:
  id-token: write
  contents: read

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: arn:aws:iam::123456789012:role/github-actions-deploy
          aws-region: us-east-1

      - run: aws ecs update-service --cluster prod --service api --force-new-deployment
```

## Networking (VPC)

### Estructura de VPC

```
VPC (10.0.0.0/16)
├── Public Subnets (internet-facing)
│   ├── 10.0.1.0/24 (AZ-a) — ALB, NAT Gateway, Bastion
│   └── 10.0.2.0/24 (AZ-b) — ALB, NAT Gateway
├── Private Subnets (application)
│   ├── 10.0.10.0/24 (AZ-a) — ECS Tasks, EC2
│   └── 10.0.20.0/24 (AZ-b) — ECS Tasks, EC2
└── Isolated Subnets (data)
    ├── 10.0.100.0/24 (AZ-a) — RDS, ElastiCache
    └── 10.0.200.0/24 (AZ-b) — RDS, ElastiCache
```

### Security Groups

```bash
# ALB Security Group
# Inbound: 80, 443 from 0.0.0.0/0
# Outbound: app port to App SG

# App Security Group
# Inbound: app port from ALB SG only
# Outbound: 5432 to DB SG, 443 to 0.0.0.0/0 (APIs externas)

# DB Security Group
# Inbound: 5432 from App SG only
# Outbound: none (o minimal)
```

## Compute

### ECS Fargate (Containers)

```json
{
  "family": "api-task",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "512",
  "memory": "1024",
  "containerDefinitions": [
    {
      "name": "api",
      "image": "123456789012.dkr.ecr.us-east-1.amazonaws.com/api:latest",
      "portMappings": [
        { "containerPort": 8000, "protocol": "tcp" }
      ],
      "healthCheck": {
        "command": ["CMD-SHELL", "curl -f http://localhost:8000/health || exit 1"],
        "interval": 30,
        "timeout": 5,
        "retries": 3,
        "startPeriod": 60
      },
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/api",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "api"
        }
      },
      "secrets": [
        {
          "name": "DATABASE_URL",
          "valueFrom": "arn:aws:secretsmanager:us-east-1:123456789012:secret:prod/database-url"
        }
      ]
    }
  ]
}
```

### Lambda (Serverless)

```python
# handler.py
import json
import logging

logger = logging.getLogger()
logger.setLevel(logging.INFO)

def handler(event, context):
    """Lambda handler con structured logging."""
    logger.info("Processing event", extra={"event_type": event.get("type")})

    try:
        result = process(event)
        return {
            "statusCode": 200,
            "body": json.dumps(result),
            "headers": {"Content-Type": "application/json"},
        }
    except ValueError as e:
        logger.warning("Validation error: %s", e)
        return {"statusCode": 400, "body": json.dumps({"error": str(e)})}
    except Exception:
        logger.exception("Unexpected error")
        return {"statusCode": 500, "body": json.dumps({"error": "Internal server error"})}
```

## Storage (S3)

### Configuracion segura

```bash
# Bloquear acceso publico (siempre)
aws s3api put-public-access-block \
  --bucket my-bucket \
  --public-access-block-configuration \
  "BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true"

# Encriptacion por default
aws s3api put-bucket-encryption \
  --bucket my-bucket \
  --server-side-encryption-configuration \
  '{"Rules": [{"ApplyServerSideEncryptionByDefault": {"SSEAlgorithm": "aws:kms"}}]}'

# Versionamiento
aws s3api put-bucket-versioning \
  --bucket my-bucket \
  --versioning-configuration Status=Enabled

# Lifecycle rules (cost optimization)
aws s3api put-bucket-lifecycle-configuration \
  --bucket my-bucket \
  --lifecycle-configuration '{
    "Rules": [{
      "ID": "archive-old-objects",
      "Status": "Enabled",
      "Transitions": [
        {"Days": 90, "StorageClass": "STANDARD_IA"},
        {"Days": 180, "StorageClass": "GLACIER"}
      ],
      "NoncurrentVersionExpiration": {"NoncurrentDays": 30}
    }]
  }'
```

### Presigned URLs (acceso temporal)

```python
import boto3

s3 = boto3.client("s3")

# Upload presigned URL (10 min)
url = s3.generate_presigned_url(
    "put_object",
    Params={"Bucket": "my-bucket", "Key": "uploads/file.pdf"},
    ExpiresIn=600,
)

# Download presigned URL (1 hora)
url = s3.generate_presigned_url(
    "get_object",
    Params={"Bucket": "my-bucket", "Key": "reports/report.pdf"},
    ExpiresIn=3600,
)
```

## Database (RDS)

### Configuracion segura

```bash
# RDS en subnets privadas
# Multi-AZ para produccion
# Encrypted at rest (KMS)
# Automated backups (retention 7+ days)
# Performance Insights habilitado
# Enhanced Monitoring habilitado

# Parametros de seguridad
# - ssl = 1 (forzar conexiones SSL)
# - log_connections = 1
# - log_disconnections = 1
# - log_statement = 'ddl'
```

### Connection pooling

```python
# Con SQLAlchemy + asyncpg
from sqlalchemy.ext.asyncio import create_async_engine

engine = create_async_engine(
    DATABASE_URL,
    pool_size=5,            # Conexiones base
    max_overflow=10,        # Conexiones extras bajo carga
    pool_timeout=30,        # Timeout esperando conexion
    pool_recycle=1800,      # Reciclar conexiones cada 30 min
    pool_pre_ping=True,     # Verificar conexion antes de usar
)
```

## Secrets Management

### AWS Secrets Manager

```bash
# Crear secret
aws secretsmanager create-secret \
  --name prod/database-url \
  --secret-string "postgresql://user:pass@host:5432/db"

# Leer secret
aws secretsmanager get-secret-value --secret-id prod/database-url

# Rotar secret automaticamente
aws secretsmanager rotate-secret \
  --secret-id prod/database-url \
  --rotation-lambda-arn arn:aws:lambda:us-east-1:123:function:rotate-db
```

```python
# En Python
import boto3
import json

def get_secret(secret_name: str) -> dict:
    client = boto3.client("secretsmanager")
    response = client.get_secret_value(SecretId=secret_name)
    return json.loads(response["SecretString"])
```

## Monitoring

### CloudWatch Alarms

```bash
# CPU alta en ECS
aws cloudwatch put-metric-alarm \
  --alarm-name "api-high-cpu" \
  --metric-name CPUUtilization \
  --namespace AWS/ECS \
  --statistic Average \
  --period 300 \
  --threshold 80 \
  --comparison-operator GreaterThanThreshold \
  --evaluation-periods 2 \
  --alarm-actions arn:aws:sns:us-east-1:123:alerts

# 5xx errors en ALB
aws cloudwatch put-metric-alarm \
  --alarm-name "api-5xx-errors" \
  --metric-name HTTPCode_Target_5XX_Count \
  --namespace AWS/ApplicationELB \
  --statistic Sum \
  --period 60 \
  --threshold 10 \
  --comparison-operator GreaterThanThreshold \
  --evaluation-periods 1 \
  --alarm-actions arn:aws:sns:us-east-1:123:alerts
```

## Cost Optimization

### Estrategias clave

```bash
# 1. Right-sizing
# Usar Compute Optimizer para recomendaciones de instancias

# 2. Savings Plans / Reserved Instances
# Para workloads predecibles (RDS, ECS, EC2)

# 3. Spot Instances para workloads tolerantes a interrupciones
# CI/CD runners, batch processing

# 4. S3 Intelligent Tiering
# Mueve objetos automaticamente entre tiers

# 5. Apagar recursos de dev/staging fuera de horas
# Usar AWS Instance Scheduler

# 6. Tags para cost allocation
aws ec2 create-tags \
  --resources i-1234567890abcdef0 \
  --tags Key=Environment,Value=production Key=Team,Value=backend Key=Project,Value=myapp
```

## CLI AWS Util

```bash
# Configurar perfil
aws configure --profile myapp-prod

# Usar perfil
export AWS_PROFILE=myapp-prod

# Verificar identidad
aws sts get-caller-identity

# Listar recursos
aws ec2 describe-instances --filters "Name=tag:Environment,Values=production"
aws ecs list-services --cluster prod
aws rds describe-db-instances
aws s3 ls

# Logs de ECS
aws logs get-log-events \
  --log-group-name /ecs/api \
  --log-stream-name api/api/TASK-ID \
  --limit 50
```

## Mejores Practicas

### DO

- Usar roles (no access keys) para aplicaciones
- OIDC para CI/CD (GitHub Actions assume role)
- Subnets privadas para aplicaciones y databases
- Encriptar todo at rest (KMS) y in transit (TLS)
- Multi-AZ para produccion (RDS, ECS)
- Tags en todos los recursos para cost tracking
- CloudWatch alarms para metricas criticas
- Secrets Manager para credenciales (no env vars en task definitions)
- VPC endpoints para servicios AWS (evitar internet)
- Automated backups con retention policies

### DON'T

- Crear IAM users con access keys para aplicaciones
- Poner bases de datos en subnets publicas
- Usar security groups con `0.0.0.0/0` en inbound (excepto ALB 80/443)
- Hardcodear credenciales en codigo o task definitions
- Usar la cuenta root para operaciones diarias
- Dejar recursos sin tags
- Ignorar billing alerts — configurar budget alarms
- Usar `:latest` tag en task definitions de produccion
- Omitir health checks en ECS/ALB
- Dejar S3 buckets con acceso publico

## Recursos

- [AWS Well-Architected Framework](https://aws.amazon.com/architecture/well-architected/)
- [AWS Security Best Practices](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html)
- [AWS Cost Optimization](https://aws.amazon.com/aws-cost-management/)
- [ECS Best Practices](https://docs.aws.amazon.com/AmazonECS/latest/bestpracticesguide/)
- [AWS CLI Reference](https://docs.aws.amazon.com/cli/latest/)
