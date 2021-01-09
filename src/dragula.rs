use crate::drake::Drake;
use crate::options::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/js/dragula.min.js")]
extern "C" {
    fn wasm_dragula(
        containers: Box<[JsValue]>,
        options: OptionsImpl,
    ) -> JsValue;
}

pub fn dragula<T>(objs: &[T]) -> Drake
where
    T: JsCast + Clone,
{
    dragula_options(objs, Options::default())
}

pub fn dragula_options<T>(objs: &[T], options: Options) -> Drake
where
    T: JsCast + Clone,
{
    let obj_array = objs.iter().cloned().map(|o| JsValue::from(&o)).collect();
    let options = OptionsImpl::from(options);
    let drake = wasm_dragula(obj_array, options);
    drake.into()
}
