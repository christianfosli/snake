resource "azurerm_key_vault" "vault" {
  name                       = "kv-snake-${var.ENVIRONMENT}"
  location                   = data.azurerm_resource_group.rg.location
  resource_group_name        = data.azurerm_resource_group.rg.name
  tenant_id                  = data.azurerm_client_config.current.tenant_id
  soft_delete_retention_days = 7
  purge_protection_enabled   = false

  sku_name = "standard"

  tags = local.common_tags
}

resource "azurerm_key_vault_access_policy" "tfAgent" {
  key_vault_id = azurerm_key_vault.vault.id
  tenant_id    = data.azurerm_client_config.current.tenant_id
  object_id    = data.azurerm_client_config.current.object_id

  key_permissions         = ["Get"]
  secret_permissions      = ["Get", "Set", "Delete", "Purge", "Recover", "Restore"]
  certificate_permissions = ["Get"]
}
