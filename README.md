# Hygea 🩺 – Personal Health Companion Platform

**Hygea** is an extensible, privacy-first health companion platform designed to help individuals track, understand, and take control of their health data.
Built with a modular architecture, Hygea starts with a health journaling MVP and grows toward a complete personal health ecosystem.


## 🚀 Features (Alpha Version)


- 🧾 **Secure user authentication** via Keycloak
- 👤 **User profile management** (age, weight, height, diabetes status, etc.)
- 🗃️ **Health data storage** with PostgreSQL
- 🛡️ **JWT-protected API** built in Rust with Axum
- 📊 **Observability stack** (Prometheus + Grafana)
- 🐇 Ready for **asynchronous event-driven architecture** with RabbitMQ


## 🏗️ Tech Stack

| Layer       | Technology                     |
|-------------|--------------------------------|
| Frontend    | React + ShadCN + TailwindCSS   |
| Backend     | Rust + Axum                    |
| Auth        | Keycloak                       |
| Database    | PostgreSQL (CloudNativePG)     |
| Messaging   | RabbitMQ                       |
| Monitoring  | Prometheus + Grafana           |
| Infra       | Kubernetes (k3d) + Helm + Terraform |


## 📦 Helm Charts Structure

| Chart Name       | Purpose                                      |
|------------------|----------------------------------------------|
| `infrastructure` | Core components (RabbitMQ Operator, monitoring stack) |
| `hygea-stack`    | Database, RabbitMQ cluster, Keycloak         |
| `hygea-apps`     | Application deployments (API, frontend, etc.)|



## 📚 Getting Started (Dev)

> **Prerequisites:**  
> - Docker  
> - `k3d`, `kubectl`, `helm`, and `terraform`

```bash
# Create local Kubernetes cluster
terraform -chdir=infrastructure/terraform init
terraform -chdir=infrastructure/terraform apply

# Deploy helm charts manually
helm upgrade --install --create-namespace --namespace hygea hygea-stack ./infrastructure/charts/hygea-stack

helm upgrade --install --create-namespace --namespace infrastructure infrastructure ./infrastructure/charts/infra
```