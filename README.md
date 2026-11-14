# SpaceOooops

Rust + Leptos (client-side rendered) browser game, built with Trunk.

## Prerequisites

    rustup target add wasm32-unknown-unknown
    cargo install trunk --locked

## Run locally

    trunk serve        # http://127.0.0.1:8080/

## Test

    cargo test         # runs natively, no browser needed (tests the `sim` module)

## Build and deploy

    trunk build --release --public-url /SpaceOooops/

Pushing to `main` runs `.github/workflows/deploy.yml`: `cargo test`, the release build, then a deploy to GitHub Pages
(https://kulunchick.github.io/SpaceOooops/). In the repo settings, set Pages source to "GitHub Actions".

## Layout

- `src/sim/` pure-Rust simulation, no web/UI dependencies (library crate `spaceooops`)
- `src/ui/` Leptos UI, one file per screen (binary `spaceooops-app`)
- `style/main.css` dark mission-control base styles and colour tokens
