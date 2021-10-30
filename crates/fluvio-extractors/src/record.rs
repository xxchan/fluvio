use bytes::Bytes;
use fluvio_dataplane_protocol::record::Record as FluvioRecord;
use crate::traits::{FromBytes, FromRecord};

#[derive(Debug)]
pub struct Record<K, V> {
    pub key: Option<K>,
    pub value: V,
}

#[derive(thiserror::Error, Debug)]
pub enum RecordError<K, V> {
    #[error("error deserializing key")]
    Key(#[source] K),
    #[error("error deserializing value")]
    Value(#[source] V),
}

impl<'a, K: FromBytes<'a>, V: FromBytes<'a>> FromRecord<'a> for Record<K, V> {
    type Error = RecordError<<K as FromBytes<'a>>::Error, <V as FromBytes<'a>>::Error>;

    fn from_record(record: &'a FluvioRecord) -> Result<Self, Self::Error> {
        let key = record
            .key
            .as_ref()
            .map(|k| K::from_bytes(k.inner()))
            .transpose()
            .map_err(RecordError::Key)?;
        let value = V::from_bytes(&record.value.inner()).map_err(RecordError::Value)?;
        Ok(Record { key, value })
    }
}

impl<'a, K: FromBytes<'a>, V: FromBytes<'a>> Record<K, V> {
    pub fn parts(&self) -> (Option<&K::Inner>, &V::Inner) {
        (self.key(), self.value())
    }

    pub fn into_parts(self) -> (Option<K::Inner>, V::Inner) {
        (self.key.map(|k| k.into_inner()), self.value.into_inner())
    }

    pub fn key(&self) -> Option<&K::Inner> {
        self.key.as_ref().map(|k| k.inner())
    }

    pub fn value(&self) -> &V::Inner {
        self.value.inner()
    }

    pub fn into_key(self) -> Option<K::Inner> {
        self.key.map(|k| k.into_inner())
    }

    pub fn into_value(self) -> V::Inner {
        self.value.into_inner()
    }
}

#[derive(Debug)]
pub struct Key<K>(pub Option<K>);

impl<'a, K: FromBytes<'a>> FromRecord<'a> for Key<K> {
    type Error = <K as FromBytes<'a>>::Error;

    fn from_record(record: &'a FluvioRecord) -> Result<Self, Self::Error> {
        let key = record
            .key
            .as_ref()
            .map(|k| K::from_bytes(k.inner()))
            .transpose()?;
        Ok(Self(key))
    }
}

impl<'a, K: FromBytes<'a>> Key<K> {
    pub fn into_inner(self) -> Option<K::Inner> {
        self.0.map(|k| k.into_inner())
    }
}

#[derive(Debug)]
pub struct Value<V>(pub V);

impl<'a, V: FromBytes<'a>> FromRecord<'a> for Value<V> {
    type Error = <V as FromBytes<'a>>::Error;

    fn from_record(record: &'a FluvioRecord) -> Result<Self, Self::Error> {
        let value = V::from_bytes(record.value.inner())?;
        Ok(Self(value))
    }
}

impl<'a, V: FromBytes<'a>> Value<V> {
    pub fn into_inner(self) -> V::Inner {
        self.0.into_inner()
    }
}

impl<'a, V: FromBytes<'a>> FromBytes<'a> for Value<V> {
    type Error = <V as FromBytes<'a>>::Error;
    type Inner = <V as FromBytes<'a>>::Inner;

    fn inner(&self) -> &Self::Inner {
        self.0.inner()
    }

    fn into_inner(self) -> Self::Inner {
        self.0.into_inner()
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        Ok(Self(V::from_bytes(bytes)?))
    }
}
