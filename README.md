# visnake-wasm

![CI\_CD](https://github.com/christianfosli/visnake-wasm/workflows/CI_CD/badge.svg)

Snake with vi/vim navigation.
Visit [playsnake.no](https://www.playsnake.no) to play!

Push to master triggers build and deploy to Azure storage.

Work-in-progress replacement for [visnake with Flask and vanilla js](
https://github.com/christianfosli/visnake),
but using Rust and WebAssembly.
I also plan to add highscore functionality through an Azure Function.

---

## Based on [rust-parcel-template](https://github.com/rustwasm/rust-parcel-template)

* `npm run start` -- Serve the project locally _with hot reload!_ at `http://localhost:1234`.

* `npm run build` -- Bundle the project (in production mode)

* `cargo test` -- Run rust unit tests

* `wasm-pack test --chrome` -- Run `#[wasm_bindgen_test]` tests in chrome
