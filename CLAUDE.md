# Fractal Explorer

## Architecture

Interactive fractal explorer with a web-first, desktop-second approach. A shared Rust core (`fractal-core`) drives the math, coloring, and WGSL shader generation. Web build (`fractal-web`) compiles to WASM and renders via WebGPU with a WebGL2 fallback. Native desktop build (`fractal-desktop`) uses wgpu + egui. Fractals are registered as data (trait + metadata), not hardcoded — adding one means implementing the trait and registering it.

## Stack

- Language: Rust (edition 2021, MSRV 1.75), plus WGSL shaders and a thin JS/CSS layer for the web shell.
- Build: `cargo build` (workspace). Web: `wasm-pack` or `trunk` (TBD). Desktop: `cargo build -p fractal-desktop --release`.
- Test: `cargo test` (workspace). Golden-image tests come later under `crates/fractal-core/tests/`.
- Lint: `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check`.
- Run: Desktop — `cargo run -p fractal-desktop`. Web — `trunk serve` (once set up).

## Conventions

- Module layout follows the VISION architecture sketch (`fractal-core` owns math; platform crates own I/O and windowing).
- Fractals implement a `Fractal` trait; never hardcode a fractal into the renderer.
- Shader source is generated, not copy-pasted — one WGSL definition per fractal, ported to GLSL where WebGL2 fallback is required.
- Error handling: `Result<_, _>` at crate boundaries, `panic!` only for genuine invariants. No `unwrap()` in library code without a justifying comment.
- Commit style: conventional commits (`feat:`, `fix:`, `refactor:`, `chore:`, `docs:`).
- License: MIT OR Apache-2.0 dual.

## Gotchas

- (populate as we hit them)

## Key Docs

- Vision: see VISION.md
- Workflow philosophy: see docs/WORKFLOW.md
- Skill reference: see docs/SKILL-MAP.md
- Plans: see docs/plans/
- Decisions: see docs/decisions/
