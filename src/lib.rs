mod board;
mod dom;
mod fps;
mod main;
mod math;
mod profiler;
mod utils;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    main::run();
    Ok(())
}
