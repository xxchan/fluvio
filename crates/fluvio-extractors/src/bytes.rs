use crate::FromBytes;

impl<'a> FromBytes<'a> for &'a [u8] {
    type Error = std::convert::Infallible;
    type Inner = &'a [u8];

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn into_inner(self) -> Self::Inner {
        self
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(bytes)
    }
}
