# dragula

[![](http://meritbadge.herokuapp.com/dragula)](https://crates.io/crates/dragula)
![GitHub](https://img.shields.io/github/license/Xavientois/dragula-rs)
![GitHub Workflow Status](https://github.com/Xavientois/dragula-rs/workflows/tests/badge.svg)

Drag and drop so simple it hurts

Wrapper for the [Dragula](https://bevacqua.github.io/dragula/) Javascript library.

## Usage
Dragula provides the easiest possible API to make drag and drop a breeze in your
applications.
```rust
use dragula::*;

let doc = web_sys::window().unwrap().document().unwrap();
let element = doc.get_element_by_id("drag-container").unwrap();

let drake = dragula(&[element]);

```

You can also provide an `Options` instance to specify
behaviour of certain drag-and-drop features.
```rust
use dragula::*;
use dragula::options::Direction;
use web_sys::Element;

//--snip--

let options = Options {
    is_container: Box::new(|el| {
        Element::from(el).class_list().contains("drag-container")
    }),
    direction: Direction::Horizontal,
    revert_on_spill: true,
    ..Options::default()
};

let drake = dragula_options(&[element], options);

//--snip--

```

## `cargo` Features
- **js-sys**: On by default. Can be used to toggle dependencies on the `js-sys`
  crate. Most of this crate relies solely on `wasm-bindgen`, so disabling this
  feature currently just prevents you from getting containers on an existing
  Drake. The main reason you might want to disable this would be to improve compile
  times.

License: MIT
