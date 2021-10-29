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
            .map(|k| K::from_bytes(k.as_ref()))
            .transpose()
            .map_err(RecordError::Key)?;
        let value = V::from_bytes(record.value.as_ref()).map_err(RecordError::Value)?;
        Ok(Record { key, value })
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
            .map(|k| K::from_bytes(k.as_ref()))
            .transpose()?;
        Ok(Self(key))
    }
}

#[derive(Debug)]
pub struct Value<V>(pub V);

impl<'a, V: FromBytes<'a>> FromRecord<'a> for Value<V> {
    type Error = <V as FromBytes<'a>>::Error;

    fn from_record(record: &'a FluvioRecord) -> Result<Self, Self::Error> {
        let value = V::from_bytes(record.value.as_ref())?;
        Ok(Self(value))
    }
}

impl<'a, V: FromBytes<'a>> FromBytes<'a> for Value<V> {
    type Error = <V as FromBytes<'a>>::Error;

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self(V::from_bytes(bytes)?))
    }
}
