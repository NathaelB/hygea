resource "null_resource" "k3d_cluster" {
  triggers = {
    cluster = var.cluster_name
  }


  provisioner "local-exec" {
    when        = create
    interpreter = ["bash", "-c"]
    command     = <<-EOT
      k3d cluster create ${self.triggers.cluster} \
        --registry-create ${self.triggers.cluster}-registry:0.0.0.0:5000 \
        --k3s-arg "--disable=traefik@server:*" \
        -p "8443:443@loadbalancer" \
        -p "8080:80@loadbalancer" \
        --servers 1 \
        --agents 3
    EOT
  }

  provisioner "local-exec" {
    when        = destroy
    interpreter = ["bash", "-c"]
    command     = "k3d cluster delete ${self.triggers.cluster}"
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "helm_release" "monitoring" {
  depends_on       = [null_resource.k3d_cluster]
  name             = "monitoring"
  repository       = "https://prometheus-community.github.io/helm-charts"
  chart            = "kube-prometheus-stack"
  namespace        = "monitoring"
  create_namespace = true

  set {
    name  = "grafana.adminUser"
    value = var.grafana_admin_user
  }
  set {
    name  = "grafana.adminPassword"
    value = var.grafana_admin_password
  }
  set {
    name  = "grafana.service.type"
    value = "LoadBalancer"
  }

  set {
    name  = "prometheus.prometheusSpec.podMonitorSelectorNilUsesHelmValues"
    value = "true"
  }
}

resource "null_resource" "grafana_port_forward" {
  depends_on = [helm_release.monitoring]

  provisioner "local-exec" {
    when        = "create"
    interpreter = ["bash", "-c"]
    command     = <<-EOT
      nohup kubectl port-forward -n monitoring svc/monitoring-grafana 3000:80 \
        >/tmp/grafana-port-forward.log 2>&1 & echo $! > /tmp/grafana-pf.pid
    EOT
  }

  provisioner "local-exec" {
    when        = "destroy"
    interpreter = ["bash", "-c"]
    command     = "kill $(cat /tmp/grafana-pf.pid) || true"
  }

  triggers = { pf = "init" }
}

resource "helm_release" "infrastructure" {
  depends_on = [helm_release.monitoring, null_resource.grafana_port_forward]
  name       = "infrastructure"
  chart      = "../charts/infra"
  repository = null

  namespace        = "infrastructure"
  create_namespace = true
}

resource "helm_release" "hygea_stack" {
  depends_on = [helm_release.monitoring, null_resource.grafana_port_forward, helm_release.infrastructure]
  name       = "hygea-stack"
  chart      = "../charts/hygea-stack"
  repository = null

  namespace        = "hygea"
  create_namespace = true
}

resource "grafana_folder" "hygea_folder" {
  depends_on = [null_resource.grafana_port_forward]
  title      = "Hygea"

}


resource "grafana_dashboard" "rabbitmq" {
  depends_on  = [null_resource.grafana_port_forward]
  config_json = file("${path.module}/dashboards/rabbitmq_dashboard.json")
  folder      = grafana_folder.hygea_folder.id
}


resource "grafana_dashboard" "k8s_overview" {
  depends_on  = [null_resource.grafana_port_forward]
  config_json = file("${path.module}/dashboards/dashboard_15757.json")
  folder      = grafana_folder.hygea_folder.id
}

resource "grafana_dashboard" "pg_monitoring" {
  depends_on  = [null_resource.grafana_port_forward]
  config_json = file("${path.module}/dashboards/pg_dashboard.json")
  folder      = grafana_folder.hygea_folder.id
}

resource "grafana_dashboard" "keycloak_monitoring" {
  depends_on = [null_resource.grafana_port_forward]
  config_json = file("${path.module}/dashboards/keycloak_dashboard.json")
  folder = grafana_folder.hygea_folder.id
}