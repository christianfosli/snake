# snake

Snake with vi/vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

![wasm_app](https://github.com/christianfosli/visnake-wasm/workflows/wasm_app/badge.svg)
![highscore_functions](https://github.com/christianfosli/visnake-wasm/workflows/highscore_functions/badge.svg)
[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square)](https://github.com/prettier/prettier)

## Architecture 🏗

The system consists of 3 parts, which are all defined in this repository:

 * Front-end application where snake is implemented with rust/webassembly

 * Azure functions for setting and fetching highscores from a SQL Server database

 * MS SQL Server database

 ![architecture diagram](./architecture.svg)

## Development 🐳

Run all services with docker compose:

```console
docker-compose up -d
```

Then open front-end at [localhost:1234](http://localhost:1234)

To stop all services and remove their containers:

```console
docker-compose down
```

Changing files in front-end/src, front-end/style.css and front-end/index.html should update automatically,
as these are mounted as volumes in the container.
**All other changes require rebuilding the images**

```console
docker-compose build
```

---

**The front-end is based on [rust-parcel-template](https://github.com/rustwasm/rust-parcel-template)**

### More useful commands for front-end:

* `npm run build` -- Bundle the project (in production mode)

* `cargo test` -- Run rust unit tests

* `wasm-pack test --chrome` -- Run `#[wasm_bindgen_test]` tests in chrome

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
