# 01: Skeleton and deploy

**What to build:** A visitor opens the public GitHub Pages URL and sees the SpaceOooops title screen. The Rust project is a Leptos (client-side) app built with Trunk. It has a separate pure-Rust `sim` module with no web dependencies, which already passes one `cargo test`. Every push to the default branch builds the app and redeploys it. See ADR-0001 and ADR-0002.

Repo: https://github.com/Kulunchick/SpaceOooops. The expected Pages URL is https://kulunchick.github.io/SpaceOooops/, so the build must use that sub-path as its public URL.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [ ] The app builds locally with Trunk and serves a title screen: "SpaceOooops", a one-line tagline, and a "Start" button that leads to a placeholder screen
- [ ] `sim` is a separate module with no web or UI dependencies; `cargo test` runs natively and passes at least one test through its public interface
- [ ] A GitHub Actions workflow builds the wasm app and deploys it to GitHub Pages on push to the default branch
- [ ] Assets and wasm load correctly under the repo sub-path, with no 404s for the JS, wasm or CSS
- [ ] The README explains how to run locally, test and deploy
- [ ] A dark mission-control base stylesheet exists (background, fonts, colour tokens) for later tickets to build on
