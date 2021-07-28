# snake

Snake with vi/vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

![wasm_app](https://github.com/christianfosli/visnake-wasm/workflows/wasm_app/badge.svg)
![highscore_functions](https://github.com/christianfosli/visnake-wasm/workflows/highscore_functions/badge.svg)
![terraform](https://github.com/christianfosli/snake/actions/workflows/terraform.yml/badge.svg)

## Architecture 🏗

The system consists of 3 parts

 * Front-end application where snake is implemented with rust/webassembly

 * Azure functions for setting and fetching highscores from a database

 * MongoDB database

 **TODO: Update the diagram after switching DB provider**
 ![architecture diagram](./architecture.svg)

We use a serverless approach, where the majority of the code is front-end.

## Development 🐳

Run all services with docker compose:

```console
# Enable BuildKit if you haven't. This is one several ways to do so.
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
```

### Required Tools

[docker](https://www.docker.com/) and docker-compose (incuded in Docker Desktop)
