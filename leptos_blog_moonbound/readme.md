## leptos_blog_moonbound

This project is to evaluate the developer experience using Leptos.

<br/>

---

### Generated Project Notes

This project was created using `cargo leptos new --git leptos-rs/start-axum`.

The starting point in the code is `src/app.rs`.

Addtionally, `Cargo.toml` may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

#### Prerequisites

-   Leptos plugin for cargo. Install it using `cargo install leptos`
-   Install WASM target using `rustup target add wasm32-unknown-unknown`
-   `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
-   `npm install -g sass` - install `dart-sass` (should be optional in future)

#### Running & Building the Project

-   Use `cargo leptos watch` to run the project in "dev mode".
-   Use `cargo leptos build --release` to compile the release.
    -   It will generate your server binary in `target/server/release` and your site package in `target/site`.

#### Testing the Project

```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in `end2end/tests` directory.

#### Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
leptos_blog_moonbound
site/
```

Set the following environment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="leptos_blog_moonbound"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

#### Database

This project uses SQLite database and it interacts with it usind the popular [sqlx](https://github.com/launchbadge/sqlx) library.
And using its CLI (installable using `cargo install sqlx-cli`), the initial setup was done as using:

```shell
> export DATABASE_URL="sqlite:post.db"
> sqlx database create   # To create the database file (named post.db).
> sqlx migrate add post  # To create the migration (sql file) for `post` table.
```

Then that generated file (named `migrations/20240318181214_post.sql` in this case) was filled in with the CREATE TABLE statement.
