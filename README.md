# snake

Snake with vi/vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

![wasm_app](https://github.com/christianfosli/visnake-wasm/workflows/wasm_app/badge.svg)
![highscore_functions](https://github.com/christianfosli/visnake-wasm/workflows/highscore_functions/badge.svg)
[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square)](https://github.com/prettier/prettier)

## Architecture 🏗

The system consists of 3 parts, which are all defined in this repository.

 * Front-end application where snake is implemented with rust/webassembly

 * Azure functions for setting and fetching highscores from a SQL Server database

 * MS SQL Server database

 ![architecture diagram](./architecture.svg)

We use a server-less approach, where the majority of the code is front-end.

## Development 🐳

Run all services with docker compose:

```console
docker-compose up -d --build
```

Then open front-end at [localhost:8080](http://localhost:8080)

To stop all services and remove their containers:

```console
docker-compose down
```

### Required Tools

**[docker](https://www.docker.com/) and docker-compose (incuded in Docker Desktop)**

Or

* [azure-functions-core-tools](https://github.com/Azure/azure-functions-core-tools)

* [dotnet core 3.1](https://github.com/dotnet/core)

* [node](https://nodejs.org/en/)

* [rust](http://rustlang.org/)

* [wasm-pack](https://github.com/rustwasm/wasm-pack)

* A SQL Server database with schema matching database/schema.sql,
  and the database connection string exposed to the function apps as `CONNECTION_STRING` env variable

### Formatting

Prettier is used to format HTML, CSS and JavaScript files.
Rustfmt is used to format Rust files.
