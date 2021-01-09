use wasm_bindgen::prelude::*;

pub fn to_js_1<F: 'static>(listener: F) -> JsValue
where
    F: FnMut(JsValue),
{
    let closure = Closure::wrap(Box::new(listener) as Box<dyn FnMut(JsValue)>);
    closure.into_js_value()
}

pub fn to_js_2<F: 'static>(listener: F) -> JsValue
where
    F: FnMut(JsValue, JsValue),
{
    let closure =
        Closure::wrap(Box::new(listener) as Box<dyn FnMut(JsValue, JsValue)>);
    closure.into_js_value()
}

pub fn to_js_3<F: 'static>(listener: F) -> JsValue
where
    F: FnMut(JsValue, JsValue, JsValue),
{
    let closure = Closure::wrap(
        Box::new(listener) as Box<dyn FnMut(JsValue, JsValue, JsValue)>
    );
    closure.into_js_value()
}

pub fn to_js_4<F: 'static>(listener: F) -> JsValue
where
    F: FnMut(JsValue, JsValue, JsValue, JsValue),
{
    let closure = Closure::wrap(Box::new(listener)
        as Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue)>);
    closure.into_js_value()
}

pub fn to_js_1_ret<F: 'static>(listener: F) -> JsValue
where
    F: FnMut(JsValue) -> bool,
{
    let closure =
        Closure::wrap(Box::new(listener) as Box<dyn FnMut(JsValue) -> bool>);
    closure.into_js_value()
}

pub fn to_js_2_ret<F: 'static>(listener: F) -> JsValue
where
    F: FnMut(JsValue, JsValue) -> bool,
{
    let closure = Closure::wrap(
        Box::new(listener) as Box<dyn FnMut(JsValue, JsValue) -> bool>
    );
    closure.into_js_value()
}

pub fn to_js_4_ret<F: 'static>(listener: F) -> JsValue
where
    F: FnMut(JsValue, JsValue, JsValue, JsValue) -> bool,
{
    let closure = Closure::wrap(Box::new(listener)
        as Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue) -> bool>);
    closure.into_js_value()
}
