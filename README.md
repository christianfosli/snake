# visnake-wasm

![wasm_app](https://github.com/christianfosli/visnake-wasm/workflows/wasm_app/badge.svg)
![highscore_functions](https://github.com/christianfosli/visnake-wasm/workflows/highscore_functions/badge.svg)
[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square)](https://github.com/prettier/prettier)

Snake with vi/vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

## Development

Run all services with docker compose:

```console
docker-compose up -d
```

Then open front-end at [localhost:1234](http://localhost:1234)

To stop all services and remove their containers:

```console
docker-compose down
```

Changing rust files (files in front-end/src) should update automatically,
as this directory is mounted as a volume in the container.
**All other changes require rebuilding the images**

```console
docker-compose build
```

### Front-End App

**Based on [rust-parcel-template](https://github.com/rustwasm/rust-parcel-template)**

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

### Formatting

Prettier is used to format HTML, CSS and JavaScript files.
Rustfmt is used to format Rust files.
