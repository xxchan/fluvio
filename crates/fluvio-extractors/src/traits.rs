use bytes::Bytes;
use std::error::Error as StdError;
use fluvio_dataplane_protocol::record::Record;

/// A trait for types that may extract themselves from a `Record`
pub trait FromRecord<'a>: Sized {
    type Error: StdError + Send + Sync + 'static;

    fn from_record(record: &'a Record) -> Result<Self, Self::Error>;
}

/// A trait for types that may extract themselves from `&Bytes`
pub trait FromBytes<'a>: Sized {
    type Error: StdError + Send + Sync + 'static;
    type Inner;

    fn inner(&self) -> &Self::Inner;
    fn into_inner(self) -> Self::Inner;
    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error>;
}

/// A trait for types that may write themselves to `Bytes`.
pub trait IntoBytes {
    type Error: StdError + Send + Sync + 'static;

    fn into_bytes(self) -> Result<Bytes, Self::Error>;
}

pub trait IntoRecord {
    type Error: StdError + Send + Sync + 'static;

    fn into_record(self, record: &mut Record) -> Result<(), Self::Error>;
}

// IMPLS ///////////////////////////////////////////////////////////////////////

impl<'a> FromRecord<'a> for &'a Record {
    type Error = std::convert::Infallible;

    fn from_record(record: &'a Record) -> Result<Self, Self::Error> {
        Ok(record)
    }
}

impl IntoRecord for Record {
    type Error = std::convert::Infallible;

    fn into_record(self, record: &mut Record) -> Result<(), Self::Error> {
        *record = self;
        Ok(())
    }
}
