//! Shared fractal math, coloring, and shader generation.
//!
//! This crate is the single source of truth for fractal definitions and
//! rendering math. It is consumed by `fractal-web` (compiled to WASM) and
//! `fractal-desktop` (native).

#[cfg(test)]
mod tests {
    #[test]
    fn crate_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
