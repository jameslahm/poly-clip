pub mod clip;

use clip::{Point, Polygon};
use wasm_bindgen::prelude::*;



// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, poly-clip!");
}

#[wasm_bindgen]
pub fn clip(clip_points: JsValue, primary_points: JsValue) -> JsValue {
    let clip_points: Vec<Point> = clip_points.into_serde().unwrap();
    let primary_points: Vec<Point> = primary_points.into_serde().unwrap();
    let clip_polygon = Polygon::new(clip_points);
    let primary_polygon = Polygon::new(primary_points);
    let res = clip_polygon.clip(&primary_polygon);
    return JsValue::from_serde(&res).unwrap();
}

