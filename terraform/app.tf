resource "azurerm_static_site" "app" {
  name                = "stapp-snake-${var.ENVIRONMENT}"
  resource_group_name = data.azurerm_resource_group.rg.name
  location            = "West Europe" # Norway not yet supported for azure static webapp
  tags                = local.common_tags
}

# TODO / Manual Step
# Linking function app and static web app together is not yet possible as IaC
# Done manually in the portal following https://docs.microsoft.com/en-us/azure/static-web-apps/functions-bring-your-own
