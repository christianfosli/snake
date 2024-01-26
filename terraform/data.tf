data "azurerm_client_config" "current" {}

data "azurerm_resource_group" "rg" {
  name = "rg-snake-${var.ENVIRONMENT}"
}

data "azurerm_storage_account" "st" {
  name                = "stsnake${var.ENVIRONMENT}2"
  resource_group_name = data.azurerm_resource_group.rg.name
}

variable "ENVIRONMENT" {
  type    = string
  default = "staging"
}

locals {
  common_tags = {
    Owner       = "Christian Fosli"
    Application = "snake (hobby project)"
    Environment = var.ENVIRONMENT
    CreatedBy   = "terraform"
  }
}
