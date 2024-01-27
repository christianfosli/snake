resource "azurerm_key_vault" "vault" {
  name                       = "kv-snake-${var.ENVIRONMENT}"
  location                   = data.azurerm_resource_group.rg.location
  resource_group_name        = data.azurerm_resource_group.rg.name
  tenant_id                  = data.azurerm_client_config.current.tenant_id
  soft_delete_retention_days = 7
  enable_rbac_authorization  = true
  purge_protection_enabled   = false

  sku_name = "standard"

  tags = local.common_tags
}
