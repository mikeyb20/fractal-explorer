use web_sys::{HtmlCanvasElement, Window};

/// Size the `<canvas>` backing store to its CSS box times `devicePixelRatio`.
/// Returns the device-pixel (w, h) to use for the render surface.
pub fn canvas_size_px(canvas: &HtmlCanvasElement, window: &Window) -> (u32, u32) {
    let dpr = window.device_pixel_ratio().max(1.0);
    let rect = canvas.get_bounding_client_rect();
    let w = ((rect.width() * dpr).round() as u32).max(1);
    let h = ((rect.height() * dpr).round() as u32).max(1);
    canvas.set_width(w);
    canvas.set_height(h);
    (w, h)
}
