//! Check if a Serialize is a specific &'static str.
//!
//! This is done by implementing a Serializer whose entire purpose is to check
//! whether a single method, `serialize_str`, is called with a given string.

use core::fmt::{self, Display};
use serde::{ser, Serialize};

pub fn is_serialize_str<T: ?Sized + Serialize>(value: &T, expected_str: &'static str) -> bool {
    let mut ser = Serializer::new(expected_str);
    match value.serialize(&mut ser) {
        Ok(void) => match void {},
        Err(SerializerState::GotExpectedStr) => true,
        Err(SerializerState::GotUnexpected) => false,
    }
}

enum Void {}

#[derive(Debug)]
enum SerializerState {
    GotExpectedStr,
    GotUnexpected,
}

impl serde::ser::Error for SerializerState {
    fn custom<M: Display>(_message: M) -> Self {
        SerializerState::GotUnexpected
    }
}

impl serde::ser::StdError for SerializerState {}

impl Display for SerializerState {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

struct Serializer {
    expected_str: &'static str,
}

impl Serializer {
    fn new(expected_str: &'static str) -> Serializer {
        Serializer { expected_str }
    }

    #[allow(clippy::unused_self)]
    fn unexpected<Any>(&mut self) -> Result<Any, SerializerState> {
        Err(SerializerState::GotUnexpected)
    }
}

impl ser::Serializer for &mut Serializer {
    type Ok = Void;
    type Error = SerializerState;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if v == self.expected_str {
            Err(SerializerState::GotExpectedStr)
        } else {
            self.unexpected()
        }
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.unexpected()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.unexpected()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.unexpected()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.unexpected()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.unexpected()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.unexpected()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.unexpected()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.unexpected()
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = Void;
    type Error = SerializerState;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        self.unexpected()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = Void;
    type Error = SerializerState;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        self.unexpected()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = Void;
    type Error = SerializerState;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        self.unexpected()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = Void;
    type Error = SerializerState;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        self.unexpected()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = Void;
    type Error = SerializerState;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        self.unexpected()
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        self.unexpected()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = Void;
    type Error = SerializerState;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _: &'static str,
        _: &T,
    ) -> Result<(), Self::Error> {
        self.unexpected()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }
}

impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = Void;
    type Error = SerializerState;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _: &'static str,
        _: &T,
    ) -> Result<(), Self::Error> {
        self.unexpected()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.unexpected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_serialize_str() {
        assert!(is_serialize_str("hello", "hello"));
        assert!(!is_serialize_str("hello", "bye"));
    }
}
