resource "azurerm_static_site" "app" {
  name                = "stapp-snake-${var.ENVIRONMENT}"
  resource_group_name = data.azurerm_resource_group.rg.name
  location            = "West Europe" # Norway not yet supported for azure static webapp
  default_host_name   = "snake-${var.ENVIRONMENT}.azurestaticapps.net"
  sku_tier            = "Standard" # Free tier does not support bring-your-own functions
  sku_size            = "Standard" # Free tier does not support bring-your-own functions
  tags                = local.common_tags
}

# Manual Steps
# - Associate with GitHub target repository
# - Link function app and static web app together
#   (https://docs.microsoft.com/en-us/azure/static-web-apps/functions-bring-your-own)
