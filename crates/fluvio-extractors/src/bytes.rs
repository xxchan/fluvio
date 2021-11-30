use bytes::Bytes;
use fluvio_dataplane_protocol::record::RecordData;
use crate::FromBytes;
use crate::traits::IntoBytes;

// impl FromBytes //////////////////////////////////////////////////////////////

impl<'a> FromBytes<'a> for Bytes {
    type Error = std::convert::Infallible;
    type Inner = Bytes;

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn into_inner(self) -> Self::Inner {
        self
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        Ok(bytes.clone())
    }
}

impl<'a> FromBytes<'a> for &'a [u8] {
    type Error = std::convert::Infallible;
    type Inner = &'a [u8];

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn into_inner(self) -> Self::Inner {
        self
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        Ok(bytes.as_ref())
    }
}

impl<'a> FromBytes<'a> for Vec<u8> {
    type Error = std::convert::Infallible;
    type Inner = Vec<u8>;

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn into_inner(self) -> Self::Inner {
        self
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        Ok(bytes.to_vec())
    }
}

impl<'a> FromBytes<'a> for RecordData {
    type Error = std::convert::Infallible;
    type Inner = RecordData;

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn into_inner(self) -> Self::Inner {
        self
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        Ok(RecordData::from_bytes(bytes))
    }
}

// impl IntoBytes //////////////////////////////////////////////////////////////

impl IntoBytes for Bytes {
    type Error = std::convert::Infallible;

    fn into_bytes(self) -> Result<Bytes, Self::Error> {
        Ok(self)
    }
}

impl IntoBytes for Vec<u8> {
    type Error = std::convert::Infallible;

    fn into_bytes(self) -> Result<Bytes, Self::Error> {
        Ok(Bytes::from(self))
    }
}

impl<'a> IntoBytes for &'a [u8] {
    type Error = std::convert::Infallible;

    fn into_bytes(self) -> Result<Bytes, Self::Error> {
        Vec::from(self).into_bytes()
    }
}
