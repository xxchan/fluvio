mod record;
mod traits;
mod string;
mod bytes;
mod serde;

pub use eyre::Error;
pub use record::{Record, Key, Value};
pub use traits::{FromRecord, FromBytes, IntoRecord, IntoBytes};
pub use string::Parse;

#[cfg(feature = "serde_json")]
pub use self::serde::json::Json;

#[cfg(feature = "serde_yaml")]
pub use self::serde::yaml::Yaml;

#[cfg(feature = "toml")]
pub use self::serde::toml::Toml;
