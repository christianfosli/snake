# Currently (July 2023) terraform azurerm provider does not support declaring azure container
# app jobs. Azure CLI to the rescue

resource "null_resource" "createCleanupJob" {
  triggers = {
    container_app_env = azurerm_container_app_environment.containerAppEnv.id
  }

  provisioner "local-exec" {
    command = <<EOF
    set -eo pipefail

    az extension add -n containerapp

    az containerapp job create -n "caj-snakehighscoreclean-${var.ENVIRONMENT}" \
      -g "${azurerm_container_app_environment.containerAppEnv.resource_group_name}" \
      --environment "${azurerm_container_app_environment.containerAppEnv.name}" \
      --trigger-type "Schedule" --cron-expression "0 7 * * 0" \
      --replica-timeout 600 --replica-retry-limit 1 --replica-completion-count 1 --parallelism 1 \
      --image "ghcr.io/christianfosli/snake/highscore-cleanup-job:latest" \
      --cpu "0.25" --memory "0.5Gi" \
      --secrets "db-connstr=${nonsensitive(azurerm_key_vault_secret.mongoConnectionString.value)}" \
      --env-vars "DB_CONNSTR=secretref:db-connstr"
    EOF

    interpreter = ["/bin/bash", "-c"]

    # nonsensitive used on sensitive value to prevent all stdout from being hidden
  }
}
