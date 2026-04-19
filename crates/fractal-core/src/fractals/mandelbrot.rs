use crate::fractal::Fractal;
use crate::viewport::Viewport;

pub struct Mandelbrot;

const MANDELBROT_WGSL: &str = r"
fn iter(c: vec2<f32>, iter_cap: u32) -> f32 {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let bailout_sq: f32 = 256.0;
    loop {
        if (i >= iter_cap) { break; }
        let zx = z.x * z.x - z.y * z.y + c.x;
        let zy = 2.0 * z.x * z.y + c.y;
        z = vec2<f32>(zx, zy);
        let mag2 = z.x * z.x + z.y * z.y;
        if (mag2 > bailout_sq) {
            let log_zn = log(mag2) * 0.5;
            let nu = log(log_zn / 0.693147) / 0.693147;
            return f32(i) + 1.0 - nu;
        }
        i = i + 1u;
    }
    return -1.0;
}
";

impl Fractal for Mandelbrot {
    fn id(&self) -> &'static str {
        "mandelbrot"
    }
    fn display_name(&self) -> &'static str {
        "Mandelbrot"
    }
    fn default_view(&self) -> Viewport {
        Viewport::MANDELBROT_DEFAULT
    }
    fn default_iter_cap(&self) -> u32 {
        256
    }
    fn wgsl_iter_snippet(&self) -> &'static str {
        MANDELBROT_WGSL
    }
}
