use super::*;
use js_sys::Function;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn default_is_container() {
    console_error_panic_hook::set_once();

    let default_options = OptionsImpl::default();

    let default_result = Function::from(default_options.is_container_func)
        .call1(&JsValue::NULL, &JsValue::TRUE)
        .unwrap();

    assert_eq!(default_result, JsValue::FALSE);
}
