use crate::closure;
use derive_builder::Builder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Builder, Debug, Clone)]
pub struct Options {
    #[builder(private, default = "closure::to_js_1_ret(|_| JsValue::FALSE)")]
    is_container_func: JsValue,

    #[builder(default = "closure::to_js_4_ret(|_,_,_,_| JsValue::TRUE)")]
    #[builder(private)]
    moves_func: JsValue,

    #[builder(default = "closure::to_js_4_ret(|_,_,_,_| JsValue::TRUE)")]
    #[builder(private)]
    accepts_func: JsValue,

    #[builder(private, default = "closure::to_js_2_ret(|_,_| JsValue::FALSE)")]
    invalid_func: JsValue,

    #[builder(private, default = "JsValue::FALSE")]
    copy_func_or_bool: JsValue,
}

impl OptionsBuilder {
    pub fn is_container<F: 'static>(&mut self, value: F) -> &mut Self
    where
        F: FnMut(JsValue) -> JsValue,
    {
        let mut new = self;
        new.is_container_func = Some(closure::to_js_1_ret(value));
        new
    }

    pub fn moves<F: 'static>(&mut self, value: F) -> &mut Self
    where
        F: FnMut(JsValue, JsValue, JsValue, JsValue) -> JsValue,
    {
        let mut new = self;
        new.moves_func = Some(closure::to_js_4_ret(value));
        new
    }

    pub fn accepts<F: 'static>(&mut self, value: F) -> &mut Self
    where
        F: FnMut(JsValue, JsValue, JsValue, JsValue) -> JsValue,
    {
        let mut new = self;
        new.accepts_func = Some(closure::to_js_4_ret(value));
        new
    }

    pub fn invalid<F: 'static>(&mut self, value: F) -> &mut Self
    where
        F: FnMut(JsValue, JsValue) -> JsValue,
    {
        let mut new = self;
        new.invalid_func = Some(closure::to_js_2_ret(value));
        new
    }

    pub fn copy_function<F: 'static>(&mut self, value: F) -> &mut Self
    where
        F: FnMut(JsValue, JsValue) -> JsValue,
    {
        let mut new = self;
        new.copy_func_or_bool = Some(closure::to_js_2_ret(value));
        new
    }

    pub fn copy(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.copy_func_or_bool = Some(JsValue::from(value));
        new
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            is_container_func: closure::to_js_1_ret(|_| JsValue::FALSE),
            moves_func: closure::to_js_4_ret(|_, _, _, _| JsValue::TRUE),
            accepts_func: closure::to_js_4_ret(|_, _, _, _| JsValue::TRUE),
            invalid_func: closure::to_js_2_ret(|_, _| JsValue::FALSE),
            copy_func_or_bool: JsValue::FALSE,
        }
    }
}

#[wasm_bindgen]
#[doc(hidden)]
impl Options {
    #[wasm_bindgen(getter = isContainer)]
    pub fn is_container_func(&self) -> JsValue {
        self.is_container_func.clone()
    }

    #[wasm_bindgen(setter = isContainer)]
    pub fn set_is_container_func(&mut self, val: JsValue) {
        self.is_container_func = val;
    }

    #[wasm_bindgen(getter = moves)]
    pub fn moves_func(&self) -> JsValue {
        self.moves_func.clone()
    }

    #[wasm_bindgen(setter = moves)]
    pub fn set_moves_func(&mut self, val: JsValue) {
        self.moves_func = val;
    }

    #[wasm_bindgen(getter = accepts)]
    pub fn accepts_func(&self) -> JsValue {
        self.accepts_func.clone()
    }

    #[wasm_bindgen(setter = accepts)]
    pub fn set_accepts_func(&mut self, val: JsValue) {
        self.accepts_func = val;
    }

    #[wasm_bindgen(getter = invalid)]
    pub fn invalid_func(&self) -> JsValue {
        self.invalid_func.clone()
    }

    #[wasm_bindgen(setter = invalid)]
    pub fn set_invalid_func(&mut self, val: JsValue) {
        self.invalid_func = val;
    }

    #[wasm_bindgen(getter = copy)]
    pub fn copy_func_or_bool(&self) -> JsValue {
        self.copy_func_or_bool.clone()
    }

    #[wasm_bindgen(setter = copy)]
    pub fn set_copy_func_or_bool(&mut self, val: JsValue) {
        self.copy_func_or_bool = val;
    }
}

#[cfg(test)]
mod test;
