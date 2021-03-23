//! [![github]](https://github.com/dtolnay/typetag)&ensp;[![crates-io]](https://crates.io/crates/typetag)&ensp;[![docs-rs]](https://docs.rs/typetag)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! **Serde serializable and deserializable trait objects.**
//!
//! This crate provides a macro for painless serialization of `&dyn Trait` trait
//! objects and serialization + deserialization of `Box<dyn Trait>` trait
//! objects.
//!
//! Let's dive into the example and I'll explain some more below.
//!
//! <br>
//!
//! # Example
//!
//! Suppose I have a trait `WebEvent` and I require that every implementation of
//! the trait be serializable and deserializable so that I can send them to my
//! ad-serving AI. Here are just the types and trait impls to start with:
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! #
//! trait WebEvent {
//!     fn inspect(&self);
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct PageLoad;
//!
//! impl WebEvent for PageLoad {
//!     fn inspect(&self) {
//!         println!("200 milliseconds or bust");
//!     }
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct Click {
//!     x: i32,
//!     y: i32,
//! }
//!
//! impl WebEvent for Click {
//!     fn inspect(&self) {
//!         println!("negative space between the ads: x={} y={}", self.x, self.y);
//!     }
//! }
//! ```
//!
//! We'll need to be able to send an arbitrary web event as JSON to the AI:
//!
//! ```
//! # use serde::{Serialize, Serializer};
//! # use serde_json::Result;
//! #
//! # trait WebEvent {}
//! #
//! # impl<'a> Serialize for dyn WebEvent + 'a {
//! #     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//! #     where
//! #         S: Serializer,
//! #     {
//! #         unimplemented!()
//! #     }
//! # }
//! #
//! # fn somehow_send_json(json: String) -> Result<()> {
//! #     unimplemented!()
//! # }
//! #
//! fn send_event_to_money_factory(event: &dyn WebEvent) -> Result<()> {
//!     let json = serde_json::to_string(event)?;
//!     somehow_send_json(json)?;
//!     Ok(())
//! }
//! ```
//!
//! and receive an arbitrary web event as JSON on the server side:
//!
//! ```
//! # use serde::{Deserialize, Deserializer};
//! # use serde_json::Result;
//! #
//! # trait WebEvent {}
//! #
//! # impl<'de> Deserialize<'de> for Box<dyn WebEvent> {
//! #     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//! #     where
//! #         D: Deserializer<'de>,
//! #     {
//! #         unimplemented!()
//! #     }
//! # }
//! #
//! # fn overanalyze(event: Box<dyn WebEvent>) -> Result<()> {
//! #     unimplemented!()
//! # }
//! #
//! fn process_event_from_clickfarm(json: &str) -> Result<()> {
//!     let event: Box<dyn WebEvent> = serde_json::from_str(json)?;
//!     overanalyze(event)?;
//!     Ok(())
//! }
//! ```
//!
//! The introduction claimed that this would be painless but I'll let you be the
//! judge.
//!
//! First stick an attribute on top of the trait.
//!
//! ```
//! #[typetag::serde(tag = "type")]
//! trait WebEvent {
//!     fn inspect(&self);
//! }
//! ```
//!
//! Then stick a similar attribute on all those impl blocks too.
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! #
//! # #[typetag::serde(tag = "type")]
//! # trait WebEvent {
//! #     fn inspect(&self);
//! # }
//! #
//! # #[derive(Serialize, Deserialize)]
//! # struct PageLoad;
//! #
//! # #[derive(Serialize, Deserialize)]
//! # struct Click {
//! #     x: i32,
//! #     y: i32,
//! # }
//! #
//! #[typetag::serde]
//! impl WebEvent for PageLoad {
//!     fn inspect(&self) {
//!         println!("200 milliseconds or bust");
//!     }
//! }
//!
//! #[typetag::serde]
//! impl WebEvent for Click {
//!     fn inspect(&self) {
//!         println!("negative space between the ads: x={} y={}", self.x, self.y);
//!     }
//! }
//! ```
//!
//! And now it works as described. All in all, three lines were added!
//!
//! <br>
//!
//! # What?
//!
//! Trait objects are serialized by this library like Serde enums. Every impl of
//! the trait (anywhere in the program) looks like one variant of the enum.
//!
//! All three of Serde's tagged [enum representations] are supported. The one
//! shown above is the "internally tagged" style so our two event types would be
//! represented in JSON as:
//!
//! [enum representations]: https://serde.rs/enum-representations.html
//!
//! ```json
//! {"type":"PageLoad"}
//! {"type":"Click","x":10,"y":10}
//! ```
//!
//! The choice of enum representation is controlled by the attribute that goes
//! on the trait definition. Let's check out the "adjacently tagged" style:
//!
//! ```
//! #[typetag::serde(tag = "type", content = "value")]
//! trait WebEvent {
//!     fn inspect(&self);
//! }
//! ```
//!
//! ```json
//! {"type":"PageLoad","value":null}
//! {"type":"Click","value":{"x":10,"y":10}}
//! ```
//!
//! and the "externally tagged" style, which is Serde's default for enums:
//!
//! ```
//! #[typetag::serde]
//! trait WebEvent {
//!     fn inspect(&self);
//! }
//! ```
//!
//! ```json
//! {"PageLoad":null}
//! {"Click":{"x":10,"y":10}}
//! ```
//!
//! Separately, the value of the tag for a given trait impl may be defined as
//! part of the attribute that goes on the trait impl. By default the tag will
//! be the type name when no name is specified explicitly.
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! #
//! # #[typetag::serde]
//! # trait WebEvent {
//! #     fn inspect(&self);
//! # }
//! #
//! # #[derive(Serialize, Deserialize)]
//! # struct Click {
//! #     x: i32,
//! #     y: i32,
//! # }
//! #
//! #[typetag::serde(name = "mouse_button_down")]
//! impl WebEvent for Click {
//!     fn inspect(&self) {
//!         println!("negative space between the ads: ({}, {})", self.x, self.y);
//!     }
//! }
//! ```
//!
//! ```json
//! {"type":"mouse_button_down","x":10,"y":10}
//! ```
//!
//! Conceptually all you're getting with this crate is that we build for you an
//! enum in which every impl of the trait in your program is automatically
//! registered as an enum variant. The behavior is the same as if you had
//! written the enum yourself and implemented Serialize and Deserialize for the
//! dyn Trait object in terms of the enum.
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! #
//! # #[derive(Serialize, Deserialize)]
//! # struct PageLoad;
//! #
//! # #[derive(Serialize, Deserialize)]
//! # struct Click;
//! #
//! // generated (conceptually)
//! #[derive(Serialize, Deserialize)]
//! enum WebEvent {
//!     PageLoad(PageLoad),
//!     Click(Click),
//!     /* ... */
//! }
//! ```
//!
//! <br>
//!
//! # So many questions
//!
//! - *Does it work if the trait impls are spread across different crates?*
//!   **Yes**
//!
//!   Serialization and deserialization both support every single impl of the
//!   trait across the dependency graph of the final program binary.
//!
//! - *Does it work in non-self-describing data formats like Bincode?* **Yes**
//!
//!   All three choices of enum representation will round-trip correctly through
//!   compact binary formats including Bincode.
//!
//! - *Does it support non-struct types?* **Yes**
//!
//!   The implementations of the trait can be structs, enums, primitives, or
//!   anything else supported by Serde. The Serialize and Deserialize impls may
//!   be derived or handwritten.
//!
//! - *Didn't someone explain to me why this wasn't possible?* **Yes**
//!
//!   It might have been me.
//!
//! - *Then how does it work?*
//!
//!   We use the [`inventory`] crate to produce a registry of impls of your
//!   trait, which is built on the [`ctor`] crate to hook up initialization
//!   functions that insert into the registry. The first `Box<dyn Trait>`
//!   deserialization will perform the work of iterating the registry and
//!   building a map of tags to deserialization functions. Subsequent
//!   deserializations find the right deserialization function in that map. The
//!   [`erased-serde`] crate is also involved, to do this all in a way that does
//!   not break object safety.
//!
//! [`inventory`]: https://github.com/dtolnay/inventory
//! [`ctor`]: https://github.com/mmastrac/rust-ctor
//! [`erased-serde`]: https://github.com/dtolnay/erased-serde

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::unnested_or_patterns
)]

#[doc(hidden)]
pub use typetag_impl::*;

// Not public API. Used by generated code.
#[doc(hidden)]
pub use inventory;

// Not public API. Used by generated code.
#[doc(hidden)]
pub use erased_serde;

// Not public API. Used by generated code.
//
// Extern crate because on Rust 1.31 a plain `use` would conflict with the
// `serde` attribute macro re-exported from typetag_impl.
#[doc(hidden)]
pub extern crate serde;

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
pub type DeserializeFn<T> = fn(&mut dyn erased_serde::Deserializer) -> erased_serde::Result<Box<T>>;

use std::collections::BTreeMap;

// Not public API. Used by generated code.
#[doc(hidden)]
pub struct Registry<T: ?Sized> {
    pub map: BTreeMap<&'static str, Option<DeserializeFn<T>>>,
    pub names: Vec<&'static str>,
}

// Object-safe trait bound inserted by typetag serialization. We want this just
// so the serialization requirement appears on rustdoc's view of your trait.
// Otherwise not public API.
#[doc(hidden)]
pub trait Serialize: erased_serde::Serialize {}

impl<T: ?Sized> Serialize for T where T: erased_serde::Serialize {}

// Object-safe trait bound inserted by typetag deserialization. We want this
// just so the serialization requirement appears on rustdoc's view of your
// trait. Otherwise not public API.
#[doc(hidden)]
pub trait Deserialize {}

impl<T> Deserialize for T {}

// Not public API.
#[doc(hidden)]
pub trait Strictest {
    type Object: ?Sized;
}
