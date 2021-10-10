resource "mongodbatlas_project" "project" {
  name   = "snake-${var.ENVIRONMENT}"
  org_id = var.MONGO_ORG_ID
}

# TODO / Manual Step:
# As of now (October, 2021) one cannot programatically give the Mongo API key
# used to provisions these resources the "Project Owner" role on the above project.
# So this must be done manually in cloud.mongodb.com access manager before creation
# of the below resources will succeed

resource "mongodbatlas_cluster" "db" {
  project_id                  = mongodbatlas_project.project.id
  name                        = "${var.ENVIRONMENT}-azure-${var.MONGO_TIER}-snake"
  cluster_type                = "SHARED"
  provider_name               = "AZURE"
  provider_region_name        = "EUROPE_WEST"
  provider_instance_size_name = var.MONGO_TIER
}

resource "random_password" "funcDbPassword" {
  length  = 16
  special = false
}

resource "mongodbatlas_database_user" "funcUser" {
  username           = "func-snake"
  password           = random_password.funcDbPassword.result
  project_id         = mongodbatlas_project.project.id
  auth_database_name = "admin"

  roles {
    role_name     = "readWrite"
    database_name = mongodbatlas_cluster.db.name
  }

  roles {
    role_name     = "readAnyDatabase"
    database_name = "admin"
  }

  scopes {
    name = mongodbatlas_cluster.db.name
    type = "CLUSTER"
  }
}

resource "azurerm_key_vault_secret" "mongoConnectionString" {
  name         = "connstring-mongodb"
  value        = mongodbatlas_cluster.db.connection_strings.0.standard_srv
  key_vault_id = azurerm_key_vault.vault.id
  tags         = local.common_tags
}
