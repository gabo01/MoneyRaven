use uuid;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::marker::PhantomData;

pub struct DBId<T> {
    id: uuid::Bytes,
    marker: PhantomData<T>
}

impl<T> Serialize for DBId<T> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.id.serialize(ser)
    }
}

impl<'de, T> Deserialize<'de> for DBId<T> {
    fn deserialize<D: Deserializer<'de>>(de: D) -> std::result::Result<Self, D::Error> {
        Ok(Self {
            id: uuid::Bytes::deserialize(de)?,
            marker: PhantomData
        })
    }
}