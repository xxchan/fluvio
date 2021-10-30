use serde_json::Value as JsonValue;
use fluvio_smartmodule::{smartmodule, RecordData, Result};
use fluvio_smartmodule::extract::*;

#[smartmodule(map)]
pub fn map(
    record: Record<&[u8], Value<Json<JsonValue>>>,
) -> Result<(Option<RecordData>, RecordData)> {
    let yaml_bytes = serde_yaml::to_vec(record.value())?;
    Ok((record.key.map(|it| it.into()), yaml_bytes.into()))
}
