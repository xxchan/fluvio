mod traits;
mod record;
pub mod bytes;
pub mod serde;
pub mod string;

pub use eyre::Error;
pub use record::{Record, Key, Value};
pub use traits::{FromRecord, FromBytes};

pub mod prelude {
    pub use super::{FromRecord, FromBytes};
    pub use super::{Record, Key, Value};
    pub use super::serde::Json;
    pub use super::string::Parse;
    pub use super::bytes::Slice;
}
