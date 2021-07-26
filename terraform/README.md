## Terraform / Infrastructure

Provisions infrastructure needed for our webapp / api.

### Initial infrastructure required for terraform state management

A resource group, storage account and storage container must be provisioned
manually for state management.

Something like this
```sh
az group create --name rg-snake-prod --location norwayeast
az storage account create --name stsnakeprod --resource-group rg-snake-prod
az storage container create --account-name stsnakeprod --name tfstate
```

### Running locally

Requires having access to the subscription/resource group.

```sh
az login
az account set -s <azure-subscription>
terraform init
```
