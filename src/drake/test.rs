use super::*;
use crate::dragula;
use wasm_bindgen_test::*;
use web_sys::*;

wasm_bindgen_test_configure!(run_in_browser);

fn run_dom_test<F, R>(test: F, inner_html: &str) -> R
where
    F: FnOnce(&Element) -> R,
{
    let doc = web_sys::window().unwrap().document().unwrap();
    let body = doc.body().expect("Unable to get document body");

    // Create and add testing node
    let div = doc.create_element("div").expect("Could not add node");
    let div_id = format!("TEST_DIV_{}", js_sys::Math::random());
    div.set_id(&div_id);
    div.set_inner_html(inner_html);
    body.append_with_node_1(div.as_ref())
        .expect("Could not append node");

    let retval = test(&div);

    div.remove();

    retval
}

#[wasm_bindgen_test]
fn empty_init() {
    let in_containers: Vec<JsValue> = vec![];
    let drake = dragula(&in_containers);
    let out_containers = drake.containers();

    assert_eq!(in_containers, out_containers);
}

#[wasm_bindgen_test]
fn multiple_containers() {
    console_error_panic_hook::set_once();

    for i in 0..8 {
        let html = (0..i).fold(String::new(), |acc, j| {
            format!("{}<div id=\"cnt_{}\"></div>", acc, j)
        });

        let test = |element: &Element| {
            let containers = element.children();
            let in_containers: Vec<_> = (0..containers.length())
                .map(|j| containers.item(j))
                .map(Option::unwrap)
                .collect();

            let drake = dragula(&in_containers);

            let out_containers: Vec<_> = drake
                .containers()
                .iter()
                .cloned()
                .map(Element::from)
                .collect();

            assert_eq!(in_containers, out_containers);
        };

        run_dom_test(test, &html);
    }
}
