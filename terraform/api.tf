resource "azurerm_container_app_environment" "containerAppEnv" {
  name                       = "cae-snake-${var.ENVIRONMENT}"
  location                   = data.azurerm_resource_group.rg.location
  resource_group_name        = data.azurerm_resource_group.rg.name
  log_analytics_workspace_id = azurerm_log_analytics_workspace.logWorkspace.id
}

resource "azurerm_container_app" "highscoreApi" {
  name                         = "ca-snakehighscoreapi-${var.ENVIRONMENT}"
  container_app_environment_id = azurerm_container_app_environment.containerAppEnv.id
  resource_group_name          = azurerm_container_app_environment.containerAppEnv.resource_group_name
  revision_mode                = "Single"

  template {
    min_replicas = 0
    max_replicas = 3

    container {
      name   = "highscore-api"
      image  = "ghcr.io/christianfosli/snake/highscore-api:latest" # <-- tag will be overridden by ci/cd
      cpu    = 0.25
      memory = "0.5Gi"

      env {
        name        = "DB_CONNSTR"
        secret_name = "db-connstr"
      }

      env {
        name  = "LISTEN_ADDR"
        value = "0.0.0.0:3000"
      }

      liveness_probe {
        transport = "HTTP"
        path      = "/livez"
        port      = 3000
      }

      readiness_probe {
        transport = "HTTP"
        path      = "/readyz"
        port      = 3000
      }
    }
  }

  ingress {
    external_enabled = true
    target_port      = 3000

    traffic_weight {
      percentage = 100
    }

    # traffic weight will be adjusted during CI/CD as new revisions are published

    # custom domain must be configured in the azure portal for now
    # because custom domain with managed TLS certificates is currently (July, 2023)
    # not supported by azurerm terraform provider

    # cors must be configured in the azure portal for now
    # because it is currently (July, 2023) not supported by azurerm terraform provider
  }

  secret {
    name  = "db-connstr"
    value = azurerm_key_vault_secret.mongoConnectionString.value
  }

  lifecycle {
    ignore_changes = [template[0].container[0].image, ingress[0]]
  }

  tags = local.common_tags
}
