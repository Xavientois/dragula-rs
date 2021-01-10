use crate::closure;
#[cfg(any(feature = "js-sys", test))]
use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    /// Interface provided by Dragula to interact with active drag-and-drop system
    ///
    /// ### Example:
    #[cfg_attr(feature = "js-sys", doc = "```no_run")]
    #[cfg_attr(not(feature = "js-sys"), doc = "```no_compile")]
    /// use dragula::*;
    ///
    /// let doc = web_sys::window().unwrap().document().unwrap();
    /// let element_1 = doc.get_element_by_id("drag-container-1").unwrap();
    /// let element_2 = doc.get_element_by_id("drag-container-2").unwrap();
    ///
    /// let mut drake = dragula(&[element_1]);
    ///
    /// drake.add_container(element_2);
    ///
    /// ```
    #[wasm_bindgen(js_name = Drake)]
    pub type Drake;

    /// This property will be `true` whenever an element is being dragged.
    #[wasm_bindgen(method, getter)]
    pub fn dragging(this: &Drake) -> bool;

    /// Gracefully end the drag event as if using **the last position marked by
    /// the preview shadow** as the drop target. The proper `cancel` or `drop`
    /// event will be fired, depending on whether the item was dropped back
    /// where it was originally lifted from _(which is essentially a no-op
    /// that's treated as a `cancel` event)_.
    #[wasm_bindgen(method)]
    pub fn end(this: &Drake);

    /// If an element managed by `Drake` is currently being dragged, this method
    /// will gracefully remove it from the DOM.
    #[wasm_bindgen(method)]
    pub fn remove(this: &Drake);

    /// Removes all drag and drop events used by `dragula` to manage drag and
    /// drop between the `containers`. If `destroy` is called while an element
    /// is being dragged, the drag will be effectively cancelled.
    #[wasm_bindgen(method)]
    pub fn destroy(this: &Drake);

    #[wasm_bindgen(method)]
    fn on(this: &Drake, event_type: &str, listener: JsValue);

    /// If an element managed by `Drake` is currently being dragged, this method
    /// will gracefully cancel the drag action.
    ///
    /// Note that **a _"cancellation"_ will result in a `cancel` event** only in
    /// the following scenarios.
    ///
    /// - `revert_on_spill` is `true`
    /// - Drop target _(as previewed by the feedback shadow)_ is the source
    /// container **and** the item is dropped in the same position where it
    /// was originally dragged from
    #[wasm_bindgen(method)]
    pub fn cancel(this: &Drake);

    /// If an element managed by `Drake` is currently being dragged, this method
    /// will gracefully cancel the drag action. If `true` is passed to this
    /// function, it will effectively produce the same result as if
    /// `revert_on_spill` is true.
    ///
    /// - `revert_on_spill` is `true`
    /// - Drop target _(as previewed by the feedback shadow)_ is the source
    /// container **and** the item is dropped in the same position where it
    /// was originally dragged from
    #[wasm_bindgen(method, js_name = cancel)]
    pub fn cancel_with_revert(this: &Drake, revert: bool);

    #[wasm_bindgen(method, getter = containers)]
    fn containers_getter_impl(this: &Drake) -> JsValue;

    #[wasm_bindgen(method, setter = containers)]
    fn containers_setter_impl(this: &Drake, val: Box<[JsValue]>);

    #[wasm_bindgen(method, js_name = start)]
    fn start_impl(this: &Drake, item: JsValue);

    #[wasm_bindgen(method, js_name = canMove)]
    fn can_move_impl(this: &Drake, item: JsValue) -> bool;
}

impl Drake {
    /// Gets the active containers currently allowing dragging
    ///
    /// Requires that feature `js-sys` be turned on (it is on by default)
    #[cfg(any(feature = "js-sys", test))]
    pub fn containers(&self) -> Vec<JsValue> {
        let containers = self.containers_getter_impl();
        let containers = Array::from(&containers);
        containers.to_vec()
    }

    /// Sets the list of active containers for dragging. This overrides the
    /// list that is currently there.
    pub fn set_containers<T>(&mut self, objs: &[T])
    where
        T: JsCast + Clone,
    {
        let obj_array =
            objs.iter().cloned().map(|o| JsValue::from(&o)).collect();
        self.containers_setter_impl(obj_array);
    }

    /// Adds to the list of active containers for dragging
    ///
    /// Requires that feature `js-sys` be turned on (it is on by default)
    #[cfg(feature = "js-sys")]
    pub fn add_container<T>(&mut self, obj: T)
    where
        T: JsCast,
    {
        let mut containers = self.containers();
        let container_to_add = JsValue::from(&obj);
        containers.push(container_to_add);
        self.set_containers(&containers);
    }

    /// Enter drag mode **without a shadow**. This function is most useful when
    /// providing complementary keyboard shortcuts to an existing drag and drop
    /// solution. Even though a shadow won't be created at first, the user will
    /// get one as soon as they click on `item` and start dragging it around.
    /// Note that if they click and drag something else, `end` will be called
    /// before picking up the new item.
    pub fn start<T>(&mut self, item: &T)
    where
        T: JsCast,
    {
        let item = JsValue::from(item);
        self.start_impl(item);
    }

    /// Returns whether the `Drake` instance can accept drags for a DOM element
    /// `item`. This function returns `true` when all the conditions outlined
    /// below are met, and `false` otherwise.
    ///
    /// - `item` is a child of one of the specified containers for `Drake`
    /// - `item` passes the pertinent [`invalid`](crate::Options::invalid) checks
    /// - `item` passes a `moves` check
    pub fn can_move<T>(&self, item: &T) -> bool
    where
        T: JsCast,
    {
        let item = JsValue::from(item);
        self.can_move_impl(item)
    }

    /// Sets callback for `drag` event.
    /// Callback will be passed arguments `(el, source)`
    /// The `drag` event implies that
    /// `el` was lifted from `source`.
    pub fn on_drag<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue),
    {
        const EVENT_NAME: &str = "drag";

        let listener = closure::to_js_2(listener);

        self.on(EVENT_NAME, listener);
    }

    /// Sets callback for `dragend` event.
    /// Callback will be passed argument `(el)`
    /// The `dragend` event implies that
    /// dragging event for `el` ended with either `cancel`, `remove`, or `drop`.
    pub fn on_dragend<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue),
    {
        const EVENT_NAME: &str = "dragend";

        let listener = closure::to_js_1(listener);

        self.on(EVENT_NAME, listener);
    }

    /// Sets callback for `drop` event.
    /// Callback will be passed arguments `(el, target, source, sibling)`
    /// The `drop` event implies that
    /// `el` was dropped into `target` before a `sibling` element, and
    /// originally came from `source`.
    pub fn on_drop<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "drop";

        let listener = closure::to_js_4(listener);

        self.on(EVENT_NAME, listener);
    }

    /// Sets callback for `cancel` event.
    /// Callback will be passed argument `(el, container, source)`
    /// The `cancel` event implies that
    /// `el` was being dragged but it got nowhere and went back into
    /// `container`, its last stable parent; `el` originally came from `source`.
    pub fn on_cancel<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "cancel";

        let listener = closure::to_js_3(listener);

        self.on(EVENT_NAME, listener);
    }

    /// Sets callback for `remove` event.
    /// Callback will be passed argument `(el, container, source)`
    /// The `remove` event implies that
    /// `el` was being dragged but it got nowhere and it was removed from the
    /// DOM. Its last stable parent was `container`, and originally came from
    /// `source`.
    pub fn on_remove<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "remove";

        let listener = closure::to_js_3(listener);

        self.on(EVENT_NAME, listener);
    }

    /// Sets callback for `shadow` event.
    /// Callback will be passed argument `(el, container, source)`
    /// The `shadow` event implies that
    /// `el`, _the visual aid shadow_, was moved into `container`. May trigger
    /// many times as the position of `el` changes, even within the same
    /// `container`; `el` originally came from `source`.
    pub fn on_shadow<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "shadow";

        let listener = closure::to_js_3(listener);

        self.on(EVENT_NAME, listener);
    }

    /// Sets callback for `over` event.
    /// Callback will be passed argument `(el, container, source)`
    /// The `over` event implies that
    /// `el` is over `container`, and originally came from `source`.
    pub fn on_over<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "over";

        let listener = closure::to_js_3(listener);

        self.on(EVENT_NAME, listener);
    }

    /// Sets callback for `out` event.
    /// Callback will be passed argument `(el, container, source)`
    /// The `out` event implies that
    /// `el` was dragged out of `container` or dropped, and originally came from
    /// `source`.
    pub fn on_out<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "out";

        let listener = closure::to_js_3(listener);

        self.on(EVENT_NAME, listener);
    }

    /// Sets callback for `cloned` event.
    /// Callback will be passed argument `(clone, original, type)`
    /// The `cloned` event implies that
    /// DOM element `original` was cloned as `clone`, of `type` _(`'mirror'` or
    /// `'copy'`)_. Fired for mirror images and when `copy: true`.
    pub fn on_cloned<F: 'static>(&mut self, listener: F)
    where
        F: FnMut(JsValue, JsValue, JsValue),
    {
        const EVENT_NAME: &str = "cloned";

        let listener = closure::to_js_3(listener);

        self.on(EVENT_NAME, listener);
    }
}

#[cfg(test)]
pub mod test;
