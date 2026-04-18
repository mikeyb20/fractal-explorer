# Fractal Explorer — Vision

A living design document. Nothing here is final; we edit as we learn.

## One-line pitch

An interactive fractal explorer that runs in the browser and as a faster native desktop app, sharing a Rust core and GPU shaders between both.

## Guiding principles

- **Start simple.** Ship Mandelbrot + Julia first. Everything else is "later."
- **Scale up, don't rewrite.** Architecture should let us add fractals, renderers, and platforms without gutting the core.
- **Shared core.** The math (iteration, coloring, coordinate transforms) lives in one Rust crate, consumed by both web (via WASM) and desktop.
- **Interactive first, then pretty.** Smooth pan/zoom beats beautiful stills. Quality modes come after the feel is right.
- **Mobile is a first-class target.** Touch controls, responsive layout, and perf budgets included from day one — not bolted on later.

## Platforms

### v1 — Web (browser)

- Rust → WASM for core logic.
- Dual GPU backend: WebGPU (preferred, modern browsers) with a WebGL2/GLSL fallback for wider reach. Shader source generated from a single WGSL definition where possible, hand-ported to GLSL where not.
- Shareable URL state: fractal id, center, zoom, palette, iteration cap.
- Runs on desktop and mobile browsers. PWA / offline support is deferred past v1.

### v2 — Native desktop

- Rust + wgpu, same shaders as web (WGSL is portable).
- UI shell: `egui` — fast iteration, Rust-native, integrates directly with wgpu.
- Enables true f64 precision on GPU via split-double emulation, multi-threaded CPU fallback, and frame-accumulated supersampling.
- Deep zoom beyond f32 limits.

### Shared across both

- Same `fractal-core` crate drives math, coloring, viewport, and shader source generation.
- Platform code is thin: input handling, windowing, URL/file I/O, export.

## Fractals — roadmap

### Ship first (v1 fractals)

- **Mandelbrot** — escape-time, smooth (continuous) coloring.
- **Julia** — linked to Mandelbrot cursor. Hovering Mandelbrot previews the corresponding Julia set in a side panel; clicking "lock" switches the main view.

### Next (v2 fractals)

- Burning Ship
- Newton's method basins (select polynomial)
- Multibrot (`z^n + c`, n adjustable)
- Tricorn / Mandelbar

### Fractals — later / stretch

- IFS & L-systems (Barnsley fern, Sierpinski, dragon curve, Lindenmayer trees) — different renderer (point cloud / line rasterization, not per-pixel escape-time).
- 3D: Mandelbulb, Mandelbox (ray-marched) — very different renderer.
- Deep zoom with perturbation theory + reference orbits (arbitrary precision on CPU, delta orbits on GPU).

## Core features

### v1

- Smooth pan (mouse drag, touch drag) and zoom (wheel, pinch).
- Double-tap / double-click to zoom toward cursor; two-finger rotate optional.
- Iteration count auto-adjusts with zoom depth (configurable cap).
- 4–6 built-in palettes with smooth coloring.
- URL state sync (copy link = share exact view).
- **Tour mode** — on first load, a guided zoom through 4–6 curated Mandelbrot locations (Seahorse Valley, Elephant Valley, Mini-Mandelbrot, spiral, dendrite, cusp). Each stop shows only its name; no narration in v1. Skippable. Serves as both onboarding and a showcase.
- **Gallery** — a persistent menu of curated locations (tour stops + extras), browsable at any time. Each entry is a preset with full URL state; users can jump to one directly or use it as a starting point to explore.
- **Keyboard shortcuts** — arrows pan, `+`/`-` zoom, `r` reset, `s` screenshot, `h` hide UI, `esc` exit tour, `g` open gallery.
- Responsive layout: on narrow screens the Julia preview collapses into a toggle.
- Pause/resume rendering when tab is hidden.

### v2

- Palette editor (custom stops, cycling animation, import/export palette JSON).
- Orbit traps.
- Distance-estimation shading for crisp edges.
- Screenshot export (PNG, user-chosen resolution, tiled render for sizes beyond GPU texture limits).
- Bookmarks (user-saved locations in local storage, export/import as JSON).
- PWA / offline support.

### Stretch

- Keyframe-based zoom video export (MP4 via ffmpeg.wasm on web, ffmpeg CLI on desktop).
- Side-by-side comparison view (two fractals, synced navigation).
- 3D lighting effects on 2D fractals (slope shading).

## Mobile / touch

Treat as primary, not fallback.

- **Gestures:** single-finger drag = pan, pinch = zoom, two-finger drag = pan during pinch, double-tap = zoom in toward tap, two-finger tap = zoom out.
- **Layout:** palette + controls live in a bottom sheet (thumb-reachable). Julia preview is a toggleable overlay, not a side-by-side panel.
- **Perf budget:** target 30 fps on a ~2021 mid-range Android (Snapdragon 7-series class). Auto-drop render resolution (1/2 or 1/4) during active gesture, upscale to full on gesture end.
- **Battery:** cap iteration count more aggressively when `navigator.getBattery()` reports low, or when the device is thermal-throttling (detect via frame-time heuristic).

## Tour mode — details

Goal: within 30 seconds of page load, the user has seen something beautiful without touching anything.

- Auto-starts on first visit; subsequent visits open where the user left off.
- Sequence: 4–6 short segments (~5 s each), each animating from the previous view to a named location with an eased zoom + pan.
- Overlay text shows the location name only (e.g. "Seahorse Valley"). No descriptive narration in v1 — keep it clean; narration can be added later without changing the data format.
- "Skip tour" and "Explore" buttons visible throughout.
- Interacting (drag, zoom, tap) immediately exits the tour and hands control to the user.
- Each tour stop is a preset with full URL state, so users can share or revisit individual locations. These same stops seed the gallery.
- Tour sequence is data (JSON), not code — easy to curate and extend later.

## Architecture sketch

```text
┌─────────────────────────────────────────────────────┐
│ fractal-core/ (Rust crate, no_std-friendly)         │
│   - fractal registry (trait + implementations)      │
│   - coloring (palette sampling, smooth iter count)  │
│   - coordinate / viewport math                      │
│   - shader source generation (WGSL)                 │
│   - tour preset data                                │
├──────────────────────────┬──────────────────────────┤
│ fractal-web/             │ fractal-desktop/         │
│   - wasm-bindgen shim    │   - winit + wgpu         │
│   - WebGPU + WebGL2      │   - egui UI              │
│     renderers            │   - file I/O, export     │
│   - DOM/CSS UI           │   - native file dialogs  │
│   - URL state sync       │   - (optional) CPU       │
│   - touch event handling │     fallback for deep    │
│                          │     zoom                 │
└──────────────────────────┴──────────────────────────┘
```

Fractals are **data, not hardcoded UI**. Adding a new one means:

1. Implement a `Fractal` trait (name, parameter schema, WGSL snippet, default view).
2. Register it in the fractal registry.

The renderer and UI pick it up automatically. No switch-statements to update.

## Precision strategy

- **v1:** f32 on GPU. Good to roughly 1e-5 zoom depth before pixelation.
- **v2 (desktop):** split-double (emulated f64 in shader via two f32s). Good to ~1e-14.
- **Stretch:** perturbation theory with arbitrary-precision reference orbit on CPU, delta orbits on GPU. Good to thousands of digits.

Commit to a precision ceiling per release so we don't half-finish the next tier.

## Repo layout (proposed)

```text
fractal-explorer/
├── Cargo.toml           (workspace)
├── crates/
│   ├── fractal-core/    (shared math + shader gen)
│   ├── fractal-web/     (wasm + WebGPU/WebGL2)
│   └── fractal-desktop/ (wgpu + egui)
├── web/                 (index.html, CSS, JS glue, served in dev)
├── assets/
│   ├── palettes/
│   └── tour.json
└── VISION.md
```

## Testing & quality

- **Unit tests** in `fractal-core` for coordinate math, palette sampling, iteration counting edge cases.
- **Golden-image tests** — render known views at known sizes, compare to committed PNGs with a small tolerance. Catches regressions in shader math and coloring.
- **Perf smoke tests** — render N frames at target resolution, assert under a frame-time budget.
- **Manual QA checklist** — mobile pinch, URL round-trip, tour skip, palette switch, extreme zoom.

## License

Dual-licensed **MIT OR Apache-2.0** — the Rust ecosystem standard. `LICENSE-MIT` and `LICENSE-APACHE` at repo root.

## Non-goals (for now)

- Not a general-purpose generative art tool.
- Not trying to beat existing deep-zoom champions (Kalles Fraktaler, Ultra Fractal) on raw depth.
- No user accounts, no cloud storage, no social features.
- No real-time multi-user collaboration.

## Decisions log

Locked choices and the reasoning, so future-us remembers why.

- **Rust everywhere** — shared core, compiles to WASM for web and native for desktop. Avoids a second language.
- **wgpu + WGSL** — one GPU stack for web (via WebGPU) and native. Portable shaders.
- **Dual web GPU backend** — WebGPU preferred, WebGL2/GLSL fallback for reach. Accepted cost: some shader porting.
- **`egui` for desktop UI** — fast iteration, integrates with wgpu, Rust-native. Rejected Tauri-wraps-web (slower, webview quirks).
- **MIT OR Apache-2.0 dual license** — Rust ecosystem norm.
- **Named-only tour stops in v1** — no narration. Easy to add later without changing data format.
- **PWA/offline deferred** — v2 feature. Ship online first.
- **Gallery in v1** — persistent curated-locations menu alongside the tour; seeds from tour data.
- **Keyboard defaults** — arrows/±/r/s/h/esc/g (see v1 features).
