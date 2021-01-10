use crate::closure;
use wasm_bindgen::prelude::*;

/// Since the `copy` option can be either a function or a boolean, this enum
/// encapsulates the possible values for the copy option.
///
/// The closure signature is `(el, handle)`, the element to check and the
/// element that was directly clicked on.
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

impl ToString for Direction {
    fn to_string(&self) -> String {
        const VERTICAL: &str = "vertical";
        const HORIZONTAL: &str = "horizontal";

        match self {
            Direction::Vertical => String::from(VERTICAL),
            Direction::Horizontal => String::from(HORIZONTAL),
        }
    }
}

/// Used to pass options when activating Dragula
///
/// When passed to the [`dragula_options`](crate::dragula_options) function,
/// this struct can be used to specify options to control the behaviour of the
/// drag-and-drop functionality.
///
/// For example:
/// ```no_run
/// use dragula::*;
/// use dragula::options::CopyValue;
/// use web_sys::Element;
/// # use wasm_bindgen::JsValue;
///
/// # let element = JsValue::TRUE;
/// //--snip--
///
/// let options = Options {
///     invalid: Box::new(|el, _handle| {
///         Element::from(el).tag_name() == String::from("A")
///     }),
///     copy: CopyValue::Bool(true),
///     copy_sort_source: true,
///     remove_on_spill: true,
///     slide_factor_x: 10,
///     slide_factor_y: 10,
///     ..Options::default()
/// };
///
/// let drake = dragula_options(&[element], options);
///
/// //--snip--
/// ```
pub struct Options {
    /// Besides the containers that you pass to [`dragula`](crate::dragula()),
    /// or the containers you dynamically add, you can also use this closure to
    /// specify any sort of logic that defines what is a container
    /// for this particular [`Drake`](crate::Drake) instance.
    ///
    /// This closure will be invoked with the element that is being checked for
    /// whether it is a container.
    pub is_container: Box<dyn FnMut(JsValue) -> bool>,
    /// You can define a `moves` closure which will be invoked with `(el, source,
    /// handle, sibling)` whenever an element is clicked. If this closure returns
    /// `false`, a drag event won't begin, and the event won't be prevented
    /// either. The `handle` element will be the original click target, which
    /// comes in handy to test if that element is an expected _"drag handle"_.
    pub moves: Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue) -> bool>,
    /// You can set `accepts` to a closure with the following signature: `(el,
    /// target, source, sibling)`. It'll be called to make sure that an element
    /// `el`, that came from container `source`, can be dropped on container
    /// `target` before a `sibling` element. The `sibling` can be `null`, which
    /// would mean that the element would be placed as the last element in the
    /// container. Note that if [`copy`](Options::copy) is set to `true`, `el` will be
    /// set to the copy, instead of the originally dragged element.
    pub accepts: Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue) -> bool>,
    /// You can provide an `invalid` closure with a `(el, handle)` signature.
    /// This closure should return `true` for elements that shouldn't trigger a
    /// drag. The `handle` argument is the element that was clicked, while `el`
    /// is the item that would be dragged.
    pub invalid: Box<dyn FnMut(JsValue, JsValue) -> bool>,
    /// If `copy` is set to `true` _(or a closure that returns `true`)_, items
    /// will be copied rather than moved. This implies the following differences:
    ///
    /// Event     | Move                                     | Copy
    /// ----------|------------------------------------------|---------------------------------------------
    /// `drag`    | Element will be concealed from `source`  | Nothing happens
    /// `drop`    | Element will be moved into `target`      | Element will be cloned into `target`
    /// `remove`  | Element will be removed from DOM         | Nothing happens
    /// `cancel`  | Element will stay in `source`            | Nothing happens
    ///
    /// If a closure is passed, it'll be called whenever an element starts being
    /// dragged in order to decide whether it should follow `copy` behavior or
    /// not. This closure will be passed the element to be dragged as well as
    /// its source container, in other words, the signature is `(el, handle)`.
    ///
    /// `false` by default.
    pub copy: CopyValue,
    /// If [`copy`](Options::copy) is set to `true` _(or a closure that
    /// returns `true`)_ and `copy_sort_source` is `true` as well, users will
    /// be able to sort elements in `copy`-source containers.
    ///
    /// `false` by default.
    pub copy_sort_source: bool,
    /// By default, spilling an element outside of any containers will move the
    /// element back to the _drop position previewed by the feedback shadow_.
    /// Setting `revert_on_spill` to `true` will ensure elements dropped outside
    /// of any approved containers are moved back to the source element where
    /// the drag event began, rather than stay at the _drop position previewed
    /// by the feedback shadow_.
    ///
    /// `false` by default.
    pub revert_on_spill: bool,
    /// By default, spilling an element outside of any containers will move the
    /// element back to the _drop position previewed by the feedback shadow_.
    /// Setting `remove_on_spill` to `true` will ensure elements dropped outside
    /// of any approved containers are removed from the DOM. Note that `remove`
    /// events won't fire if [`copy`](Options::copy) is set to `true`.
    ///
    /// `false` by default.
    pub remove_on_spill: bool,
    /// When an element is dropped onto a container, it'll be placed near the
    /// point where the mouse was released. If the `direction` is
    /// [`Vertical`](Direction::Vertical),
    /// the default value, the Y axis will be considered. Otherwise, if the
    /// `direction` is [`Horizontal`](Direction::Horizontal),
    ///  the X axis will be considered.
    ///
    /// [`Vertical`](Direction::Vertical), by default.
    pub direction: Direction,
    /// The DOM element where the mirror element displayed while dragging will
    /// be appended to.
    ///
    /// `document.body` by default.
    pub mirror_container: JsValue,
    /// When this option is enabled, if the user clicks on an input element the
    /// drag won't start until their mouse pointer exits the input. This
    /// translates into the user being able to select text in inputs contained
    /// inside draggable elements, and still drag the element by moving their
    /// mouse outside of the input -- so you get the best of both worlds.
    ///
    /// `true` by default.
    pub ignore_input_text_selection: bool,
    /// The amount of horizontal movement (in pixels) for a click to be
    /// considered a drag
    ///
    /// `0` by default.
    pub slide_factor_x: i32,
    /// The amount of vertical movement (in pixels) for a click to be
    /// considered a drag
    ///
    /// `0` by default.
    pub slide_factor_y: i32,
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
            slide_factor_x: 0,
            slide_factor_y: 0,
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

    direction: String,

    mirror_container_elem: JsValue,

    #[wasm_bindgen(js_name = ignoreInputTextSelection)]
    pub ignore_input_text_selection: bool,

    #[wasm_bindgen(js_name = slideFactorX)]
    pub slide_factor_x: i32,

    #[wasm_bindgen(js_name = slideFactorY)]
    pub slide_factor_y: i32,
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
            direction: options.direction.to_string(),
            ignore_input_text_selection: options.ignore_input_text_selection,
            slide_factor_x: options.slide_factor_x,
            slide_factor_y: options.slide_factor_y,
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

    #[wasm_bindgen(getter)]
    pub fn direction(&self) -> String {
        self.direction.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_direction(&mut self, val: String) {
        self.direction = val;
    }
}

#[cfg(test)]
mod test;
