#![doc(
    html_favicon_url = "https://bevacqua.github.io/dragula/resources/icon.svg"
)]
#![doc(html_logo_url = "https://bevacqua.github.io/dragula/resources/icon.svg")]

//! Drag and drop so simple it hurts
//!
//! Wrapper for the [Dragula](https://bevacqua.github.io/dragula/) Javascript library.

mod dragula;
mod drake;
mod options;

// Helpers
mod closure;

#[doc(inline)]
pub use crate::dragula::*;

#[doc(inline)]
pub use drake::Drake;

#[doc(inline)]
pub use options::*;
