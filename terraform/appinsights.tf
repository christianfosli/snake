resource "azurerm_application_insights" "appInsights" {
  name                = "appi-snake-${var.ENVIRONMENT}"
  location            = data.azurerm_resource_group.rg.location
  resource_group_name = data.azurerm_resource_group.rg.name
  application_type    = "web"
  retention_in_days   = 30
  tags                = local.common_tags
}
