# user_dir_lap

An implementation of a User Directory solution using Leptos, Axum, and PostgreSQL.

Technical capabilities (showcases):

-   Client-side rendering.
-   Server functions.

<br/>

## Getting Started

### Prerequisites

Install the following:

-   To build the Rust code:
    -   [Rust](https://www.rust-lang.org/tools/install)
    -   Cargo Leptos plugin\
        Install it using `cargo install --locked cargo-leptos`
-   To start a PostgreSQL instance locally
    -   As a [Docker](https://www.docker.com/get-docker) container.
    -   Alternatively, you can use an existing PostgreSQL instance and configure the `DATABASE_URL` environment variable (in `.env` file).
-   To build (update) the `main.css` file:
    -   This is needed only if you use additional Tailwind CSS rules.
    -   Otherwise, you can skip this step.
    -   Install it using `npm install tailwindcss@4.1.11 @tailwindcss/cli@4.1.11`

<br/>

## Quick Start

Run the followings:

-   `./dev_css.sh` to compile Tailwind CSS related styles.
-   `./dev_svc.sh` to start the service.
