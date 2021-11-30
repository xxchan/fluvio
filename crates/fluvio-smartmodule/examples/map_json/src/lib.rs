use serde_json::Value as JsonValue;
use fluvio_smartmodule::{smartmodule, Result};
use fluvio_smartmodule::extract::*;

#[smartmodule(map)]
pub fn map(record: Value<Json<JsonValue>>) -> Result<Value<Vec<u8>>> {
    let yaml_bytes = serde_yaml::to_vec(record.inner())?;
    Ok(Value(yaml_bytes))
}
