#[doc(hidden)]
pub use typetag_impl::*;

// Not public API. Used by generated code.
#[doc(hidden)]
pub use inventory;

// Not public API. Used by generated code.
#[doc(hidden)]
pub use erased_serde;

// Not public API. Used by generated code.
#[doc(hidden)]
pub use serde;

// Not public API. Used by generated code.
#[doc(hidden)]
pub use lazy_static;

// Not public API. Used by generated code.
#[doc(hidden)]
pub mod externally;

// Not public API. Used by generated code.
#[doc(hidden)]
pub mod internally;

// Not public API. Used by generated code.
#[doc(hidden)]
pub mod adjacently;

mod content;
mod de;
mod ser;

// Not public API. Used by generated code.
#[doc(hidden)]
pub type DeserializeFn<T> = fn(&mut erased_serde::Deserializer) -> erased_serde::Result<Box<T>>;

use std::collections::BTreeMap;

// Not public API. Used by generated code.
#[doc(hidden)]
pub struct Registry<T: ?Sized> {
    pub map: BTreeMap<&'static str, DeserializeFn<T>>,
    pub names: Vec<&'static str>,
}
