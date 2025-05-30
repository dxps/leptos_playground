# A counter sample w/ backend awareness using Leptos & Axum

This sample was created using [start-axum-workspace](https://github.com/leptos-rs/start-axum-workspace) template.

Besides the classic counter feature that can be increased within the page (using \_Increase` button),
the counter can be sent to the server (using _Send it to the server` button).

## Running your project

```bash
cargo leptos watch --hot-reload
```

<br/>

---

# Project generation notes

This section contains what the template contains in its readme.

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future

## Compiling for Release

```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project

Cargo-leptos uses (https://playwright.dev)[Playwright] as the end-to-end test tool.

Prior to the first run of the end-to-end tests run Playwright must be installed.
In the project's `end2end` directory run `npm install -D playwright @playwright/test` to install playwright and browser specific APIs.

To run the tests during development in the project root run:

```bash
cargo leptos end-to-end
```

To run tests for release in the project root run:

```bash
cargo leptos end-to-end --release
```

There are some examples tests are located in `end2end/tests` directory that pass tests with the sample Leptos app.

A web-based report on tests is available by running `npx playwright show-report` in the `end2end` directory.

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
start-axum-workspace
site/
```

Set the following environment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="start-axum-workspace"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
