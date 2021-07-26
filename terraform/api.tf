resource "azurerm_app_service_plan" "apiPlan" {
  name                = "asp-snake-${var.ENVIRONMENT}"
  location            = data.azurerm_resource_group.rg.location
  resource_group_name = data.azurerm_resource_group.rg.name
  kind                = "Linux"
  reserved            = true

  sku {
    tier = "Dynamic"
    size = "Y1"
  }

  tags = local.common_tags
}

resource "azurerm_function_app" "api" {
  name                       = "func-snake-${var.ENVIRONMENT}"
  location                   = data.azurerm_resource_group.rg.location
  resource_group_name        = data.azurerm_resource_group.rg.name
  app_service_plan_id        = azurerm_app_service_plan.apiPlan.id
  storage_account_name       = data.azurerm_storage_account.st.name
  storage_account_access_key = data.azurerm_storage_account.st.primary_access_key
  os_type                    = "linux"
  version                    = "~3"

  app_settings = {
    "CONNECTION_STRING"        = "@Microsoft.KeyVault(SecretUri=${azurerm_key_vault_secret.mongoConnectionString.id})"
    "FUNCTIONS_WORKER_RUNTIME" = "dotnet-isolated"
  }

  site_config {
    ftps_state = "Disabled"
  }

  identity {
    type = "SystemAssigned"
  }

  tags = local.common_tags
}
