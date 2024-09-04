use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> i32 { return n1 + n2 }

#[wasm_bindgen]
pub fn sub(n1: i32, n2: i32) -> i32 { return n1 - n2 }

#[wasm_bindgen]
pub fn mul(n1: i32, n2: i32) -> i32 { return n1 * n2 }

#[wasm_bindgen]
pub fn rem(n1: i32, n2: i32) -> i32 { return n1 % n2 }
