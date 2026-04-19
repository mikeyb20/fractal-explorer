use std::cell::RefCell;
use std::rc::Rc;

use fractal_core::{Fractal, Viewport};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, PointerEvent, ResizeObserver, WheelEvent, Window};

use crate::renderer::Renderer;
use crate::{state, util};

struct AppState {
    vp: Viewport,
    iter_cap: u32,
    renderer: Renderer,
    size: (u32, u32),
    dpr: f64,
    dragging: bool,
    last: (f32, f32),
    dirty: bool,
    fractal_id: String,
    last_url_write_ms: f64,
}

pub fn install(
    canvas: &HtmlCanvasElement,
    window: &Window,
    vp: Viewport,
    iter_cap: u32,
    renderer: Renderer,
    fractal: impl Fractal + 'static,
) {
    let dpr = window.device_pixel_ratio().max(1.0);
    let size = renderer.size();
    let state = Rc::new(RefCell::new(AppState {
        vp,
        iter_cap,
        renderer,
        size,
        dpr,
        dragging: false,
        last: (0.0, 0.0),
        dirty: true,
        fractal_id: fractal.id().to_string(),
        last_url_write_ms: 0.0,
    }));

    // pointerdown
    {
        let canvas_el = canvas.clone();
        let state = state.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |ev: PointerEvent| {
            let _ = canvas_el.set_pointer_capture(ev.pointer_id());
            let (x, y) = canvas_px(&canvas_el, &ev);
            let mut s = state.borrow_mut();
            s.dragging = true;
            s.last = (x, y);
        });
        canvas
            .add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref())
            .expect("add pointerdown listener");
        cb.forget();
    }

    // pointermove
    {
        let canvas_el = canvas.clone();
        let state = state.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |ev: PointerEvent| {
            let mut s = state.borrow_mut();
            if !s.dragging {
                return;
            }
            let (x, y) = canvas_px(&canvas_el, &ev);
            let dx = x - s.last.0;
            let dy = y - s.last.1;
            s.last = (x, y);
            let dpr = s.dpr as f32;
            let size = s.size;
            s.vp.pan_by_pixels((dx * dpr, dy * dpr), size);
            s.dirty = true;
        });
        canvas
            .add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref())
            .expect("add pointermove listener");
        cb.forget();
    }

    // pointerup / pointercancel
    for ev_name in ["pointerup", "pointercancel"] {
        let state = state.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |_ev: PointerEvent| {
            state.borrow_mut().dragging = false;
        });
        canvas
            .add_event_listener_with_callback(ev_name, cb.as_ref().unchecked_ref())
            .expect("add pointerup/cancel listener");
        cb.forget();
    }

    // wheel
    {
        let canvas_el = canvas.clone();
        let state = state.clone();
        let cb = Closure::<dyn FnMut(WheelEvent)>::new(move |ev: WheelEvent| {
            ev.prevent_default();
            let (x, y) = canvas_px(&canvas_el, &ev);
            let mut s = state.borrow_mut();
            let factor = if ev.delta_y() > 0.0 { 1.1 } else { 1.0 / 1.1 };
            let dpr = s.dpr as f32;
            let size = s.size;
            s.vp.zoom_toward((x * dpr, y * dpr), size, factor);
            s.dirty = true;
        });
        canvas
            .add_event_listener_with_callback("wheel", cb.as_ref().unchecked_ref())
            .expect("add wheel listener");
        cb.forget();
    }

    // ResizeObserver
    {
        let canvas_el = canvas.clone();
        let window_el = window.clone();
        let state = state.clone();
        let cb = Closure::<dyn FnMut(js_sys::Array)>::new(move |_entries: js_sys::Array| {
            let (w, h) = util::canvas_size_px(&canvas_el, &window_el);
            let mut s = state.borrow_mut();
            s.size = (w, h);
            s.dpr = window_el.device_pixel_ratio().max(1.0);
            s.vp.aspect_ratio = w as f32 / h as f32;
            s.renderer.resize(w, h);
            s.dirty = true;
        });
        let observer =
            ResizeObserver::new(cb.as_ref().unchecked_ref()).expect("ResizeObserver::new");
        observer.observe(canvas);
        cb.forget();
        std::mem::forget(observer);
    }

    // rAF render loop
    {
        let window_for_tick = window.clone();
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        let state_tick = state.clone();
        *g.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            {
                let mut s = state_tick.borrow_mut();
                if s.dirty {
                    let vp = s.vp;
                    let iter_cap = s.iter_cap;
                    s.renderer.draw(&vp, iter_cap);
                    s.dirty = false;
                    let now = js_sys::Date::now();
                    if now - s.last_url_write_ms > 150.0 {
                        state::write_url(&window_for_tick, &s.fractal_id, &s.vp, s.iter_cap);
                        s.last_url_write_ms = now;
                    }
                }
            }
            if let Some(cb) = f.borrow().as_ref() {
                let _ = window_for_tick
                    .request_animation_frame(cb.as_ref().unchecked_ref());
            }
        }));
        if let Some(cb) = g.borrow().as_ref() {
            let _ = window
                .request_animation_frame(cb.as_ref().unchecked_ref());
        }
        std::mem::forget(g);
    }
}

fn canvas_px(canvas: &HtmlCanvasElement, ev: &MouseEvent) -> (f32, f32) {
    let rect = canvas.get_bounding_client_rect();
    let x = ev.client_x() as f32 - rect.left() as f32;
    let y = ev.client_y() as f32 - rect.top() as f32;
    (x, y)
}
