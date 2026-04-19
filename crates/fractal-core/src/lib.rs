//! Shared fractal math, coloring, and shader generation.
//!
//! This crate is the single source of truth for fractal definitions and
//! rendering math. It is consumed by `fractal-web` (compiled to WASM) and
//! `fractal-desktop` (native).

pub mod coloring;
pub mod fractal;
pub mod fractals;
pub mod shader;
pub mod viewport;

pub use fractal::Fractal;
pub use fractals::mandelbrot::Mandelbrot;
pub use shader::build_shader;
pub use viewport::Viewport;
