use bytes::Bytes;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use crate::{FromBytes, IntoBytes};

#[derive(Debug, Serialize, Deserialize)]
pub struct Yaml<T>(pub T);

impl<'a, T: DeserializeOwned> FromBytes<'a> for Yaml<T> {
    type Error = serde_yaml::Error;
    type Inner = T;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn into_inner(self) -> Self::Inner {
        self.0
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        let inner: T = serde_yaml::from_slice(bytes.as_ref())?;
        Ok(Self(inner))
    }
}

impl<T: Serialize> IntoBytes for Yaml<T> {
    type Error = serde_yaml::Error;

    fn into_bytes(self) -> Result<Bytes, Self::Error> {
        let vec = serde_yaml::to_vec(&self)?;
        Ok(Bytes::from(vec))
    }
}
