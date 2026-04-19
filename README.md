# Fractal Explorer

Interactive fractal explorer built on a shared Rust core, rendered with WebGPU.
Runs in the browser today (WASM + wgpu); a native desktop build is planned.

This repository is at **v0.1** — the "hello-Mandelbrot" slice. It proves the
full pipeline (Rust → WASM → Trunk → wgpu → WebGPU → `<canvas>`) and gives you
a smooth, shareable Mandelbrot view. Julia previews, palette picker, WebGL2
fallback, touch gestures, and the full desktop app are on the roadmap.

---

## What it can do today (v0.1)

- **Render the Mandelbrot set** in real time on the GPU. A full-screen
  fragment shader iterates `z = z² + c` with smooth (continuous) iteration
  counts and a cosine-based "electric" palette.
- **Drag to pan.** Pointer down + move translates the view; pointer capture
  keeps the drag alive if the cursor leaves the canvas.
- **Scroll to zoom toward the cursor.** The point under the cursor stays
  fixed while the view scales around it.
- **Resize-aware.** A `ResizeObserver` keeps the canvas backing store matched
  to the CSS box at device-pixel resolution (DPR-aware, so HiDPI screens stay
  sharp). Aspect ratio updates live.
- **Shareable URL.** The view state (`fractal id`, `center`, `zoom`,
  `iter_cap`) is written to the page hash, debounced to ~150 ms. Paste the
  URL into a new tab to restore the exact same view.
- **Graceful failure on browsers without WebGPU.** If `navigator.gpu` is
  missing or adapter request fails, a readable overlay replaces the white
  screen and links to `caniuse.com/webgpu`.

### What it does **not** do yet

Julia preview, WebGL2 fallback, tour mode, gallery, palette picker,
screenshot, keyboard shortcuts, pinch / two-finger pan, iteration-cap auto
scaling, deep-zoom precision tricks, desktop UI. See `VISION.md` for the
long-term plan and `docs/plans/` for the next slice.

The `fractal-desktop` crate exists to match the workspace layout but is a
one-line stub (`println!("fractal-desktop coming in v2")`).

---

## Architecture at a glance

```text
crates/fractal-core/       # Shared: math, Fractal trait, WGSL shader assembly
  ├── viewport.rs          # pixel↔complex mapping, pan, zoom-toward-cursor
  ├── fractal.rs           # Fractal trait (id, default view, WGSL snippet)
  ├── fractals/mandelbrot  # The first concrete fractal
  ├── coloring.rs          # DEFAULT_PALETTE_WGSL (cosine gradient)
  └── shader.rs            # build_shader(&dyn Fractal) -> String

crates/fractal-web/        # cdylib+rlib, compiled to WASM
  ├── renderer.rs          # wgpu 0.20 / WebGPU pipeline + uniform buffer
  ├── input.rs             # pointer / wheel / ResizeObserver / rAF loop
  ├── state.rs             # UrlState parse/to_hash + load_initial/write_url
  ├── error_overlay.rs     # DOM overlay for unsupported browsers
  └── util.rs              # DPR-aware canvas sizing helper

crates/fractal-desktop/    # stub binary (v2 target)

web/                       # Trunk-served shell: index.html, style.css
Trunk.toml                 # Trunk config (serves web/index.html)
```

Fractals are **data, not hardcoded**: anything implementing the `Fractal`
trait and exposing a WGSL `iter(c, iter_cap) -> f32` snippet can be rendered
without touching the renderer. Adding a new fractal means a new file in
`fractals/` and one line in `fractals/mod.rs`.

---

## Prerequisites

- **Rust 1.75 or newer** (workspace `rust-version = "1.75"`).
  Install via <https://rustup.rs>.
- **`wasm32-unknown-unknown` target** for the web build:

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- **[Trunk](https://trunkrs.dev)** + `wasm-bindgen-cli` to serve the web app:

  ```bash
  cargo install trunk wasm-bindgen-cli
  ```

- **A WebGPU-enabled browser** to view it:
  - Chrome 113+ or Edge 113+ (WebGPU on by default)
  - Firefox Nightly with `dom.webgpu.enabled` = true
  - Safari Technology Preview

---

## Run it

### Web (the real thing)

From the repo root:

```bash
trunk serve
```

Trunk compiles `fractal-web` to WASM, bundles `web/index.html` + `web/style.css`
into `dist/`, and serves <http://127.0.0.1:8080>. Opening that URL in a
WebGPU browser paints the Mandelbrot set across the viewport.

For a one-off production-style build without a server:

```bash
trunk build --release
# -> dist/ contains the hashed wasm + js + html
```

### Desktop (stub)

```bash
cargo run -p fractal-desktop
# -> prints "fractal-desktop coming in v2"
```

This exists so `cargo build --workspace` matches the architecture in
`VISION.md`. A real desktop UI (wgpu + egui) is a v2 concern.

---

## Using the web app

Once the page loads, the canvas fills the viewport and starts rendering.

| Action | Gesture |
| --- | --- |
| **Pan** | Hold left mouse button and drag. The point under the cursor moves with the mouse. |
| **Zoom in** | Scroll the wheel **up** (or two-finger scroll up on a trackpad). The view zooms ~10% toward the cursor per tick. |
| **Zoom out** | Scroll the wheel **down**. Same ~10% factor, inverted. |
| **Resize** | Resize the browser window. The canvas, backing store, and aspect ratio update live. |
| **Share a view** | Copy the URL from the address bar at any point. The hash (`#f=mandelbrot&cx=…&cy=…&z=…&iter=…`) encodes the exact view. Paste into a new tab to restore it. |

The URL updates automatically as you navigate (debounced to ~150 ms so the
address bar doesn't thrash).

### URL hash format

```text
#f=<fractal-id>&cx=<center-x>&cy=<center-y>&z=<half-width>&iter=<iter-cap>
```

- `f` — fractal id. Currently only `mandelbrot`.
- `cx`, `cy` — center in the complex plane (f64).
- `z` — zoom, expressed as the **half-width** of the view in complex-plane
  units. Smaller = more zoomed in. Default is `1.5` (x-range ≈ `[-2.0, 1.0]`).
- `iter` — iteration cap. Defaults to `256`. Higher values resolve deeper
  detail at zoom but cost more per frame.

Example:
`#f=mandelbrot&cx=-0.7436438885706&cy=0.1318259043124&z=0.0001&iter=1000`

If the hash is absent, malformed, or references a different fractal, the app
falls back to the Mandelbrot default view (`center = (-0.5, 0)`, `z = 1.5`).

### Troubleshooting

- **Black screen, nothing happens.** Almost always means your browser
  doesn't expose `navigator.gpu`. You should see an overlay saying so; if
  not, check DevTools console for a panic trace.
- **Overlay says "WebGPU not available".** Use Chrome/Edge 113+ or enable
  the WebGPU flag in your browser. Safari requires TP; Firefox requires
  Nightly.
- **Blurry output on HiDPI displays.** Shouldn't happen — the canvas is
  sized to `devicePixelRatio`. If it does, reload after any DPR change
  (e.g. dragging the window between monitors).
- **Address bar URL not updating.** Writes are debounced and only fire
  after the first render tick following a change. Tiny nudges under the
  debounce window may not write immediately; any further input flushes it.

---

## Development

### Everyday commands

```bash
# Build everything (native target).
cargo build --workspace

# Run all tests (viewport math + WGSL shader parse + URL roundtrip).
cargo test --workspace

# Lint with pedantic clippy, warnings as errors.
cargo clippy --workspace --all-targets -- -D warnings

# Check the wasm build. Does not produce runnable output by itself —
# use `trunk serve` for that — but catches wasm-specific breakage fast.
cargo check -p fractal-web --target wasm32-unknown-unknown
```

A `scripts/validate-turn.sh` hook runs `cargo build --workspace` and
`cargo test --workspace` automatically at the end of each Claude Code turn,
so the workspace never drifts into a broken state during agent-driven work.

### Tests today

- `fractal-core::viewport::tests` — pixel-to-complex mapping, zoom-invariant
  fixed-point, pan reversibility.
- `fractal-core::shader::tests::mandelbrot_shader_parses` — round-trips the
  generated WGSL through `naga` to catch shader-source breakage at test time
  rather than at page load.
- `fractal-web::state::tests::roundtrip` — URL hash encode/parse symmetry.

### Adding a new fractal

1. Create `crates/fractal-core/src/fractals/<name>.rs` and implement the
   `Fractal` trait. Provide a WGSL snippet exposing
   `fn iter(c: vec2<f32>, iter_cap: u32) -> f32`.
2. Declare the module in `crates/fractal-core/src/fractals/mod.rs`.
3. Add a naga-parse test mirroring the Mandelbrot one in `shader.rs`.
4. Eventually, register it in a fractal registry (not yet wired up in v0.1 —
   the renderer hardcodes `Mandelbrot`). This will land as part of v0.2.

### Coding conventions

- Conventional commits (`feat:`, `fix:`, `refactor:`, `chore:`, `docs:`).
- Error handling: `Result<_, _>` at crate boundaries; `panic!` only for
  real invariants. No ad-hoc `unwrap()` in library code without a comment.
- Shader source is **generated**, not copy-pasted. One WGSL snippet per
  fractal, assembled by `fractal-core::build_shader`.
- `fractal-web` gates WebGPU-only code with `#[cfg(target_arch = "wasm32")]`
  so `cargo build --workspace` stays green natively.

### Project docs

- `VISION.md` — long-term design document (fractals, platforms, principles).
- `docs/plans/` — per-slice implementation plans (start with
  `v0.1-hello-mandelbrot.md`).
- `docs/decisions/` — architectural decision records.
- `docs/WORKFLOW.md` — how Claude-assisted development is structured here.
- `docs/SKILL-MAP.md` — which skills/commands cover which parts of the cycle.

---

## Roadmap (abbreviated)

| Version | Focus |
| --- | --- |
| v0.1 (now) | Mandelbrot on WebGPU; pan, zoom, URL sharing. |
| v0.2 | Palette registry + picker, keyboard shortcuts, iteration-cap auto-scale. |
| v0.3 | Julia preview linked to Mandelbrot cursor. |
| v0.4 | Touch gestures (pinch, two-finger pan, double-tap); mobile layout. |
| v0.5 | Tour mode + gallery. |
| v0.6 | WebGL2/GLSL fallback renderer for non-WebGPU browsers. |
| v1.0 | Perf validation on mid-range Android, PWA basics, polish. |
| v2.0 | Native desktop (wgpu + egui), deep-zoom precision. |

Full detail in `VISION.md`.

---

## License

Dual-licensed under **MIT OR Apache-2.0**, at your option. See the crate
manifests for the SPDX identifier.
