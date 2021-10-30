use fluvio_dataplane_protocol::record::RecordData;
use crate::FromBytes;

pub struct Slice<'a>(pub &'a [u8]);

impl<'a> FromBytes<'a> for Slice<'a> {
    type Error = std::convert::Infallible;

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Slice(bytes))
    }
}

impl From<Slice<'_>> for RecordData {
    fn from(slice: Slice) -> Self {
        RecordData::from(slice.0)
    }
}
