use std::io::{Error as IoError, ErrorKind};
use std::str::{from_utf8, FromStr};
use crate::FromBytes;

pub struct Parse<T>(pub T);

impl<'a, T: FromStr> FromBytes<'a> for Parse<T>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Error = IoError;

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let string = from_utf8(bytes).map_err(|e| IoError::new(ErrorKind::InvalidData, e))?;
        let it = T::from_str(string).map_err(|e| IoError::new(ErrorKind::InvalidData, e))?;
        Ok(Self(it))
    }
}
