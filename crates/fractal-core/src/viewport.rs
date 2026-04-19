use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub center: (f64, f64),
    pub zoom: f64,
    pub aspect_ratio: f32,
}

impl Viewport {
    pub const MANDELBROT_DEFAULT: Self = Self {
        center: (-0.5, 0.0),
        zoom: 1.5,
        aspect_ratio: 1.0,
    };

    pub fn pixel_to_complex(&self, pixel: (f32, f32), size: (u32, u32)) -> (f64, f64) {
        let nx = (f64::from(pixel.0) / f64::from(size.0)) * 2.0 - 1.0;
        let ny = 1.0 - (f64::from(pixel.1) / f64::from(size.1)) * 2.0;
        let half_w = self.zoom;
        let half_h = self.zoom / f64::from(self.aspect_ratio);
        (self.center.0 + nx * half_w, self.center.1 + ny * half_h)
    }

    pub fn zoom_toward(&mut self, pixel: (f32, f32), size: (u32, u32), factor: f64) {
        let before = self.pixel_to_complex(pixel, size);
        self.zoom *= factor;
        let after = self.pixel_to_complex(pixel, size);
        self.center.0 += before.0 - after.0;
        self.center.1 += before.1 - after.1;
    }

    pub fn pan_by_pixels(&mut self, delta: (f32, f32), size: (u32, u32)) {
        let half_w = self.zoom;
        let half_h = self.zoom / f64::from(self.aspect_ratio);
        let dx = (f64::from(delta.0) / f64::from(size.0)) * 2.0 * half_w;
        let dy = (f64::from(delta.1) / f64::from(size.1)) * 2.0 * half_h;
        self.center.0 -= dx;
        self.center.1 += dy;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Uniforms {
    pub center: [f32; 2],
    pub zoom: f32,
    pub aspect: f32,
    pub iter_cap: u32,
    _pad: [u32; 3],
}

impl Uniforms {
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_viewport(vp: &Viewport, iter_cap: u32) -> Self {
        Self {
            center: [vp.center.0 as f32, vp.center.1 as f32],
            zoom: vp.zoom as f32,
            aspect: vp.aspect_ratio,
            iter_cap,
            _pad: [0; 3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vp() -> Viewport {
        Viewport {
            center: (0.0, 0.0),
            zoom: 1.0,
            aspect_ratio: 1.0,
        }
    }

    #[test]
    fn pixel_center_maps_to_complex_center() {
        let p = vp().pixel_to_complex((50.0, 50.0), (100, 100));
        assert!((p.0).abs() < 1e-9 && (p.1).abs() < 1e-9);
    }

    #[test]
    fn pixel_corners_map_to_plane_corners() {
        let v = vp();
        let tl = v.pixel_to_complex((0.0, 0.0), (100, 100));
        let br = v.pixel_to_complex((100.0, 100.0), (100, 100));
        assert!((tl.0 + 1.0).abs() < 1e-9 && (tl.1 - 1.0).abs() < 1e-9);
        assert!((br.0 - 1.0).abs() < 1e-9 && (br.1 + 1.0).abs() < 1e-9);
    }

    #[test]
    fn zoom_toward_fixed_point_invariant() {
        let mut v = vp();
        let pixel = (25.0, 25.0);
        let size = (100, 100);
        let before = v.pixel_to_complex(pixel, size);
        v.zoom_toward(pixel, size, 0.5);
        let after = v.pixel_to_complex(pixel, size);
        assert!((before.0 - after.0).abs() < 1e-9);
        assert!((before.1 - after.1).abs() < 1e-9);
    }

    #[test]
    fn pan_reverses_with_opposite_delta() {
        let mut v = vp();
        let c0 = v.center;
        v.pan_by_pixels((10.0, -5.0), (100, 100));
        v.pan_by_pixels((-10.0, 5.0), (100, 100));
        assert!((v.center.0 - c0.0).abs() < 1e-9);
        assert!((v.center.1 - c0.1).abs() < 1e-9);
    }
}
