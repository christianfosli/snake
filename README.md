# snake

Snake with vi/vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

![wasm_app](https://github.com/christianfosli/visnake-wasm/workflows/wasm_app/badge.svg)
![highscore_functions](https://github.com/christianfosli/visnake-wasm/workflows/highscore_functions/badge.svg)
![terraform](https://github.com/christianfosli/snake/actions/workflows/terraform.yml/badge.svg)

## Architecture 🏗

```
|--------------------------------|      |-------------------------|      |---------|
| Front-end                      |      | Highscore API           |      | Database| 
|                                | <--> |                         | <--> |         |
| Snake implemented in rust(wasm)|      | Azure Functions with F# |      | MongoDB |
| Deployed as Azure static webapp|      |                         | <-|  |---------|
|--------------------------------|      |-------------------------|   V             
                                                                    |---------------|
  https://www.playsnake.no          https://highscores.playsnake.no | Secrets store |
                                                                    |               |
                                                                    | Azure KeyVault|
                                                                    |---------------|
```

All the cloud infra is Infrastructure-as-Code managed by Terraform.

The highscores API is documented with OpenApi. [Swagger UI](https://highscores.playsnake.no/api/swagger/ui) |
[json](https://highscores.playsnake.no/api/openapi/v3.json) | [yaml](https://highscores.playsnake.no/api/openapi/v3.json).

## Development 🐳

Run all services with docker compose:

```console
# Enable BuildKit if you haven't.
# This is one several ways to do so.
export DOCKER_BUILDKIT=1
export COMPOSE_DOCKER_CLI_BUILD=1

# Run with composev2
docker compose up -d --build

# Or run with "traditional" docker compose
docker-compose up -d --build
```

Then open front-end at [localhost:8080](http://localhost:8080)

To stop all services and remove their containers:

```console
docker compose down
# or
docker-compose down
```

### Required Tools

[docker](https://www.docker.com/) and docker-compose (incuded in Docker Desktop)
