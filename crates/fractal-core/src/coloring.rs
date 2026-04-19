/// "Electric" palette — smooth cosine-based gradient, `t` in [0,1].
pub const DEFAULT_PALETTE_WGSL: &str = r"
fn palette(t: f32) -> vec3<f32> {
    let tt = sqrt(clamp(t, 0.0, 1.0));
    let tau = 6.2831853;
    return vec3<f32>(
        0.5 + 0.5 * cos(tau * (tt + 0.00)),
        0.5 + 0.5 * cos(tau * (tt + 0.33)),
        0.5 + 0.5 * cos(tau * (tt + 0.67))
    );
}
";
