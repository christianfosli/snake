# visnake-wasm

![CI\_CD](https://github.com/christianfosli/visnake-wasm/workflows/CI_CD/badge.svg)
[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square)](https://github.com/prettier/prettier)

Snake with vi/vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

Push to master triggers build and deploy to Azure storage.

Work-in-progress replacement for [visnake with Flask and vanilla js](
https://github.com/christianfosli/visnake).

## Development

**Based on [rust-parcel-template](https://github.com/rustwasm/rust-parcel-template)**

* `npm run start` -- Serve the project locally _with hot reload!_ at `http://localhost:1234`.

* `npm run build` -- Bundle the project (in production mode)

* `cargo test` -- Run rust unit tests

* `wasm-pack test --chrome` -- Run `#[wasm_bindgen_test]` tests in chrome

### Required Tools

* [wasm-pack](https://github.com/rustwasm/wasm-pack)

* [Node](https://nodejs.org/en/)

### Formatting

Prettier is used to format HTML, CSS and JavaScript files.
Rustfmt is used to format Rust files.
