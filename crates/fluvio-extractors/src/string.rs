use std::io::{Error as IoError, ErrorKind};
use std::str::{from_utf8, FromStr, Utf8Error};
use std::string::FromUtf8Error;
use bytes::Bytes;
use crate::FromBytes;

pub struct Parse<T>(pub T);

impl<'a, T: FromStr> FromBytes<'a> for Parse<T>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Error = IoError;
    type Inner = T;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn into_inner(self) -> Self::Inner {
        self.0
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        let string =
            from_utf8(bytes.as_ref()).map_err(|e| IoError::new(ErrorKind::InvalidData, e))?;
        let it = T::from_str(string).map_err(|e| IoError::new(ErrorKind::InvalidData, e))?;
        Ok(Self(it))
    }
}

impl<'a> FromBytes<'a> for String {
    type Error = FromUtf8Error;
    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn into_inner(self) -> Self::Inner {
        self
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        let string = String::from_utf8(bytes.to_vec())?;
        Ok(string)
    }
}

impl<'a> FromBytes<'a> for &'a str {
    type Error = Utf8Error;
    type Inner = &'a str;

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn into_inner(self) -> Self::Inner {
        self
    }

    fn from_bytes(bytes: &'a Bytes) -> Result<Self, Self::Error> {
        let string = std::str::from_utf8(bytes.as_ref())?;
        Ok(string)
    }
}
