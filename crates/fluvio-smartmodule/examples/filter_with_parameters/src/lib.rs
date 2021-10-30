use fluvio_smartmodule::{smartmodule, SmartOpt, Result};
use fluvio_smartmodule::extract::*;

#[derive(SmartOpt)]
pub struct FilterOpt {
    key: String,
}

impl Default for FilterOpt {
    fn default() -> Self {
        Self {
            key: "a".to_string(),
        }
    }
}

#[smartmodule(filter, params)]
pub fn filter(value: Value<String>, opt: &FilterOpt) -> Result<bool> {
    Ok(value.inner().contains(&opt.key))
}
