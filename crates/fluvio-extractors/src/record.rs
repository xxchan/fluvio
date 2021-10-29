use fluvio_dataplane_protocol::record::Record as FluvioRecord;
use crate::traits::{FromBytes, FromRecord};

#[derive(Debug)]
pub struct Record<K: FromBytes, V: FromBytes> {
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

impl<K: FromBytes, V: FromBytes> FromRecord for Record<K, V> {
    type Error = RecordError<<K as FromBytes>::Error, <V as FromBytes>::Error>;

    fn from_record(record: &FluvioRecord) -> Result<Self, Self::Error> {
        let key = record
            .key
            .as_ref()
            .map(|k| K::from_bytes(k.as_ref()))
            .transpose()
            .map_err(RecordError::Key)?;
        let value = V::from_bytes(record.value.as_ref()).map_err(RecordError::Value)?;
        Ok(Record { key, value })
    }
}

#[derive(Debug)]
pub struct Key<K: FromBytes>(pub Option<K>);

impl<K: FromBytes> FromRecord for Key<K> {
    type Error = <K as FromBytes>::Error;

    fn from_record(record: &FluvioRecord) -> Result<Self, Self::Error> {
        let key = record
            .key
            .as_ref()
            .map(|k| K::from_bytes(k.as_ref()))
            .transpose()?;
        Ok(Self(key))
    }
}

#[derive(Debug)]
pub struct Value<V>(pub V);

impl<V: FromBytes> FromRecord for Value<V> {
    type Error = <V as FromBytes>::Error;

    fn from_record(record: &FluvioRecord) -> Result<Self, Self::Error> {
        let value = V::from_bytes(record.value.as_ref())?;
        Ok(Self(value))
    }
}
