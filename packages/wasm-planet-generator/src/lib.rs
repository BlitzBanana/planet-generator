#[macro_use]
extern crate serde_derive;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use std::panic;

mod map;
mod points;
mod elevation;

// API methods
#[wasm_bindgen(js_name = generateGrid)]
pub fn _generate_grid(seed: String, width: f64, height: f64, spacing: f64, chaos: f64) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let map = map::Map::generate(seed, width, height, spacing, chaos);
  JsValue::from_serde(&map).unwrap()
}
