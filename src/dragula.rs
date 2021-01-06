use crate::drake::Drake;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[cfg(feature = "yew-types")]
use yew::NodeRef;

#[wasm_bindgen(module = "/js/dragula.min.js")]
extern "C" {
    fn wasm_dragula(containers: Box<[JsValue]>, options: JsValue) -> JsValue;
}

pub fn dragula<T>(objs: &[&T]) -> Drake
where
    T: JsCast + Clone,
{
    let obj_array = objs.iter().cloned().map(JsValue::from).collect();
    let drake = wasm_dragula(obj_array, JsValue::NULL);
    drake.into()
}

#[cfg(feature = "yew-types")]
pub fn dragula_node_refs(nodes: &[NodeRef]) -> Drake {
    let node_array = nodes
        .iter()
        .map(NodeRef::get)
        .map(|n| n.unwrap())
        .map(JsValue::from)
        .collect();
    let drake = wasm_dragula(node_array, JsValue::NULL);
    drake.into()
}
