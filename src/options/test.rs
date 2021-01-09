use super::*;
use js_sys::Function;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn can_build_default() {
    console_error_panic_hook::set_once();

    let build_options =
        OptionsBuilder::default().build().expect("Build failed");
    let default_options = Options::default();

    let build_result = Function::from(build_options.is_container_func)
        .call1(&JsValue::NULL, &JsValue::TRUE)
        .unwrap();

    let default_result = Function::from(default_options.is_container_func)
        .call1(&JsValue::NULL, &JsValue::TRUE)
        .unwrap();

    assert_eq!(default_result, build_result);
}

#[wasm_bindgen_test]
fn set_is_container() {
    console_error_panic_hook::set_once();

    let build_options = OptionsBuilder::default()
        .is_container(|_| JsValue::TRUE)
        .build()
        .expect("Build failed");
    let default_options = Options::default();

    let build_result = Function::from(build_options.is_container_func)
        .call1(&JsValue::NULL, &JsValue::TRUE)
        .unwrap();

    let default_result = Function::from(default_options.is_container_func)
        .call1(&JsValue::NULL, &JsValue::TRUE)
        .unwrap();

    assert_ne!(default_result, build_result);
}
