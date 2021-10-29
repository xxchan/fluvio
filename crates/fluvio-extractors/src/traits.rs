use std::error::Error as StdError;
use fluvio_dataplane_protocol::record::Record;

pub trait FromRecord: Sized {
    type Error: StdError + Send + Sync + 'static;

    fn from_record(record: &Record) -> Result<Self, Self::Error>;
}

pub trait FromBytes: Sized {
    type Error: StdError + Send + Sync + 'static;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
}
