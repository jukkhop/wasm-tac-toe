// mod utils;
mod board;

use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let ctx = context();
    let mut i = 0;

    let func = Box::new(move || {
        i += 1;
        render(&ctx, i);
        request_animation_frame(f.borrow().as_ref().unwrap());
    });

    // Wrap the box into a Closure
    let closure = Closure::wrap(func as Box<dyn FnMut()>);
    *g.borrow_mut() = Some(closure);

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn context() -> web_sys::CanvasRenderingContext2d {
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn render(ctx: &web_sys::CanvasRenderingContext2d, i: i32) {
    ctx.clear_rect(0.0, 0.0, 150.0, 150.0);
    ctx.begin_path();

    ctx.set_fill_style(&JsValue::from("black"));
    let text = format!("{}", i);

    ctx.set_font("16px serif");
    ctx.fill_text(&text[..], 0f64, 16f64).unwrap();

    // Draw the outer circle.
    ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    ctx.stroke();
}
