//! Drag and drop so simple it hurts
//!
//! Wrapper for the [Dragula](https://bevacqua.github.io/dragula/) Javascript library.
//!
//! ## Usage
//! Dragula provides the easiest possible API to make drag and drop a breeze in your
//! applications.
//! ```no_run
//! use dragula::*;
//!
//! let doc = web_sys::window().unwrap().document().unwrap();
//! let element = doc.get_element_by_id("drag-container").unwrap();
//!
//! let drake = dragula(&[element]);
//!
//! ```
//!
//! You can also provide an [`Options`](crate::Options) instance to specify
//! behaviour of certain drag-and-drop features.
//! ```no_run
//! use dragula::*;
//! use dragula::options::Direction;
//! use web_sys::Element;
//! # use wasm_bindgen::JsValue;
//!
//! # let element = JsValue::TRUE;
//! //--snip--
//!
//! let options = Options {
//!     is_container: Box::new(|el| {
//!         Element::from(el).class_list().contains("drag-container")
//!     }),
//!     direction: Direction::Horizontal,
//!     revert_on_spill: true,
//!     ..Options::default()
//! };
//!
//! let drake = dragula_options(&[element], options);
//!
//! //--snip--
//!
//! ```
//!
//! ## `cargo` Features
//! - **js-sys**: On by default. Can be used to toggle dependencies on the `js-sys`
//!   crate. Most of this crate relies solely on `wasm-bindgen`, so disabling this
//!   feature currently just prevents you from getting containers on an existing
//!   Drake. The main reason you might want to disable this would be to improve compile
//!   times.
#![doc(
    html_favicon_url = "https://bevacqua.github.io/dragula/resources/icon.svg"
)]
#![doc(html_logo_url = "https://bevacqua.github.io/dragula/resources/icon.svg")]

mod dragula;
mod drake;
pub mod options;

// Helpers
mod closure;

#[doc(inline)]
pub use crate::dragula::*;

#[doc(inline)]
pub use drake::Drake;

#[doc(inline)]
pub use options::Options;

#[cfg(test)]
mod test_utils;
