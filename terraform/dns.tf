# DNS doesnt perfectly fit "staging -> prod" lifecycle
# So it's a little hacky

resource "azurerm_dns_zone" "playsnakePublic" {
  count = var.ENVIRONMENT == "prod" ? 1 : 0

  name                = "playsnake.no"
  resource_group_name = data.azurerm_resource_group.rg.name
  tags                = local.common_tags
}

resource "azurerm_dns_cname_record" "app" {
  count = var.ENVIRONMENT == "prod" ? 1 : 0

  name                = "www"
  zone_name           = azurerm_dns_zone.playsnakePublic[0].name
  resource_group_name = azurerm_dns_zone.playsnakePublic[0].resource_group_name
  ttl                 = 300
  target_resource_id  = azurerm_static_site.app[0].id
  tags                = local.common_tags
}

resource "azurerm_dns_cname_record" "highScoreApi" {
  name                = var.ENVIRONMENT == "prod" ? "highscores" : "highscores-${var.ENVIRONMENT}"
  zone_name           = "playsnake.no"  # hardcoded because sometimes different env
  resource_group_name = "rg-snake-prod" # hardcoded because sometimes different env
  ttl                 = 300
  record              = azurerm_container_app.highscoreApi.ingress[0].fqdn
  tags                = local.common_tags
}

resource "azurerm_dns_txt_record" "highScoreApiVerification" {
  name                = "asuid.${azurerm_dns_cname_record.highScoreApi.name}"
  zone_name           = "playsnake.no"  # hardcoded because sometimes different env
  resource_group_name = "rg-snake-prod" # hardcoded because cometimes different env
  ttl                 = 300

  record {
    value = azurerm_container_app.highscoreApi.custom_domain_verification_id
  }

  tags = local.common_tags
}
