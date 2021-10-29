pub mod bytes;
mod traits;
mod record;
pub mod serde;

pub use eyre::Error;
pub use record::{Record, Key, Value};
pub use traits::{FromRecord, FromBytes};
