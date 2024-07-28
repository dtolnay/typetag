//! Check if a Serialize is a specific &'static str.
//!
//! This is done by implementing a Serializer whose entire purpose is to check
//! whether a single method, `serialize_str`, is called with a given string.

use core::fmt::Error;
use serde::{ser, Serialize};

pub fn is_serialize_str<T: ?Sized + Serialize>(value: &T, expected_str: &'static str) -> bool {
    let mut ser = Serializer::new(expected_str);
    let _ = value.serialize(&mut ser);
    ser.state == SerializerState::GotExpectedStr
}

#[derive(PartialEq)]
enum SerializerState {
    Start,
    GotExpectedStr,
    GotUnexpected,
}

struct Serializer {
    pub expected_str: &'static str,
    pub state: SerializerState,
}

impl Serializer {
    fn new(expected_str: &'static str) -> Serializer {
        Serializer {
            expected_str,
            state: SerializerState::Start,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn unexpected(&mut self) -> Result<(), Error> {
        self.state = SerializerState::GotUnexpected;
        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    fn unexpected2(&mut self) -> Result<&mut Self, Error> {
        self.state = SerializerState::GotUnexpected;
        Ok(self)
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_i8(self, _: i8) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_i16(self, _: i16) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_i32(self, _: i32) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_i64(self, _: i64) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_u8(self, _: u8) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_u16(self, _: u16) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_u32(self, _: u32) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_u64(self, _: u64) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_f32(self, _: f32) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_f64(self, _: f64) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_char(self, _: char) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        if self.state == SerializerState::Start && v == self.expected_str {
            self.state = SerializerState::GotExpectedStr;
        } else {
            self.state = SerializerState::GotUnexpected;
        }
        Ok(())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_none(self) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_unit(self) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.unexpected()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self, Error> {
        self.unexpected2()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Error> {
        self.unexpected()
    }

    fn end(self) -> Result<(), Error> {
        self.unexpected()
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Error> {
        self.unexpected()
    }

    fn end(self) -> Result<(), Error> {
        self.unexpected()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Error> {
        self.unexpected()
    }

    fn end(self) -> Result<(), Error> {
        self.unexpected()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Error> {
        self.unexpected()
    }

    fn end(self) -> Result<(), Error> {
        self.unexpected()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Error> {
        self.unexpected()
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Error> {
        self.unexpected()
    }

    fn end(self) -> Result<(), Error> {
        self.unexpected()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _: &'static str,
        _: &T,
    ) -> Result<(), Error> {
        self.unexpected()
    }

    fn end(self) -> Result<(), Error> {
        self.unexpected()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _: &'static str,
        _: &T,
    ) -> Result<(), Error> {
        self.unexpected()
    }

    fn end(self) -> Result<(), Error> {
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
