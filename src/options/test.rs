use super::*;
use crate::test_utils::*;
use crate::*;
use js_sys::Function;
use wasm_bindgen_test::*;
use web_sys::Element;

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

#[wasm_bindgen_test]
fn is_container_includes_correctly() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let in_containers: Vec<JsValue> = vec![];
        let options = Options {
            is_container: Box::new(|el| Element::from(el).id() == "cnt_0"),
            direction: Direction::Horizontal,
            revert_on_spill: true,
            ..Options::default()
        };
        let drake = dragula_options(&in_containers, options);

        let containers = element.children();
        let containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();
        let item = containers.first().unwrap().first_element_child().unwrap();

        assert!(drake.can_move(&item));
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn is_container_excludes_correctly() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let in_containers: Vec<JsValue> = vec![];
        let options = Options {
            is_container: Box::new(|el| Element::from(el).id() == "cnt_1"),
            direction: Direction::Horizontal,
            revert_on_spill: true,
            ..Options::default()
        };
        let drake = dragula_options(&in_containers, options);

        let containers = element.children();
        let containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();
        let item = containers.first().unwrap().first_element_child().unwrap();

        assert!(!drake.can_move(&item));
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn moves_includes_correctly() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let in_containers: Vec<JsValue> = vec![];
        let options = Options {
            is_container: Box::new(|el| Element::from(el).id() == "cnt_0"),
            moves: Box::new(|el, _, _, _| Element::from(el).id() == "drag_0_0"),
            direction: Direction::Horizontal,
            revert_on_spill: true,
            ..Options::default()
        };
        let drake = dragula_options(&in_containers, options);

        let containers = element.children();
        let containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();
        let item = containers.first().unwrap().first_element_child().unwrap();

        assert!(drake.can_move(&item));
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn moves_excludes_correctly() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let in_containers: Vec<JsValue> = vec![];
        let options = Options {
            moves: Box::new(|el, _, _, _| Element::from(el).id() == "drag_0_1"),
            direction: Direction::Horizontal,
            revert_on_spill: true,
            ..Options::default()
        };
        let drake = dragula_options(&in_containers, options);

        let containers = element.children();
        let containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();
        let item = containers.first().unwrap().first_element_child().unwrap();

        assert!(!drake.can_move(&item));
    };

    run_dom_test(test, &html);
}
