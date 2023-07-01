terraform {
  backend "azurerm" {
    resource_group_name  = "rg-snake-staging"
    storage_account_name = "stsnakestaging"
    container_name       = "tfstate"
    key                  = "staging.terraform.tfstate"
    use_oidc             = true
  }

  required_providers {
    azurerm = {
      version = "~>3.10"
    }
  }
}

provider "azurerm" {
  use_oidc = true
  features {}
}
