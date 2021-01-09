use crate::closure;
use wasm_bindgen::prelude::*;

/// Since the `copy` option can be either a function or a boolean, this enum
/// encapsulates the possible values for the copy option.
pub enum CopyValue {
    Bool(bool),
    Func(Box<dyn FnMut(JsValue, JsValue) -> bool>),
}

impl From<CopyValue> for JsValue {
    fn from(copy: CopyValue) -> JsValue {
        match copy {
            CopyValue::Bool(copy) => JsValue::from(copy),
            CopyValue::Func(copy) => closure::to_js_2_ret(copy),
        }
    }
}

/// The axis to be considered when determining the location an element will be
/// placed when dropped.
///
/// When an element is dropped onto a container, it will be placed near the
/// point where the mouse was released. If the `direction` is `Vertical`,
/// the default value, the Y axis will be considered. Otherwise, if the
/// `direction` is `Horizontal`, the X axis will be considered.
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Direction {
    pub(super) fn to_str(&self) -> &'static str {
        const VERTICAL: &str = "vertical";
        const HORIZONTAL: &str = "horizontal";

        match self {
            Direction::Vertical => VERTICAL,
            Direction::Horizontal => HORIZONTAL,
        }
    }
}

pub struct Options {
    pub is_container: Box<dyn FnMut(JsValue) -> bool>,
    pub moves: Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue) -> bool>,
    pub accepts: Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue) -> bool>,
    pub invalid: Box<dyn FnMut(JsValue, JsValue) -> bool>,
    pub copy: CopyValue,
    pub copy_sort_source: bool,
    pub revert_on_spill: bool,
    pub remove_on_spill: bool,
    pub direction: Direction,
    pub mirror_container: JsValue,
    pub ignore_input_text_selection: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            is_container: Box::new(|_| false),
            moves: Box::new(|_, _, _, _| true),
            accepts: Box::new(|_, _, _, _| true),
            invalid: Box::new(|_, _| false),
            copy: CopyValue::Bool(false),
            copy_sort_source: false,
            revert_on_spill: false,
            remove_on_spill: false,
            direction: Direction::Vertical,
            // Will default to document.body (avoiding web_sys dependency)
            mirror_container: JsValue::UNDEFINED,
            ignore_input_text_selection: true,
        }
    }
}

#[doc(hidden)]
#[wasm_bindgen]
pub struct OptionsImpl {
    is_container_func: JsValue,
    moves_func: JsValue,
    accepts_func: JsValue,
    invalid_func: JsValue,
    copy_func_or_bool: JsValue,

    #[wasm_bindgen(js_name = copySortSource)]
    pub copy_sort_source: bool,

    #[wasm_bindgen(js_name = revertOnSpill)]
    pub revert_on_spill: bool,

    #[wasm_bindgen(js_name = removeOnSpill)]
    pub remove_on_spill: bool,

    pub direction: &'static str,

    mirror_container_elem: JsValue,

    #[wasm_bindgen(js_name = ignoreInputTextSelection)]
    pub ignore_input_text_selection: bool,
}

impl From<Options> for OptionsImpl {
    fn from(options: Options) -> Self {
        OptionsImpl {
            is_container_func: closure::to_js_1_ret(options.is_container),
            moves_func: closure::to_js_4_ret(options.moves),
            accepts_func: closure::to_js_4_ret(options.accepts),
            invalid_func: closure::to_js_2_ret(options.invalid),
            copy_func_or_bool: JsValue::from(options.copy),
            mirror_container_elem: options.mirror_container,
            copy_sort_source: options.copy_sort_source,
            revert_on_spill: options.revert_on_spill,
            remove_on_spill: options.remove_on_spill,
            direction: options.direction.to_str(),
            ignore_input_text_selection: options.ignore_input_text_selection,
        }
    }
}

impl Default for OptionsImpl {
    fn default() -> Self {
        OptionsImpl::from(Options::default())
    }
}

#[wasm_bindgen]
#[doc(hidden)]
impl OptionsImpl {
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

    #[wasm_bindgen(getter = mirrorContainer)]
    pub fn mirror_container_elem(&self) -> JsValue {
        self.mirror_container_elem.clone()
    }

    #[wasm_bindgen(setter = mirrorContainer)]
    pub fn set_mirror_container_elem(&mut self, val: JsValue) {
        self.mirror_container_elem = val;
    }
}

#[cfg(test)]
mod test;
