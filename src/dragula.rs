use crate::drake::Drake;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/js/dragula.min.js")]
extern "C" {
    fn wasm_dragula(containers: Box<[JsValue]>, options: JsValue) -> JsValue;
}

pub fn dragula<T>(objs: &[T]) -> Drake
where
    T: JsCast + Clone,
{
    let obj_array = objs.iter().cloned().map(|o| JsValue::from(&o)).collect();
    let drake = wasm_dragula(obj_array, JsValue::NULL);
    drake.into()
}
