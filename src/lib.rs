//! **Serde serializable and deserializable trait objects.**
//!
//! This crate provides a macro for painless serialization of `&dyn Trait` trait
//! objects and serialization + deserialization of `Box<dyn Trait>` trait
//! objects.
//!
//! Let's dive into the example and I'll explain some more below.
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
pub type DeserializeFn<T> = fn(&mut erased_serde::Deserializer) -> erased_serde::Result<Box<T>>;

use std::collections::BTreeMap;

// Not public API. Used by generated code.
#[doc(hidden)]
pub struct Registry<T: ?Sized> {
    pub map: BTreeMap<&'static str, Option<DeserializeFn<T>>>,
    pub names: Vec<&'static str>,
}
