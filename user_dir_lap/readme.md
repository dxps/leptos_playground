# user_dir_lap

An implementation of a User Directory solution using Leptos, Axum, and PostgreSQL.

Technical capabilities (showcases):

-   Client-side rendering.
-   Server functions.

<br/>

## Getting Started

### Prerequisites

Execute the following:

-   `npm i` to install the Node.js based dependencies: in this case the Tailwind CSS CLI.

-   To build the Rust code:
    -   [Rust](https://www.rust-lang.org/tools/install)
    -   Cargo Leptos plugin\
        Install it using `cargo install --locked cargo-leptos`
-   To start a PostgreSQL instance locally
    -   As a [Docker](https://www.docker.com/get-docker) container.
    -   Alternatively, you can use an existing PostgreSQL instance and configure the `DATABASE_URL` environment variable (in `.env` file).

<br/>

## Quick Start

Run `./dev_svc.sh` to start the service.
