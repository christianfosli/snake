resource "azurerm_key_vault" "vault" {
  name                       = "kv-snake-${var.ENVIRONMENT}"
  location                   = data.azurerm_resource_group.rg.location
  resource_group_name        = data.azurerm_resource_group.rg.name
  tenant_id                  = data.azurerm_client_config.current.tenant_id
  soft_delete_enabled        = true
  soft_delete_retention_days = 7
  purge_protection_enabled   = false

  sku_name = "standard"

  access_policy {
    tenant_id = data.azurerm_client_config.current.tenant_id
    object_id = data.azurerm_client_config.current.object_id

    key_permissions        = ["Get"]
    secret_permissions     = ["Get", "Set", "Delete", "Purge", "Recover", "Restore"]
    cerificate_permissions = ["Get"]
  }

  access_policy {
    tenant_id = data.azurerm_client_config.current.tenant_id
    object_id = azurerm_function_app.api.identity.0.object_id

    key_permissions        = ["Get"]
    secret_permissions     = ["Get"]
    cerificate_permissions = ["Get"]
  }

  tags = local.common_tags
}
