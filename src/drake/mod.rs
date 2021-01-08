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

    #[wasm_bindgen(method)]
    pub fn remove(this: &Drake);

    #[wasm_bindgen(method)]
    pub fn cancel(this: &Drake, revert: bool);

    #[wasm_bindgen(method, getter = containers)]
    fn containers_getter_impl(this: &Drake) -> JsValue;

    #[wasm_bindgen(method, setter = containers)]
    fn containers_setter_impl(this: &Drake, val: Box<[JsValue]>);

    #[wasm_bindgen(method, js_name = start)]
    fn start_impl(this: &Drake, item: JsValue);

    #[wasm_bindgen(method)]
    fn on(this: &Drake, event_type: &str, listener: JsValue);
}

impl Drake {
    pub fn containers(&self) -> Vec<JsValue> {
        let containers = self.containers_getter_impl();
        let containers = Array::from(&containers);
        containers.to_vec()
    }

    pub fn set_containers<T>(&mut self, objs: &[T])
    where
        T: JsCast + Clone,
    {
        let obj_array =
            objs.iter().cloned().map(|o| JsValue::from(&o)).collect();
        self.containers_setter_impl(obj_array);
    }

    pub fn add_container<T>(&mut self, obj: T)
    where
        T: JsCast,
    {
        let mut containers = self.containers();
        let container_to_add = JsValue::from(&obj);
        containers.push(container_to_add);
        self.set_containers(&containers);
    }

    pub fn start<T>(&mut self, item: &T)
    where
        T: JsCast + Clone,
    {
        let item = JsValue::from(item);
        self.start_impl(item);
    }

    pub fn on_drag<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue),
    {
        const EVENT_NAME: &str = "drag";

        let closure = Closure::wrap(
            Box::new(listener) as Box<dyn FnMut(JsValue, JsValue)>
        );

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }

    pub fn on_dragend<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue),
    {
        const EVENT_NAME: &str = "dragend";

        let closure =
            Closure::wrap(Box::new(listener) as Box<dyn FnMut(JsValue)>);

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }

    pub fn on_drop<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "drop";

        let closure = Closure::wrap(Box::new(listener)
            as Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue)>);

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }

    pub fn on_cancel<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "cancel";

        let closure = Closure::wrap(
            Box::new(listener) as Box<dyn FnMut(JsValue, JsValue, JsValue)>
        );

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }

    pub fn on_remove<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "remove";

        let closure = Closure::wrap(
            Box::new(listener) as Box<dyn FnMut(JsValue, JsValue, JsValue)>
        );

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }

    pub fn on_shadow<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "shadow";

        let closure = Closure::wrap(
            Box::new(listener) as Box<dyn FnMut(JsValue, JsValue, JsValue)>
        );

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }

    pub fn on_over<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "over";

        let closure = Closure::wrap(
            Box::new(listener) as Box<dyn FnMut(JsValue, JsValue, JsValue)>
        );

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }

    pub fn on_out<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "out";

        let closure = Closure::wrap(
            Box::new(listener) as Box<dyn FnMut(JsValue, JsValue, JsValue)>
        );

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }

    pub fn on_cloned<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "cloned";

        let closure = Closure::wrap(
            Box::new(listener) as Box<dyn FnMut(JsValue, JsValue, JsValue)>
        );

        let listener = closure.into_js_value();

        self.on(EVENT_NAME, listener);
    }
}

#[cfg(test)]
pub mod test;
