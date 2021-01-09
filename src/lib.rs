//! Drag and drop so simple it hurts
//!
//! Wrapper for the [Dragula](https://bevacqua.github.io/dragula/) Javascript library.
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
