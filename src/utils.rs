// Util functions

use crate::dom::{window_height, window_width};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

pub fn scale_canvas(window: &Window, canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {
    let width = window_width();
    let height = window_height();
    let pixel_ratio = window.device_pixel_ratio();

    let scaled_width = width * pixel_ratio;
    let scaled_height = height * pixel_ratio;

    canvas.set_width(scaled_width as u32);
    canvas.set_height(scaled_height as u32);

    if pixel_ratio > 1.0 {
        ctx.scale(pixel_ratio, pixel_ratio).unwrap();
    }
}

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
