use crate::content::Content;
use crate::de::{FnApply, MapLookupVisitor};
use crate::ser::Wrap;
use crate::Registry;
use serde::de::{
    self, DeserializeSeed, Deserializer, IgnoredAny, IntoDeserializer, MapAccess, SeqAccess,
    Visitor,
};
use serde::ser::{SerializeStruct, Serializer};
use std::fmt;

pub fn serialize<S, T>(
    serializer: S,
    trait_object: &'static str,
    tag: &'static str,
    variant: &'static str,
    content: &'static str,
    concrete: &T,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ?Sized + erased_serde::Serialize,
{
    let mut ser = serializer.serialize_struct(trait_object, 2)?;
    ser.serialize_field(tag, variant)?;
    ser.serialize_field(content, &Wrap(concrete))?;
    ser.end()
}

pub fn deserialize<'de, D, T>(
    deserializer: D,
    trait_object: &'static str,
    fields: &'static [&'static str],
    registry: &'static Registry<T>,
) -> Result<Box<T>, D::Error>
where
    D: Deserializer<'de>,
    T: ?Sized,
{
    let visitor = TaggedVisitor {
        trait_object,
        tag: fields[0],
        content: fields[1],
        registry,
    };
    deserializer.deserialize_struct(trait_object, fields, visitor)
}

struct TaggedVisitor<T: ?Sized + 'static> {
    trait_object: &'static str,
    tag: &'static str,
    content: &'static str,
    registry: &'static Registry<T>,
}

impl<'de, T: ?Sized> Visitor<'de> for TaggedVisitor<T> {
    type Value = Box<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "dyn {}", self.trait_object)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let map_lookup = MapLookupVisitor {
            expected: &self,
            registry: self.registry,
        };

        let field_seed = TagContentOtherFieldVisitor {
            tag: self.tag,
            content: self.content,
        };

        let next_relevant_key = |map: &mut A| {
            while let Some(key) = map.next_key_seed(field_seed)? {
                match key {
                    TagContentOtherField::Tag => return Ok(Some(TagOrContentField::Tag)),
                    TagContentOtherField::Content => return Ok(Some(TagOrContentField::Content)),
                    TagContentOtherField::Other => {
                        map.next_value::<IgnoredAny>()?;
                        continue;
                    }
                }
            }
            Ok(None)
        };

        // Visit the first relevant key.
        let ret = match next_relevant_key(&mut map)? {
            // First key is the tag.
            Some(TagOrContentField::Tag) => {
                // Parse the tag.
                let deserialize_fn = map.next_value_seed(map_lookup)?;
                // Visit the second key.
                match next_relevant_key(&mut map)? {
                    // Second key is a duplicate of the tag.
                    Some(TagOrContentField::Tag) => {
                        return Err(de::Error::duplicate_field(self.tag));
                    }
                    // Second key is the content.
                    Some(TagOrContentField::Content) => {
                        let fn_apply = FnApply { deserialize_fn };
                        map.next_value_seed(fn_apply)?
                    }
                    // There is no second key; might be okay if the we have a unit variant.
                    None => {
                        let fn_apply = FnApply { deserialize_fn };
                        let unit = ().into_deserializer();
                        return fn_apply.deserialize(unit);
                    }
                }
            }
            // First key is the content.
            Some(TagOrContentField::Content) => {
                // Buffer up the content.
                let content = map.next_value::<Content>()?;
                // Visit the second key.
                match next_relevant_key(&mut map)? {
                    // Second key is the tag.
                    Some(TagOrContentField::Tag) => {
                        // Parse the tag.
                        let deserialize_fn = map.next_value_seed(map_lookup)?;
                        let fn_apply = FnApply { deserialize_fn };
                        let content = content.into_deserializer();
                        fn_apply.deserialize(content)?
                    }
                    // Second key is a duplicate of the content.
                    Some(TagOrContentField::Content) => {
                        return Err(de::Error::duplicate_field(self.content));
                    }
                    // There is no second key.
                    None => return Err(de::Error::missing_field(self.tag)),
                }
            }
            // There is no first key.
            None => return Err(de::Error::missing_field(self.tag)),
        };

        match next_relevant_key(&mut map)? {
            Some(TagOrContentField::Tag) => Err(de::Error::duplicate_field(self.tag)),
            Some(TagOrContentField::Content) => Err(de::Error::duplicate_field(self.content)),
            None => Ok(ret),
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let map_lookup = MapLookupVisitor {
            expected: &self,
            registry: self.registry,
        };

        // Visit the first element - the tag.
        let deserialize_fn = match seq.next_element_seed(map_lookup)? {
            Some(deserialize_fn) => deserialize_fn,
            None => return Err(de::Error::invalid_length(0, &self)),
        };

        // Visit the second element - the content.
        let fn_apply = FnApply { deserialize_fn };
        match seq.next_element_seed(fn_apply)? {
            Some(ret) => Ok(ret),
            None => Err(de::Error::invalid_length(1, &self)),
        }
    }
}

enum TagOrContentField {
    Tag,
    Content,
}

enum TagContentOtherField {
    Tag,
    Content,
    Other,
}

#[derive(Copy, Clone)]
struct TagContentOtherFieldVisitor {
    tag: &'static str,
    content: &'static str,
}

impl<'de> DeserializeSeed<'de> for TagContentOtherFieldVisitor {
    type Value = TagContentOtherField;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for TagContentOtherFieldVisitor {
    type Value = TagContentOtherField;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{:?}, {:?}, or other ignored fields",
            self.tag, self.content
        )
    }

    fn visit_str<E>(self, field: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if field == self.tag {
            Ok(TagContentOtherField::Tag)
        } else if field == self.content {
            Ok(TagContentOtherField::Content)
        } else {
            Ok(TagContentOtherField::Other)
        }
    }
}
