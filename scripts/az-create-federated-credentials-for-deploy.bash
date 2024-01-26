#!/bin/bash
set -eo pipefail

printf 'Creating app for staging\n'
az ad app create --display-name christianfosli/snake-staging
sleep 2
appId="$(az ad app list --display-name christianfosli/snake-staging --query [0].appId -o tsv)"
az ad sp create --id "$appId"

printf 'Creating federated credentials for GitHub Actions\n'
cat << EOF > creds.json
{
  "audiences": [
    "api://AzureADTokenExchange"
  ],
  "description": "Permission to access Azure from GitHub actions workflows - staging",
  "issuer": "https://token.actions.githubusercontent.com",
  "name": "christianfosli-snake-github-actions-staging",
  "subject": "repo:christianfosli/snake:environment:staging"
}
EOF
az ad app federated-credential create --id "$appId" --parameters creds.json

printf 'Creating app for prod\n'
az ad app create --display-name christianfosli/snake-prod
sleep 2
appId="$(az ad app list --display-name christianfosli/snake-prod --query [0].appId -o tsv)"
az ad sp create --id "$appId"

printf 'Creating federated credentials for GitHub Actions\n'
cat << EOF > creds.json
{
  "audiences": [
    "api://AzureADTokenExchange"
  ],
  "description": "Permission to access Azure from GitHub actions workflows - prod",
  "issuer": "https://token.actions.githubusercontent.com",
  "name": "christianfosli-snake-github-actions-prod",
  "subject": "repo:christianfosli/snake:environment:prod"
}
EOF
az ad app federated-credential create --id $appId --parameters creds.json

printf '!!MANUAL STEP: Assign permissions in Azure portal\n'
