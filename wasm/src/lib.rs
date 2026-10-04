use wasm_bindgen::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct Out {
    class: usize,
    scores: [u16; 10],
    early: bool,
}

#[wasm_bindgen]
pub fn infer(plane0: &[u32], plane1: &[u32]) -> JsValue {
    if plane0.len() != 24 || plane1.len() != 24 {
        wasm_bindgen::throw_str("plane arrays must have length 24");
    }

    let mut a0 = [0u32; 24];
    let mut a1 = [0u32; 24];
    a0.copy_from_slice(plane0);
    a1.copy_from_slice(plane1);

    let res = inference::infer(&a0, &a1);

    let out = Out {
        class: res.class,
        scores: res.scores,
        early: res.early,
    };

    serde_wasm_bindgen::to_value(&out).unwrap()
}
