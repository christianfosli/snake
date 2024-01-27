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

resource "azurerm_static_site_custom_domain" "app" {
  count = var.ENVIRONMENT == "prod" ? 1 : 0

  static_site_id  = azurerm_static_site.app[0].id
  domain_name     = "${azurerm_dns_cname_record.app[0].name}.${azurerm_dns_cname_record.app[0].zone_name}"
  validation_type = "cname-delegation"
}

# Manual Steps
# - Associate with GitHub target repository / add deployment token in GitHub secrets
