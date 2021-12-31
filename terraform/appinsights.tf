resource "azurerm_log_analytics_workspace" "logWorkspace" {
  name                = "log-snake-${var.ENVIRONMENT}"
  location            = data.azurerm_resource_group.rg.location
  resource_group_name = data.azurerm_resource_group.rg.name
  sku                 = "PerGB2018"
  retention_in_days   = 30
  daily_quota_gb      = 1
  tags                = local.common_tags
}

resource "azurerm_application_insights" "appInsights" {
  name                = "appi-snake-${var.ENVIRONMENT}"
  location            = data.azurerm_resource_group.rg.location
  resource_group_name = data.azurerm_resource_group.rg.name
  workspace_id        = azurerm_log_analytics_workspace.logWorkspace.id
  application_type    = "web"
  retention_in_days   = 30
  tags                = local.common_tags
}
