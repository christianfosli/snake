# DNS doesnt perfectly fit "staging -> prod" lifecycle
# So it's a little hacky

resource "azurerm_dns_zone" "playsnakePublic" {
  count = var.ENVIRONMENT == "prod" ? 1 : 0

  name                = "playsnake.no"
  resource_group_name = data.azurerm_resource_group.rg.name
}

resource "azurerm_dns_cname_record" "app" {
  count = var.ENVIRONMENT == "prod" ? 1 : 0

  name                = "www"
  zone_name           = azurerm_dns_zone.playsnakePublic[0].name
  resource_group_name = azurerm_dns_zone.playsnakePublic[0].resource_group_name
  ttl                 = 300
  target_resource_id  = azurerm_static_site.app[0].id
}

#resource "azurerm_dns_cname_record" "highscoreApi" {
#  name                = var.ENVIRONMENT == "prod" ? "highscores" : "highscores-${var.ENVIRONMENT}"
#  zone_name           = "playsnake.no"  # hardcoded because sometimes different env
#  resource_group_name = "rg-snake-prod" # hardcoded because sometimes different env
#  ttl                 = 300
#  target_resource_id  = azurerm_function_app.highScoreApi.id
#}
