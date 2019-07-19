// Utility functions for using the dom (via the web-sys crate)

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, console, HtmlCanvasElement, Window};

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn canvas() -> HtmlCanvasElement {
    window()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

pub fn set_timeout(f: &Closure<dyn FnMut()>, timeout: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), timeout)
        .expect("should register `setTimeout` OK");
}

// pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
//     window()
//         .request_animation_frame(f.as_ref().unchecked_ref())
//         .expect("should register `requestAnimationFrame` OK");
// }

pub fn log_str(s: &str) {
    console::log_1(&JsValue::from_str(s));
}

/// log anything. Uses format!
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (log_str(&format!($($arg)*)));
}
