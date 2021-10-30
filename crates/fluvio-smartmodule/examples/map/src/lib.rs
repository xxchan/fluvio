use fluvio_smartmodule::{smartmodule, RecordData, Result};
use fluvio_smartmodule::extract::*;

#[smartmodule(map)]
pub fn map(record: Record<Slice, String>) -> Result<(Option<RecordData>, RecordData)> {
    let value = record.value.to_ascii_lowercase();
    Ok((record.key.map(|k| k.into()), value.into()))
}
