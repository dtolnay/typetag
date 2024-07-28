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
    fn unexpected(&mut self) -> Res {
        self.state = SerializerState::GotUnexpected;
        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    fn unexpected2(&mut self) -> Result<&mut Self, Error> {
        self.state = SerializerState::GotUnexpected;
        Ok(self)
    }
}

type Res = Result<(), Error>;

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

    fn serialize_bool(self, _: bool) -> Res {
        self.unexpected()
    }

    fn serialize_i8(self, _: i8) -> Res {
        self.unexpected()
    }

    fn serialize_i16(self, _: i16) -> Res {
        self.unexpected()
    }

    fn serialize_i32(self, _: i32) -> Res {
        self.unexpected()
    }

    fn serialize_i64(self, _: i64) -> Res {
        self.unexpected()
    }

    fn serialize_u8(self, _: u8) -> Res {
        self.unexpected()
    }

    fn serialize_u16(self, _: u16) -> Res {
        self.unexpected()
    }

    fn serialize_u32(self, _: u32) -> Res {
        self.unexpected()
    }

    fn serialize_u64(self, _: u64) -> Res {
        self.unexpected()
    }

    fn serialize_f32(self, _: f32) -> Res {
        self.unexpected()
    }

    fn serialize_f64(self, _: f64) -> Res {
        self.unexpected()
    }

    fn serialize_char(self, _: char) -> Res {
        self.unexpected()
    }

    fn serialize_str(self, v: &str) -> Res {
        if self.state == SerializerState::Start && v == self.expected_str {
            self.state = SerializerState::GotExpectedStr;
        } else {
            self.state = SerializerState::GotUnexpected;
        }
        Ok(())
    }

    fn serialize_bytes(self, _: &[u8]) -> Res {
        self.unexpected()
    }

    fn serialize_none(self) -> Res {
        self.unexpected()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Res {
        self.unexpected()
    }

    fn serialize_unit(self) -> Res {
        self.unexpected()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Res {
        self.unexpected()
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Res {
        self.unexpected()
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _: &'static str, _: &T) -> Res {
        self.unexpected()
    }

    fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Res
    where
        T: ?Sized + Serialize,
    {
        self.unexpected()
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_tuple(self, _: usize) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self, Error> {
        self.unexpected2()
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self, Error> {
        self.unexpected2()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _: &T) -> Res {
        self.unexpected()
    }

    fn end(self) -> Res {
        self.unexpected()
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _: &T) -> Res {
        self.unexpected()
    }

    fn end(self) -> Res {
        self.unexpected()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &T) -> Res {
        self.unexpected()
    }

    fn end(self) -> Res {
        self.unexpected()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &T) -> Res {
        self.unexpected()
    }

    fn end(self) -> Res {
        self.unexpected()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _: &T) -> Res {
        self.unexpected()
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _: &T) -> Res {
        self.unexpected()
    }

    fn end(self) -> Res {
        self.unexpected()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &'static str, _: &T) -> Res {
        self.unexpected()
    }

    fn end(self) -> Res {
        self.unexpected()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &'static str, _: &T) -> Res {
        self.unexpected()
    }

    fn end(self) -> Res {
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
