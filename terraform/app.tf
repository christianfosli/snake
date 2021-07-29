resource "azurerm_static_site" "app" {
  # Static web app deploys staging versions automatically
  # as PR's are opened, thus there is no need for a specific staging resource
  count = var.ENVIRONMENT == "prod" ? 1 : 0

  name                = "stapp-snake-${var.ENVIRONMENT}"
  resource_group_name = data.azurerm_resource_group.rg.name
  location            = "West Europe" # Norway not yet supported for azure static webapp
  sku_tier            = "Free"
  sku_size            = "Free"
  tags                = local.common_tags
}

# Manual Steps
# - Associate with GitHub target repository
