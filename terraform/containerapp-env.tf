resource "azurerm_container_app_environment" "containerAppEnv" {
  name                       = "cae-snake-${var.ENVIRONMENT}"
  location                   = data.azurerm_resource_group.rg.location
  resource_group_name        = data.azurerm_resource_group.rg.name
  log_analytics_workspace_id = azurerm_log_analytics_workspace.logWorkspace.id
}

