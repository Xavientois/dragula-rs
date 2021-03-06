use super::*;
use crate::dragula;
use crate::test_utils::*;
use wasm_bindgen_test::*;
use web_sys::*;

wasm_bindgen_test_configure!(run_in_browser);

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

    for i in 0..9 {
        let html = generate_draggable_containers(i, 0);

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

#[wasm_bindgen_test]
fn set_containers_after() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(5, 0);

    let test = |element: &Element| {
        let empty_containers: Vec<JsValue> = vec![];

        let mut drake = dragula(&empty_containers);

        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        drake.set_containers(&in_containers);

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

#[wasm_bindgen_test]
#[cfg(feature = "js-sys")]
fn add_containers_individually() {
    console_error_panic_hook::set_once();

    for i in 0..9 {
        let html = generate_draggable_containers(i, 0);

        let test = |element: &Element| {
            let empty_containers: Vec<JsValue> = vec![];

            let mut drake = dragula(&empty_containers);

            let containers = element.children();
            let in_containers: Vec<_> = (0..containers.length())
                .map(|j| containers.item(j))
                .map(Option::unwrap)
                .collect();

            for container in in_containers.iter() {
                drake.add_container(container.clone());
            }

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

#[wasm_bindgen_test]
fn dragging_off_by_default() {
    let in_containers: Vec<JsValue> = vec![];
    let drake = dragula(&in_containers);
    let dragging = drake.dragging();

    assert!(!dragging);
}

#[wasm_bindgen_test]
fn start_makes_dragging_true() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        let mut drake = dragula(&in_containers);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        drake.start(&item);

        let dragging = drake.dragging();

        assert!(dragging);
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn end_makes_dragging_false() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        let mut drake = dragula(&in_containers);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        drake.start(&item);

        let dragging = drake.dragging();
        assert!(dragging);

        drake.end();

        let dragging = drake.dragging();
        assert!(!dragging);
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn remove_removes_element_from_dom() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        let mut drake = dragula(&in_containers);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        drake.start(&item);

        let dragging = drake.dragging();
        assert!(dragging);

        drake.remove();

        let parent = item.parent_element();
        assert!(parent.is_none());
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn remove_makes_dragging_false() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        let mut drake = dragula(&in_containers);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        drake.start(&item);

        let dragging = drake.dragging();
        assert!(dragging);

        drake.remove();

        let dragging = drake.dragging();
        assert_eq!(dragging, false);
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn cancel_triggers_callback() {
    console_error_panic_hook::set_once();

    const CANCEL_ATTR: &str = "cancelled";
    const TRUE: &str = "true";

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        let mut drake = dragula(&in_containers);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        let item_ref = item.clone();
        drake.on_cancel(move |_, _, _| {
            item_ref
                .set_attribute(CANCEL_ATTR, TRUE)
                .expect("Unable to set cancelled attribute");
        });

        drake.start(&item);
        drake.cancel();

        let was_cancelled = item
            .get_attribute(CANCEL_ATTR)
            .expect("Cancel attribute not set");

        assert_eq!(was_cancelled, TRUE);
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn cancel_with_revert_triggers_callback() {
    console_error_panic_hook::set_once();

    const CANCEL_ATTR: &str = "cancelled";
    const TRUE: &str = "true";

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        let mut drake = dragula(&in_containers);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        let item_ref = item.clone();
        drake.on_cancel(move |_, _, _| {
            item_ref
                .set_attribute(CANCEL_ATTR, TRUE)
                .expect("Unable to set cancelled attribute");
        });

        drake.start(&item);
        drake.cancel_with_revert(true);

        let was_cancelled = item
            .get_attribute(CANCEL_ATTR)
            .expect("Cancel attribute not set");

        assert_eq!(was_cancelled, TRUE);
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn can_move_returns_true_when_appropriate() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        // Exclude first div from in_containers
        let drake = dragula(&in_containers);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        assert!(drake.can_move(&item));
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn can_move_returns_false_when_appropriate() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        // Exclude first div from in_containers
        let drake = dragula(&in_containers[1..]);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        assert!(!drake.can_move(&item));
    };

    run_dom_test(test, &html);
}

#[wasm_bindgen_test]
fn destroy_makes_dragging_false() {
    console_error_panic_hook::set_once();

    let html = generate_draggable_containers(2, 3);

    let test = |element: &Element| {
        let containers = element.children();
        let in_containers: Vec<_> = (0..containers.length())
            .map(|i| containers.item(i))
            .map(Option::unwrap)
            .collect();

        let mut drake = dragula(&in_containers);

        let item = in_containers
            .first()
            .unwrap()
            .first_element_child()
            .unwrap();

        drake.start(&item);

        let dragging = drake.dragging();
        assert!(dragging);

        drake.destroy();

        let dragging = drake.dragging();
        assert!(!dragging);
    };

    run_dom_test(test, &html);
}
