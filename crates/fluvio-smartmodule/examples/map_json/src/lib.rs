use serde_json::Value as JsonValue;
use fluvio_smartmodule::{smartmodule, RecordData, Result};
use fluvio_smartmodule::extract::prelude::*;

#[smartmodule(map)]
pub fn map(
    record: Record<Slice, Value<Json<JsonValue>>>,
) -> Result<(Option<RecordData>, RecordData)> {
    let yaml_bytes = serde_yaml::to_vec(&record.value.0)?;
    Ok((record.key.map(|it| it.0.into()), yaml_bytes.into()))
}
