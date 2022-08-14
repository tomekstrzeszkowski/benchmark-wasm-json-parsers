mod utils;
mod car;
use wasm_bindgen::prelude::*;
use chrono::{Utc, TimeZone};

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
pub fn greets() {
    let car = car::Car{
        name: String::from("Honda"),
        miles_per_galon: 12.6,
        displacement: None,
        horsepower: 140,
        weight_in_lbs: 1900,
        cylinders: 4,
        year: Some(Utc.ymd(1970, 1, 1)),
        acceleration: 100,
    };
    let label = &format!("Work in progress, wasm-rust {}!", car.name);
    alert(label);
}