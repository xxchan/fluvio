use std::error::Error as StdError;
use fluvio_dataplane_protocol::record::Record;

pub trait FromRecord<'a>: Sized {
    type Error: StdError + Send + Sync + 'static;

    fn from_record(record: &'a Record) -> Result<Self, Self::Error>;
}

pub trait FromBytes<'a>: Sized {
    type Error: StdError + Send + Sync + 'static;
    type Inner;

    fn inner(&self) -> &Self::Inner;
    fn into_inner(self) -> Self::Inner;
    fn from_bytes(bytes: &'a [u8]) -> Result<Self, Self::Error>;
}

impl<'a> FromRecord<'a> for &'a Record {
    type Error = std::convert::Infallible;

    fn from_record(record: &'a Record) -> Result<Self, Self::Error> {
        Ok(record)
    }
}
