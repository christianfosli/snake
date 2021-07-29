# snake

Snake with vi/vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

![wasm_app](https://github.com/christianfosli/visnake-wasm/workflows/wasm_app/badge.svg)
![highscore_functions](https://github.com/christianfosli/visnake-wasm/workflows/highscore_functions/badge.svg)
![terraform](https://github.com/christianfosli/snake/actions/workflows/terraform.yml/badge.svg)

## Architecture 🏗

* [Front-end application](https://www.playsnake.no) where snake is implemented with rust/webassembly.

  * Deployed as a Azure Static WebApp. When running locally with docker nginx is used.

* [API for working with highscores](https://highscores.playsnake.no), implemented as Azure functions.

* MongoDB database for persistence.

* Azure KeyVault for safely providing function app with MongoDB connection string
  (but when running locally we just use environment variables instead).

All the cloud infra is Infrastructure-as-Code managed by Terraform.

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
