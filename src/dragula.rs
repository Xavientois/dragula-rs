use crate::drake::Drake;
use crate::options::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/js/dragula.min.js")]
extern "C" {
    fn wasm_dragula(
        containers: Box<[JsValue]>,
        options: OptionsImpl,
    ) -> JsValue;
}

/// Activates the dragula drag-and-drop system with sane default options
///
/// The simplest way to activate dragula is to call `dragula` and pass it
/// the list of containers whose contents will draggable.
///
/// ### Example:
/// ```no_run
/// use dragula::*;
///
/// let doc = web_sys::window().unwrap().document().unwrap();
/// let element_1 = doc.get_element_by_id("drag-container-1").unwrap();
/// let element_2 = doc.get_element_by_id("drag-container-2").unwrap();
///
/// let drake = dragula(&[element_1, element_2]);
///
/// ```
/// Calling `dragula` will return a [`Drake`](crate::Drake), allowing you to
/// interact with the active system managing drag-and-drop.
pub fn dragula<T>(objs: &[T]) -> Drake
where
    T: JsCast + Clone,
{
    dragula_options(objs, Options::default())
}

/// Activates the dragula drag-and-drop system with provided options
///
/// If you would like to customize the behaviour of the drag-and-drop systems
/// past the defaults provided, you can call `dragula_options` and provide an
/// [`Options`](crate::Options) instance to specify behaviour of certain
/// drag-and-drop features.
///
/// ### Example:
/// ```no_run
/// use dragula::*;
/// use dragula::options::CopyValue;
/// use web_sys::Element;
///
/// let doc = web_sys::window().unwrap().document().unwrap();
/// let element_1 = doc.get_element_by_id("drag-container-1").unwrap();
/// let element_2 = doc.get_element_by_id("drag-container-2").unwrap();
///
/// let options = Options {
///     moves: Box::new(|_el, _source, handle, _sibling| {
///         Element::from(handle).class_list().contains("drag-handle")
///     }),
///     copy: CopyValue::Func(Box::new(|_el, source| {
///         Element::from(source).class_list().contains("copy-container")
///     })),
///     ignore_input_text_selection: true,
///     ..Options::default()
/// };
///
/// let drake = dragula_options(&[element_1, element_2], options);
///
/// ```
/// Calling `dragula_options` will return a [`Drake`](crate::Drake), allowing you to
/// interact with the active system managing drag-and-drop.
pub fn dragula_options<T>(objs: &[T], options: Options) -> Drake
where
    T: JsCast + Clone,
{
    let obj_array = objs.iter().cloned().map(|o| JsValue::from(&o)).collect();
    let options = OptionsImpl::from(options);
    let drake = wasm_dragula(obj_array, options);
    drake.into()
}
