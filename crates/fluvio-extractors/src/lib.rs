mod record;
mod traits;
mod string;
mod bytes;
mod serde;

pub use eyre::Error;
pub use record::{Record, Key, Value};
pub use traits::{FromRecord, FromBytes};
pub use string::Parse;
// pub use bytes::Slice;
pub use self::serde::Json;
