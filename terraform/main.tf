terraform {
  backend "azurerm" {
    resource_group_name  = "rg-snake-prod"
    storage_account_name = "stsnakeprod"
    container_name       = "tfstate"
    key                  = "prod.terraform.tfstate"
  }

  required_providers {
    azurerm = {
      version = "~>2.69"
    }
  }
}

provider "azurerm" {
  features {}
}
