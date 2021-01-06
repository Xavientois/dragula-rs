use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Drake)]
    pub type Drake;

    #[wasm_bindgen(method, getter)]
    pub fn dragging(this: &Drake) -> bool;

    #[wasm_bindgen(method)]
    pub fn end(this: &Drake);

    #[wasm_bindgen(method, getter = containers)]
    fn containers_getter_impl(this: &Drake) -> JsValue;

    #[wasm_bindgen(method, setter = containers)]
    fn containers_setter_impl(this: &Drake, val: Box<[JsValue]>);

    #[wasm_bindgen(method, js_name = start)]
    fn start_impl(this: &Drake, item: JsValue) -> bool;

}

impl Drake {
    pub fn containers(&self) -> Vec<JsValue> {
        let containers = self.containers_getter_impl();
        let containers = Array::from(&containers);
        containers.to_vec()
    }

    pub fn set_containers<T>(&mut self, objs: &[&T])
    where
        T: JsCast + Clone,
    {
        let obj_array = objs.iter().cloned().map(JsValue::from).collect();
        self.containers_setter_impl(obj_array);
    }

    pub fn start<T>(&mut self, item: &T)
    where
        T: JsCast + Clone,
    {
        let item = JsValue::from(item);
        self.start_impl(item);
    }
}
