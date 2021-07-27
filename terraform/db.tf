# TODO:
# I would like to provision the database and database user here.
# Unfortunately creating free tier (M0) MongoDB clusters is not supported
# by MongoDB API/terraform provider, and therefore I create it manually on mongodb.com instead.
# Move it here if I decide to up the SKU

resource "azurerm_key_vault_secret" "mongoConnectionString" {
  # value      = "dummyValue"
  name         = "connstring-mongodb"
  key_vault_id = azurerm_key_vault.vault.id
  tags         = local.common_tags
}
