---
name: aws-best-practices
description: AWS cloud services, arquitectura y seguridad siguiendo Well-Architected Framework
version: 1.0.0
author: Knowledge Framework
tags: [aws, cloud, infrastructure, security, architecture]
---

# AWS Best Practices

Guía completa de mejores prácticas para Amazon Web Services basada en el AWS Well-Architected Framework, enfocada en seguridad, eficiencia de costos, confiabilidad y excelencia operacional.

## Priority Levels

- **Critical**: Seguridad, disponibilidad, compliance
- **High**: Costos, performance, disaster recovery
- **Medium**: Optimización y automatización
- **Low**: Mejoras menores y conveniencia

---

## 1. AWS Well-Architected Framework (Critical)

### Los 6 Pilares

1. **Operational Excellence** - Ejecutar y monitorear sistemas
2. **Security** - Proteger información y sistemas
3. **Reliability** - Recuperarse de fallas, escalar dinámicamente
4. **Performance Efficiency** - Usar recursos eficientemente
5. **Cost Optimization** - Reducir costos innecesarios
6. **Sustainability** - Minimizar impacto ambiental

---

## 2. Security Best Practices (Critical)

### IAM - Identity and Access Management

**Principio de Mínimo Privilegio:**

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "s3:GetObject",
        "s3:PutObject"
      ],
      "Resource": "arn:aws:s3:::prod-acme-ecommerce-data-aws/*",
      "Condition": {
        "StringEquals": {
          "s3:x-amz-server-side-encryption": "AES256"
        }
      }
    }
  ]
}
```

**Best Practices IAM:**

✅ **DO:**
- Habilitar MFA para todos los usuarios (especialmente root)
- Usar roles de IAM en lugar de access keys donde sea posible
- Rotar access keys regularmente (cada 90 días)
- Usar IAM policies con conditions
- Implementar SCPs (Service Control Policies) en AWS Organizations
- Auditar permisos con IAM Access Analyzer

❌ **DON'T:**
- Nunca usar root account para operaciones diarias
- No hardcodear access keys en código
- No dar permisos `*` (wildcard) sin necesidad
- No compartir access keys entre usuarios/servicios

### Secrets Management

**Usar AWS Secrets Manager o Parameter Store:**

```python
# Python - Boto3
import boto3

# ✅ BIEN: Secrets Manager
client = boto3.client('secretsmanager')
response = client.get_secret_value(SecretId='prod/db/password')
db_password = response['SecretString']

# ✅ BIEN: Parameter Store (para configs no-sensibles)
ssm = boto3.client('ssm')
response = ssm.get_parameter(Name='/prod/app/config', WithDecryption=True)
config = response['Parameter']['Value']

# ❌ MAL: Hardcoded
db_password = "SuperSecret123"  # NUNCA
```

### Encryption

**Encryption at Rest y in Transit:**

```hcl
# Terraform - S3 Bucket Encryption
resource "aws_s3_bucket" "data" {
  bucket = "${local.prefix}-data"
  
  tags = local.tags
}

# ✅ Server-side encryption
resource "aws_s3_bucket_server_side_encryption_configuration" "data" {
  bucket = aws_s3_bucket.data.id
  
  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm     = "aws:kms"
      kms_master_key_id = aws_kms_key.data.arn
    }
    bucket_key_enabled = true
  }
}

# ✅ Enforce SSL/TLS
resource "aws_s3_bucket_policy" "data" {
  bucket = aws_s3_bucket.data.id
  
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid       = "EnforceSSL"
        Effect    = "Deny"
        Principal = "*"
        Action    = "s3:*"
        Resource  = [
          aws_s3_bucket.data.arn,
          "${aws_s3_bucket.data.arn}/*"
        ]
        Condition = {
          Bool = {
            "aws:SecureTransport" = "false"
          }
        }
      }
    ]
  })
}
```

### VPC Security

**Network Segmentation:**

```
Production VPC (10.0.0.0/16)
├── Public Subnets (DMZ)
│   ├── 10.0.1.0/24 (us-east-1a) - ALB, NAT Gateway
│   ├── 10.0.2.0/24 (us-east-1b) - ALB, NAT Gateway
│   └── 10.0.3.0/24 (us-east-1c) - ALB, NAT Gateway
├── Private Subnets (Application)
│   ├── 10.0.11.0/24 (us-east-1a) - EC2, ECS
│   ├── 10.0.12.0/24 (us-east-1b) - EC2, ECS
│   └── 10.0.13.0/24 (us-east-1c) - EC2, ECS
└── Database Subnets (Data)
    ├── 10.0.21.0/24 (us-east-1a) - RDS, ElastiCache
    ├── 10.0.22.0/24 (us-east-1b) - RDS, ElastiCache
    └── 10.0.23.0/24 (us-east-1c) - RDS, ElastiCache
```

**Security Groups:**

```hcl
# ALB Security Group - Solo HTTPS
resource "aws_security_group" "alb" {
  name        = "${local.prefix}-alb-sg"
  vpc_id      = aws_vpc.main.id
  description = "Security group for Application Load Balancer"
  
  # Inbound - HTTPS from Internet
  ingress {
    description = "HTTPS from Internet"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  # Outbound to application tier
  egress {
    description     = "To application tier"
    from_port       = 8080
    to_port         = 8080
    protocol        = "tcp"
    security_groups = [aws_security_group.app.id]
  }
  
  tags = merge(local.tags, { Name = "${local.prefix}-alb-sg" })
}

# Application Security Group
resource "aws_security_group" "app" {
  name        = "${local.prefix}-app-sg"
  vpc_id      = aws_vpc.main.id
  description = "Security group for application servers"
  
  # Solo del ALB
  ingress {
    description     = "From ALB"
    from_port       = 8080
    to_port         = 8080
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id]
  }
  
  # Outbound a database
  egress {
    description     = "To database"
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.db.id]
  }
  
  tags = merge(local.tags, { Name = "${local.prefix}-app-sg" })
}

# Database Security Group
resource "aws_security_group" "db" {
  name        = "${local.prefix}-db-sg"
  vpc_id      = aws_vpc.main.id
  description = "Security group for database"
  
  # Solo de application tier
  ingress {
    description     = "From application"
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.app.id]
  }
  
  # No outbound necesario
  tags = merge(local.tags, { Name = "${local.prefix}-db-sg" })
}
```

---

## 3. Cost Optimization (High)

### EC2 Savings

**Right-sizing y Reserved Instances:**

```bash
# Analizar utilización
aws ce get-cost-and-usage \
  --time-period Start=2026-01-01,End=2026-02-01 \
  --granularity MONTHLY \
  --metrics UnblendedCost \
  --group-by Type=DIMENSION,Key=SERVICE

# Ver recomendaciones de right-sizing
aws ce get-rightsizing-recommendation \
  --service AmazonEC2 \
  --configuration RecommendationTarget=SAME_INSTANCE_FAMILY

# Ver recomendaciones de Reserved Instances
aws ce get-reservation-purchase-recommendation \
  --service AmazonEC2 \
  --lookback-period SIXTY_DAYS \
  --term-in-years ONE_YEAR
```

**Usar Spot Instances para cargas no-críticas:**

```hcl
# Auto Scaling Group con Spot Instances
resource "aws_autoscaling_group" "app" {
  name = "${local.prefix}-asg"
  
  min_size         = 2
  max_size         = 10
  desired_capacity = 4
  
  # ✅ Mix de On-Demand y Spot (60% ahorro)
  mixed_instances_policy {
    instances_distribution {
      on_demand_base_capacity                  = 2  # Mínimo on-demand
      on_demand_percentage_above_base_capacity = 20 # 80% Spot
      spot_allocation_strategy                 = "capacity-optimized"
    }
    
    launch_template {
      launch_template_specification {
        launch_template_id = aws_launch_template.app.id
        version            = "$Latest"
      }
      
      # Diversificar tipos de instancia
      override {
        instance_type = "t3.medium"
      }
      override {
        instance_type = "t3a.medium"
      }
      override {
        instance_type = "t2.medium"
      }
    }
  }
  
  vpc_zone_identifier = aws_subnet.private[*].id
  
  tags = concat(
    [for k, v in local.tags : { key = k, value = v, propagate_at_launch = true }],
    [{ key = "Name", value = "${local.prefix}-instance", propagate_at_launch = true }]
  )
}
```

### S3 Lifecycle Policies

```hcl
resource "aws_s3_bucket_lifecycle_configuration" "data" {
  bucket = aws_s3_bucket.data.id
  
  # Logs - Transicionar a Glacier después de 90 días
  rule {
    id     = "logs-lifecycle"
    status = "Enabled"
    
    filter {
      prefix = "logs/"
    }
    
    transition {
      days          = 30
      storage_class = "STANDARD_IA"
    }
    
    transition {
      days          = 90
      storage_class = "GLACIER"
    }
    
    expiration {
      days = 365
    }
  }
  
  # Backups - Intelligent Tiering
  rule {
    id     = "backups-lifecycle"
    status = "Enabled"
    
    filter {
      prefix = "backups/"
    }
    
    transition {
      days          = 0
      storage_class = "INTELLIGENT_TIERING"
    }
  }
}
```

### Cost Tags y Budgets

```hcl
# Budget con alertas
resource "aws_budgets_budget" "monthly" {
  name         = "${local.prefix}-monthly-budget"
  budget_type  = "COST"
  limit_amount = "1000"
  limit_unit   = "USD"
  time_unit    = "MONTHLY"
  
  cost_filter {
    name = "TagKeyValue"
    values = [
      "Project$ecommerce",
      "Environment$prod"
    ]
  }
  
  notification {
    comparison_operator       = "GREATER_THAN"
    threshold                  = 80
    threshold_type            = "PERCENTAGE"
    notification_type         = "ACTUAL"
    subscriber_email_addresses = ["devops@company.com"]
  }
  
  notification {
    comparison_operator       = "GREATER_THAN"
    threshold                  = 100
    threshold_type            = "PERCENTAGE"
    notification_type         = "ACTUAL"
    subscriber_email_addresses = ["cto@company.com"]
  }
}
```

---

## 4. High Availability & Reliability (Critical)

### Multi-AZ Deployment

**RDS Multi-AZ:**

```hcl
resource "aws_db_instance" "main" {
  identifier = "${local.prefix}-db"
  
  engine         = "postgres"
  engine_version = "15.4"
  instance_class = "db.t3.medium"
  
  # ✅ Multi-AZ para HA
  multi_az = true
  
  # ✅ Automated backups
  backup_retention_period = 30
  backup_window          = "03:00-04:00"
  maintenance_window     = "sun:04:00-sun:05:00"
  
  # ✅ Encryption
  storage_encrypted = true
  kms_key_id       = aws_kms_key.rds.arn
  
  # ✅ Deletion protection
  deletion_protection = true
  
  # ✅ Enhanced monitoring
  enabled_cloudwatch_logs_exports = ["postgresql", "upgrade"]
  monitoring_interval             = 60
  monitoring_role_arn            = aws_iam_role.rds_monitoring.arn
  
  tags = local.tags
}
```

### Auto Scaling

```hcl
# Target Tracking - CPU
resource "aws_autoscaling_policy" "cpu" {
  name                   = "${local.prefix}-cpu-scaling"
  autoscaling_group_name = aws_autoscaling_group.app.name
  policy_type            = "TargetTrackingScaling"
  
  target_tracking_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ASGAverageCPUUtilization"
    }
    target_value = 70.0
  }
}

# Target Tracking - ALB Request Count
resource "aws_autoscaling_policy" "alb" {
  name                   = "${local.prefix}-alb-scaling"
  autoscaling_group_name = aws_autoscaling_group.app.name
  policy_type            = "TargetTrackingScaling"
  
  target_tracking_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ALBRequestCountPerTarget"
      resource_label        = "${aws_lb.main.arn_suffix}/${aws_lb_target_group.app.arn_suffix}"
    }
    target_value = 1000.0
  }
}
```

### Health Checks

```hcl
# ALB Target Group con health checks
resource "aws_lb_target_group" "app" {
  name     = "${local.prefix}-tg"
  port     = 8080
  protocol = "HTTP"
  vpc_id   = aws_vpc.main.id
  
  # ✅ Health check configuration
  health_check {
    enabled             = true
    healthy_threshold   = 2
    unhealthy_threshold = 2
    timeout             = 5
    interval            = 30
    path                = "/health"
    matcher             = "200"
    protocol            = "HTTP"
  }
  
  # ✅ Deregistration delay
  deregistration_delay = 30
  
  # ✅ Stickiness (si necesario)
  stickiness {
    type            = "lb_cookie"
    cookie_duration = 86400
    enabled         = true
  }
  
  tags = local.tags
}
```

---

## 5. Monitoring & Observability (High)

### CloudWatch Dashboards

```hcl
resource "aws_cloudwatch_dashboard" "main" {
  dashboard_name = "${local.prefix}-dashboard"
  
  dashboard_body = jsonencode({
    widgets = [
      # ALB Metrics
      {
        type = "metric"
        properties = {
          metrics = [
            ["AWS/ApplicationELB", "TargetResponseTime", { stat = "Average" }],
            [".", "RequestCount", { stat = "Sum" }],
            [".", "HTTPCode_Target_5XX_Count", { stat = "Sum" }]
          ]
          period = 300
          stat   = "Average"
          region = var.aws_region
          title  = "ALB Performance"
        }
      },
      # RDS Metrics
      {
        type = "metric"
        properties = {
          metrics = [
            ["AWS/RDS", "CPUUtilization", { stat = "Average" }],
            [".", "DatabaseConnections", { stat = "Average" }],
            [".", "FreeableMemory", { stat = "Average" }]
          ]
          period = 300
          stat   = "Average"
          region = var.aws_region
          title  = "RDS Performance"
        }
      }
    ]
  })
}
```

### CloudWatch Alarms

```hcl
# ALB 5xx Errors
resource "aws_cloudwatch_metric_alarm" "alb_5xx" {
  alarm_name          = "${local.prefix}-alb-5xx"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "HTTPCode_Target_5XX_Count"
  namespace           = "AWS/ApplicationELB"
  period              = 300
  statistic           = "Sum"
  threshold           = 10
  alarm_description   = "ALB 5xx errors exceed threshold"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  
  dimensions = {
    LoadBalancer = aws_lb.main.arn_suffix
  }
  
  tags = local.tags
}

# RDS CPU High
resource "aws_cloudwatch_metric_alarm" "rds_cpu" {
  alarm_name          = "${local.prefix}-rds-cpu"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "CPUUtilization"
  namespace           = "AWS/RDS"
  period              = 300
  statistic           = "Average"
  threshold           = 80
  alarm_description   = "RDS CPU utilization is too high"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  
  dimensions = {
    DBInstanceIdentifier = aws_db_instance.main.id
  }
  
  tags = local.tags
}
```

### AWS X-Ray (Distributed Tracing)

```python
# Python - Flask con X-Ray
from aws_xray_sdk.core import xray_recorder
from aws_xray_sdk.ext.flask.middleware import XRayMiddleware

app = Flask(__name__)

# ✅ Habilitar X-Ray tracing
xray_recorder.configure(service='ecommerce-api')
XRayMiddleware(app, xray_recorder)

@app.route('/api/products')
@xray_recorder.capture('get_products')
def get_products():
    # X-Ray captura automáticamente
    products = db.query("SELECT * FROM products")
    return jsonify(products)
```

---

## 6. Backup & Disaster Recovery (Critical)

### RDS Automated Backups

```hcl
resource "aws_db_instance" "main" {
  # ... otras configuraciones ...
  
  # ✅ Backups automáticos
  backup_retention_period = 30  # 30 días
  backup_window          = "03:00-04:00"
  
  # ✅ Snapshot final antes de delete
  skip_final_snapshot       = false
  final_snapshot_identifier = "${local.prefix}-final-snapshot-${formatdate("YYYY-MM-DD-hhmm", timestamp())}"
  
  # ✅ Copy automated snapshots a otra región
  copy_tags_to_snapshot = true
}

# Cross-region snapshot copy
resource "aws_db_snapshot_copy" "replica" {
  provider = aws.disaster_recovery  # Otra región
  
  source_db_snapshot_identifier = aws_db_instance.main.latest_restorable_time
  target_db_snapshot_identifier = "${local.prefix}-dr-snapshot"
  
  kms_key_id = aws_kms_key.dr_region.arn
  
  tags = merge(local.tags, {
    Type = "disaster-recovery"
  })
}
```

### S3 Cross-Region Replication

```hcl
# Bucket de origen
resource "aws_s3_bucket" "source" {
  bucket = "${local.prefix}-data"
  
  tags = local.tags
}

resource "aws_s3_bucket_versioning" "source" {
  bucket = aws_s3_bucket.source.id
  
  versioning_configuration {
    status = "Enabled"  # ✅ Requerido para replication
  }
}

# Bucket de destino (otra región)
resource "aws_s3_bucket" "destination" {
  provider = aws.disaster_recovery
  
  bucket = "${local.prefix}-data-dr"
  
  tags = merge(local.tags, { Type = "disaster-recovery" })
}

# Replication configuration
resource "aws_s3_bucket_replication_configuration" "source" {
  bucket = aws_s3_bucket.source.id
  role   = aws_iam_role.replication.arn
  
  rule {
    id     = "replicate-all"
    status = "Enabled"
    
    destination {
      bucket        = aws_s3_bucket.destination.arn
      storage_class = "STANDARD_IA"
      
      # ✅ Replication Time Control (15 min SLA)
      replication_time {
        status = "Enabled"
        time {
          minutes = 15
        }
      }
      
      # ✅ Metrics
      metrics {
        status = "Enabled"
        event_threshold {
          minutes = 15
        }
      }
    }
    
    # Filtro opcional
    filter {
      prefix = "important/"
    }
  }
}
```

---

## 7. Performance Optimization (Medium)

### CloudFront CDN

```hcl
resource "aws_cloudfront_distribution" "main" {
  enabled             = true
  is_ipv6_enabled     = true
  comment             = "${local.prefix} CDN"
  default_root_object = "index.html"
  
  # Origin - S3
  origin {
    domain_name = aws_s3_bucket.frontend.bucket_regional_domain_name
    origin_id   = "S3-${aws_s3_bucket.frontend.id}"
    
    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.main.cloudfront_access_identity_path
    }
  }
  
  # Origin - ALB
  origin {
    domain_name = aws_lb.main.dns_name
    origin_id   = "ALB-${aws_lb.main.name}"
    
    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }
  
  # Default cache behavior
  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD", "OPTIONS"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "S3-${aws_s3_bucket.frontend.id}"
    viewer_protocol_policy = "redirect-to-https"
    compress               = true
    
    # ✅ Cache optimizado
    min_ttl     = 0
    default_ttl = 3600
    max_ttl     = 86400
    
    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
  }
  
  # API cache behavior
  ordered_cache_behavior {
    path_pattern     = "/api/*"
    allowed_methods  = ["GET", "HEAD", "OPTIONS", "PUT", "POST", "PATCH", "DELETE"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "ALB-${aws_lb.main.name}"
    
    viewer_protocol_policy = "https-only"
    compress               = true
    
    # ✅ No cache para API
    min_ttl     = 0
    default_ttl = 0
    max_ttl     = 0
    
    forwarded_values {
      query_string = true
      headers      = ["Authorization", "Accept", "Content-Type"]
      cookies {
        forward = "all"
      }
    }
  }
  
  # ✅ SSL/TLS
  viewer_certificate {
    acm_certificate_arn      = aws_acm_certificate.main.arn
    minimum_protocol_version = "TLSv1.2_2021"
    ssl_support_method       = "sni-only"
  }
  
  # ✅ Geo restriction (opcional)
  restrictions {
    geo_restriction {
      restriction_type = "whitelist"
      locations        = ["US", "CA", "GB", "DE"]
    }
  }
  
  tags = local.tags
}
```

### ElastiCache Redis

```hcl
resource "aws_elasticache_replication_group" "main" {
  replication_group_id       = "${local.prefix}-redis"
  replication_group_description = "Redis cluster for caching"
  
  engine               = "redis"
  engine_version       = "7.0"
  node_type            = "cache.t3.medium"
  number_cache_clusters = 3  # 1 primary + 2 replicas
  
  # ✅ Multi-AZ
  multi_az_enabled           = true
  automatic_failover_enabled = true
  
  # ✅ Encryption
  at_rest_encryption_enabled = true
  transit_encryption_enabled = true
  auth_token                = random_password.redis.result
  
  # Subnet group
  subnet_group_name = aws_elasticache_subnet_group.main.name
  security_group_ids = [aws_security_group.redis.id]
  
  # Maintenance
  maintenance_window       = "sun:05:00-sun:06:00"
  snapshot_window         = "03:00-04:00"
  snapshot_retention_limit = 7
  
  tags = local.tags
}
```

---

## 8. Infrastructure as Code Best Practices (High)

### Terraform State Locking

Ver **terraform-best-practices** skill para detalles completos.

### CloudFormation StackSets

Para multi-account/multi-region deployments:

```yaml
# cloudformation/baseline-security.yaml
AWSTemplateFormatVersion: '2010-09-09'
Description: Baseline security configuration for all accounts

Parameters:
  Environment:
    Type: String
    AllowedValues: [dev, staging, prod]
  
Resources:
  # S3 Block Public Access (Account-level)
  S3AccountPublicAccessBlock:
    Type: AWS::S3::AccountPublicAccessBlock
    Properties:
      BlockPublicAcls: true
      BlockPublicPolicy: true
      IgnorePublicAcls: true
      RestrictPublicBuckets: true
  
  # CloudTrail
  CloudTrailBucket:
    Type: AWS::S3::Bucket
    Properties:
      BucketName: !Sub '${Environment}-${AWS::AccountId}-cloudtrail'
      PublicAccessBlockConfiguration:
        BlockPublicAcls: true
        BlockPublicPolicy: true
        IgnorePublicAcls: true
        RestrictPublicBuckets: true
  
  CloudTrail:
    Type: AWS::CloudTrail::Trail
    Properties:
      TrailName: !Sub '${Environment}-org-trail'
      S3BucketName: !Ref CloudTrailBucket
      IsLogging: true
      IsMultiRegionTrail: true
      IncludeGlobalServiceEvents: true
      EnableLogFileValidation: true
```

---

## 9. Best Practices Checklist

### Security Checklist

- [ ] MFA habilitado en root account
- [ ] IAM roles usados en lugar de access keys
- [ ] Secrets en Secrets Manager/Parameter Store (no hardcoded)
- [ ] Encryption at rest habilitada (S3, RDS, EBS)
- [ ] Encryption in transit habilitada (SSL/TLS)
- [ ] Security groups con least privilege
- [ ] VPC Flow Logs habilitados
- [ ] CloudTrail habilitado en todas las regiones
- [ ] GuardDuty habilitado
- [ ] AWS Config habilitado
- [ ] Backups automáticos configurados

### Cost Optimization Checklist

- [ ] Right-sizing de instancias realizado
- [ ] Reserved Instances/Savings Plans comprados
- [ ] Spot Instances usadas para cargas no-críticas
- [ ] S3 Lifecycle policies configuradas
- [ ] Auto Scaling configurado
- [ ] CloudWatch alarms de presupuesto configurados
- [ ] Recursos no usados eliminados regularmente
- [ ] Cost tags aplicados a todos los recursos

### High Availability Checklist

- [ ] Multi-AZ deployments para componentes críticos
- [ ] Auto Scaling configurado
- [ ] Health checks configurados
- [ ] Backups automáticos habilitados
- [ ] Cross-region replication configurada (si aplica)
- [ ] Disaster recovery plan documentado
- [ ] RTO/RPO definidos y testeados

---

## Recursos

- [AWS Well-Architected Framework](https://aws.amazon.com/architecture/well-architected/)
- [AWS Security Best Practices](https://docs.aws.amazon.com/security/)
- [AWS Cost Optimization](https://aws.amazon.com/pricing/cost-optimization/)
- [AWS Disaster Recovery](https://aws.amazon.com/disaster-recovery/)
- [AWS Architecture Center](https://aws.amazon.com/architecture/)

---

**Última actualización**: 2026-02-18  
**Mantenido por**: Cloud Architecture Team
