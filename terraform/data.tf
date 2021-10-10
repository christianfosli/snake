data "azurerm_client_config" "current" {}

data "azurerm_resource_group" "rg" {
  name = "rg-snake-${var.ENVIRONMENT}"
}

data "azurerm_storage_account" "st" {
  name                = "stsnake${var.ENVIRONMENT}"
  resource_group_name = data.azurerm_resource_group.rg.name
}

variable "ENVIRONMENT" {
  type    = string
  default = "staging"
}

variable "MONGO_ORG_ID" {
  type    = string
  default = "5f15e102ebca9f3ede3df37c"
}

variable "MONGO_TIER" {
  type    = string
  default = "M0"
}

locals {
  common_tags = {
    Owner       = "Christian Fosli"
    Application = "snake (hobby project)"
    Environment = var.ENVIRONMENT
    CreatedBy   = "terraform"
  }
}
