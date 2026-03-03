# Flappy Rust

A complete, lightweight Flappy Bird style game written in Rust with macroquad. The game includes gravity and flap input, moving pipes, collision detection, scoring, game over, and restart, plus modern extras like a pixel-art paper plane, audio cues, collectibles, combo/flow bonuses, and temporary power-ups.

[![Deploy GitHub Pages](https://github.com/<your-username>/<your-repo>/actions/workflows/deploy-pages.yml/badge.svg)](https://github.com/<your-username>/<your-repo>/actions/workflows/deploy-pages.yml)

## Setup

Requirements:
- Rust toolchain (stable)

Run the game:

```bash
cargo run
```

Build WebAssembly (for web):

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

Run tests:

```bash
cargo test
```

Validate build in headless environments:

```bash
cargo check
```

Preview the web build locally:

```bash
mkdir -p dist
cp web/index.html dist/index.html
cp target/wasm32-unknown-unknown/release/flappy_rust.wasm dist/flappy_rust.wasm
python3 -m http.server 8080 -d dist
```

## Controls

- `Space` / `Up Arrow` / Left Mouse Button: Start + flap
- `R`: Restart after game over
- `P`: Pause / resume
- `M`: Mute / unmute audio

## Features

- Pixel-art paper plane keeps a right-facing nose, pitches with vertical velocity, and leaves a trailing wake.
- Retro chiptune BGM (looped) plus runtime flap/score/game-over SFX.
- Pixel-art pipes with segmented caps, shading, and highlights.
- Pause/resume and mute toggles with in-game HUD hints.
- Collectibles: stars (combo scoring), feathers (low gravity), shields (one-hit protection), boost orbs (score multiplier).
- Flow system: perfect gap passes build Flow for bonus points; close calls score extra.
- Modern visuals: sky day/night cycle, parallax ground, clouds, and boost speed lines.

## Open Source

- License: [MIT](./LICENSE)
- CI/CD deploy: [`.github/workflows/deploy-pages.yml`](./.github/workflows/deploy-pages.yml)

## Deploy GitHub Pages

1. Create a GitHub repository and push this project to branch `main`.
2. In repository settings, open `Pages` and ensure source uses `GitHub Actions`.
3. Keep workflow file at `.github/workflows/deploy-pages.yml`.
4. Push to `main` (or run `workflow_dispatch`) to publish.
5. Your site URL will be:
   - `https://<your-username>.github.io/<your-repo>/`

## Architecture Notes

- `src/main.rs`: Window configuration and the main game loop.
- `src/game.rs`: Game state, update loop, rendering, and pipe spawning logic.
- `src/physics.rs`: Pure helper functions (collision detection, clamp) with unit tests.

Gameplay state is centralized in `Game`, keeping the update/draw responsibilities explicit and deterministic. Physics helpers stay isolated for easier testing without rendering.
