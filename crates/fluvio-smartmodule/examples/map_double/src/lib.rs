use fluvio_smartmodule::{smartmodule, Result, RecordData};
use fluvio_smartmodule::extract::*;

#[smartmodule(map)]
pub fn map(record: Record<&[u8], Parse<i32>>) -> Result<(Option<RecordData>, RecordData)> {
    let value = (record.value() * 2).to_string();
    Ok((record.key.map(|k| k.into()), value.into()))
}