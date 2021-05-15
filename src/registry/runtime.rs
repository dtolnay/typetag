use std::sync::RwLock;

use crate::{DeserializeFn, DeserializerRegistry};
use serde::de::{self, Expected};

use super::common::Registry as InnerRegistry;

pub struct Registry<T: ?Sized>(RwLock<InnerRegistry<T>>);

impl<T: ?Sized> Default for Registry<T> {
    #[must_use]
    fn default() -> Self {
        Registry(RwLock::new(InnerRegistry::default()))
    }
}

impl<T: ?Sized> std::ops::Deref for Registry<T> {
    type Target = RwLock<InnerRegistry<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DeserializerRegistry<T> for Registry<T> {
    fn get_deserializer<E>(
        &'static self,
        key: &str,
        expected: &dyn Expected,
    ) -> Result<DeserializeFn<T>, E>
    where
        E: serde::de::Error,
    {
        let registry = self
            .read()
            .map_err(|_| de::Error::custom("Unable to acquire lock, registry lock poisoned."))?;

        match registry.map.get(key) {
            Some(Some(value)) => Ok(*value),
            Some(None) => Err(de::Error::custom(format_args!(
                "non-unique tag of {}: {:?}",
                expected, key
            ))),
            None => Err(de::Error::unknown_variant(
                key,
                &["dynamic list, unable to list variants"],
            )),
        }
    }
}
