use crate::viewport::Viewport;

pub trait Fractal {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn default_view(&self) -> Viewport;
    fn default_iter_cap(&self) -> u32;

    /// WGSL snippet exposing:
    ///   `fn iter(c: vec2<f32>, iter_cap: u32) -> f32`
    /// Returns smooth iteration count in `[0, iter_cap]` for escaping points,
    /// or a negative sentinel (e.g. -1.0) for points inside the set.
    fn wgsl_iter_snippet(&self) -> &'static str;
}
