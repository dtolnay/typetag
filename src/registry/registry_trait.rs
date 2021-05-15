use crate::DeserializeFn;
use serde::de::Expected;

pub trait DeserializerRegistry<T: ?Sized> {
    fn get_deserializer<E>(
        &'static self,
        key: &str,
        expected: &dyn Expected,
    ) -> Result<DeserializeFn<T>, E>
    where
        E: serde::de::Error;
}
