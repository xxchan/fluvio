mod record;
mod traits;
mod string;
mod bytes;
mod serde;

pub use eyre::Error;
pub use record::{Record, Key, Value};
pub use traits::{FromRecord, FromBytes, IntoRecord, IntoBytes};
pub use string::Parse;
pub use self::serde::Json;
