resource "azurerm_app_service_plan" "apiPlan" {
  name                = "asp-snake-${var.ENVIRONMENT}"
  location            = "West Europe" # Norway not yet supported for func with Linux consumption plan
  resource_group_name = data.azurerm_resource_group.rg.name
  kind                = "FunctionApp"
  reserved            = true

  sku {
    tier = "Dynamic"
    size = "Y1"
  }

  tags = local.common_tags
}

resource "azurerm_function_app" "highScoreApi" {
  name                       = "func-snakehighscores-${var.ENVIRONMENT}"
  location                   = azurerm_app_service_plan.apiPlan.location
  resource_group_name        = azurerm_app_service_plan.apiPlan.resource_group_name
  app_service_plan_id        = azurerm_app_service_plan.apiPlan.id
  storage_account_name       = data.azurerm_storage_account.st.name
  storage_account_access_key = data.azurerm_storage_account.st.primary_access_key
  os_type                    = "linux"
  version                    = "~3"

  app_settings = {
    "CONNECTION_STRING"               = "@Microsoft.KeyVault(SecretUri=${azurerm_key_vault_secret.mongoConnectionString.id})"
    "FUNCTIONS_WORKER_RUNTIME"        = "dotnet-isolated"
    "APPINSIGHTS_INSTRUMENTATION_KEY" = azurerm_application_insights.appInsights.instrumentation_key
  }

  auth_settings {
    enabled = false
  }

  site_config {
    ftps_state = "Disabled"
  }

  identity {
    type = "SystemAssigned"
  }

  tags = local.common_tags
}
