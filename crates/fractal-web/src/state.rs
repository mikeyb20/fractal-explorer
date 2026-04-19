use fractal_core::{Fractal, Viewport};

pub struct UrlState {
    pub fractal_id: String,
    pub center: (f64, f64),
    pub zoom: f64,
    pub iter_cap: u32,
}

impl UrlState {
    pub fn parse(hash: &str) -> Option<Self> {
        let s = hash.strip_prefix('#').unwrap_or(hash);
        let mut f = None;
        let mut cx = None;
        let mut cy = None;
        let mut z = None;
        let mut iter = None;
        for kv in s.split('&') {
            if let Some((k, v)) = kv.split_once('=') {
                match k {
                    "f" => f = Some(v.to_string()),
                    "cx" => cx = v.parse().ok(),
                    "cy" => cy = v.parse().ok(),
                    "z" => z = v.parse().ok(),
                    "iter" => iter = v.parse().ok(),
                    _ => {}
                }
            }
        }
        Some(Self {
            fractal_id: f?,
            center: (cx?, cy?),
            zoom: z?,
            iter_cap: iter?,
        })
    }

    pub fn to_hash(&self) -> String {
        format!(
            "#f={}&cx={}&cy={}&z={}&iter={}",
            self.fractal_id, self.center.0, self.center.1, self.zoom, self.iter_cap
        )
    }
}

pub fn load_initial(window: &web_sys::Window, fractal: &dyn Fractal) -> (Viewport, u32) {
    let hash = window.location().hash().unwrap_or_default();
    if let Some(u) = UrlState::parse(&hash) {
        if u.fractal_id == fractal.id() {
            let mut vp = fractal.default_view();
            vp.center = u.center;
            vp.zoom = u.zoom;
            return (vp, u.iter_cap);
        }
    }
    (fractal.default_view(), fractal.default_iter_cap())
}

pub fn write_url(window: &web_sys::Window, fractal_id: &str, vp: &Viewport, iter_cap: u32) {
    let h = UrlState {
        fractal_id: fractal_id.to_string(),
        center: vp.center,
        zoom: vp.zoom,
        iter_cap,
    }
    .to_hash();
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&h));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn roundtrip() {
        let s = UrlState {
            fractal_id: "mandelbrot".into(),
            center: (-0.5, 0.0),
            zoom: 1.5,
            iter_cap: 256,
        };
        let parsed = UrlState::parse(&s.to_hash()).unwrap();
        assert_eq!(parsed.fractal_id, s.fractal_id);
        assert_eq!(parsed.center, s.center);
        assert_eq!(parsed.zoom, s.zoom);
        assert_eq!(parsed.iter_cap, s.iter_cap);
    }
}
