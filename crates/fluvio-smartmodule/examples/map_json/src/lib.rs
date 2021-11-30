use serde_json::Value as SerdeValue;
use fluvio_smartmodule::{smartmodule, Result};
use fluvio_smartmodule::extract::*;

#[smartmodule(map)]
pub fn map(record: Value<Json<SerdeValue>>) -> Result<Value<Yaml<SerdeValue>>> {
    Ok(Value(Yaml(record.into_inner())))
}
