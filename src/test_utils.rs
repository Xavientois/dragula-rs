use web_sys::*;

pub fn run_dom_test<F, R>(test: F, inner_html: &str) -> R
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

pub fn generate_draggable_containers(
    containers: u32,
    draggables: u32,
) -> String {
    let generate_draggables = |label, count| {
        (0..count).fold(String::new(), |acc, j| {
            format!(
                "{}<div id=\"drag_{}_{}\">Draggable {1}-{2}!</div>",
                acc, label, j
            )
        })
    };

    (0..containers).fold(String::new(), |acc, i| {
        format!(
            "{}<div id=\"cnt_{}\">{}</div>",
            acc,
            i,
            generate_draggables(i, draggables)
        )
    })
}
