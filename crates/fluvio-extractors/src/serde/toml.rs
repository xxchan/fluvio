use bytes::Bytes;
use serde::{Serialize, Deserialize};
use crate::{FromBytes, IntoBytes};

#[derive(Debug, Serialize, Deserialize)]
pub struct Toml<T>(pub T);

impl<'a, T: Deserialize<'a>> FromBytes<'a> for Toml<T> {
    type Error = toml::de::Error;
    type Inner = T;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn into_inner(self) -> Self::Inner {
        self.0
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        let inner: T = toml::from_slice(bytes.as_ref())?;
        Ok(Self(inner))
    }
}

impl<T: Serialize> IntoBytes for Toml<T> {
    type Error = toml::ser::Error;

    fn into_bytes(self) -> Result<Bytes, Self::Error> {
        let vec = toml::to_vec(&self)?;
        Ok(Bytes::from(vec))
    }
}
