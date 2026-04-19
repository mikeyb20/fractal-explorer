//! Web (WASM) front-end for fractal-explorer.
//!
//! The WebGPU renderer and DOM wiring are compiled only for `wasm32-*` targets.
//! `state` is portable so `UrlState` roundtrip tests can run natively.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::default_trait_access,
    clippy::too_many_lines,
    clippy::needless_pass_by_value,
    clippy::type_complexity
)]

pub mod state;

#[cfg(target_arch = "wasm32")]
mod error_overlay;
#[cfg(target_arch = "wasm32")]
mod input;
#[cfg(target_arch = "wasm32")]
mod renderer;
#[cfg(target_arch = "wasm32")]
mod util;

#[cfg(target_arch = "wasm32")]
mod wasm_entry {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    use crate::{error_overlay, input, renderer, state, util};

    #[wasm_bindgen(start)]
    pub fn start() {
        console_error_panic_hook::set_once();
        let _ = console_log::init_with_level(log::Level::Info);
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = run().await {
                log::error!("fatal: {e:?}");
                error_overlay::show(&format!("{e}"));
            }
        });
    }

    async fn run() -> Result<(), AppError> {
        let window = web_sys::window().ok_or(AppError::NoWindow)?;
        let document = window.document().ok_or(AppError::NoDocument)?;
        let canvas = document
            .get_element_by_id("fractal")
            .ok_or(AppError::NoCanvas)?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| AppError::NoCanvas)?;

        let fractal = fractal_core::Mandelbrot;
        let (mut viewport, iter_cap) = state::load_initial(&window, &fractal);

        let (width, height) = util::canvas_size_px(&canvas, &window);
        viewport.aspect_ratio = width as f32 / height as f32;

        let renderer = renderer::Renderer::new(&canvas, width, height, &fractal).await?;
        input::install(&canvas, &window, viewport, iter_cap, renderer, fractal);
        Ok(())
    }

    #[derive(thiserror::Error, Debug)]
    pub(crate) enum AppError {
        #[error("window unavailable")]
        NoWindow,
        #[error("document unavailable")]
        NoDocument,
        #[error("canvas element #fractal not found")]
        NoCanvas,
        #[error("WebGPU not available in this browser")]
        NoWebGpu,
        #[error("no suitable GPU adapter")]
        NoAdapter,
        #[error("device request failed: {0}")]
        Device(String),
        #[error("surface creation failed: {0}")]
        Surface(String),
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) use wasm_entry::AppError;
