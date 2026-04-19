pub fn show(message: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(doc) = window.document() else {
        return;
    };
    let el = doc.get_element_by_id("error-overlay").or_else(|| {
        let el = doc.create_element("div").ok()?;
        el.set_id("error-overlay");
        doc.body()?.append_child(&el).ok()?;
        Some(el)
    });
    if let Some(el) = el {
        el.set_inner_html(&format!(
            "<div style=\"max-width:36rem;\">\
               <h1>Can't load Fractal Explorer</h1>\
               <p>{}</p>\
               <p>This build requires <a href=\"https://caniuse.com/webgpu\" style=\"color:#8cf\">WebGPU</a>. \
               Try the latest Chrome, Edge, or Firefox Nightly.</p>\
             </div>",
            html_escape(message)
        ));
        el.remove_attribute("hidden").ok();
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
