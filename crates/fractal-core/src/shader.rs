use crate::coloring::DEFAULT_PALETTE_WGSL;
use crate::fractal::Fractal;

const TEMPLATE: &str = r"
struct Uniforms {
    center: vec2<f32>,
    zoom: f32,
    aspect: f32,
    iter_cap: u32,
    _pad0: u32, _pad1: u32, _pad2: u32,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VsOut {
    let x = f32((idx << 1u) & 2u);
    let y = f32(idx & 2u);
    var out: VsOut;
    out.pos = vec4<f32>(x * 2.0 - 1.0, y * 2.0 - 1.0, 0.0, 1.0);
    out.uv = vec2<f32>(x, y);
    return out;
}

__ITER_SNIPPET__

__PALETTE_SNIPPET__

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let nx = in.uv.x * 2.0 - 1.0;
    let ny = 1.0 - in.uv.y * 2.0;
    let half_w = u.zoom;
    let half_h = u.zoom / u.aspect;
    let c = vec2<f32>(u.center.x + nx * half_w, u.center.y + ny * half_h);
    let t = iter(c, u.iter_cap);
    if (t < 0.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    return vec4<f32>(palette(t / f32(u.iter_cap)), 1.0);
}
";

pub fn build_shader(fractal: &dyn Fractal) -> String {
    TEMPLATE
        .replace("__ITER_SNIPPET__", fractal.wgsl_iter_snippet())
        .replace("__PALETTE_SNIPPET__", DEFAULT_PALETTE_WGSL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fractals::mandelbrot::Mandelbrot;

    #[test]
    fn mandelbrot_shader_parses() {
        let src = build_shader(&Mandelbrot);
        let module = naga::front::wgsl::parse_str(&src)
            .unwrap_or_else(|e| panic!("WGSL parse error: {e}\n---\n{src}"));
        let names: Vec<_> = module
            .entry_points
            .iter()
            .map(|e| e.name.as_str())
            .collect();
        assert!(names.contains(&"vs_main"));
        assert!(names.contains(&"fs_main"));
    }
}
