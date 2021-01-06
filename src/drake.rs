use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[cfg(feature = "yew-types")]
use yew::NodeRef;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Drake)]
    pub type Drake;

    #[wasm_bindgen(method, getter)]
    pub fn dragging(this: &Drake) -> bool;

    #[wasm_bindgen(method, getter = containers)]
    fn containers_getter(this: &Drake) -> JsValue;

    #[wasm_bindgen(method, setter = containers)]
    fn containers_setter(this: &Drake, val: Box<[JsValue]>);
}

impl Drake {
    pub fn containers(&self) -> Vec<JsValue> {
        let containers = self.containers_getter();
        let containers = Array::from(&containers);
        containers.to_vec()
    }

    #[cfg(feature = "yew-types")]
    pub fn containers_node_refs(&self) -> Vec<NodeRef> {
        let containers = self.containers_getter();
        Array::from(&containers)
            .iter()
            .map()
            .map(NodeRef::from)
            .collect()
    }

    pub fn set_containers<T>(&mut self, objs: &[&T])
    where
        T: JsCast + Clone,
    {
        let obj_array = objs.iter().cloned().map(JsValue::from).collect();
        self.containers_setter(obj_array);
    }
}
