terraform {
  backend "azurerm" {
    resource_group_name  = "rg-snake-staging"
    storage_account_name = "stsnakestaging"
    container_name       = "tfstate"
    key                  = "staging.terraform.tfstate"
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
